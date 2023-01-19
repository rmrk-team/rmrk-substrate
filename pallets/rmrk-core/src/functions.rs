// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-core.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![allow(clippy::too_many_arguments)]

use super::*;
use codec::{Codec, Decode, Encode};
use frame_support::{
	dispatch::DispatchResultWithPostInfo,
	pallet_prelude::*,
	traits::{tokens::Locker, Get},
};

use sp_runtime::{
	traits::{One, Saturating, TrailingZeroInput},
	ArithmeticError,
};

use rmrk_traits::{budget::Budget, misc::TransferHooks};
use sp_std::collections::btree_set::BTreeSet;

// Randomness to generate NFT virtual accounts
pub const SALT_RMRK_NFT: &[u8; 8] = b"RmrkNft/";

impl<T: Config>
	Priority<
		StringLimitOf<T>,
		T::AccountId,
		BoundedVec<ResourceId, T::MaxPriorities>,
		T::CollectionId,
		T::ItemId,
	> for Pallet<T>
{
	fn priority_set(
		_sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		priorities: BoundedVec<ResourceId, T::MaxPriorities>,
	) -> DispatchResultWithPostInfo {
		let _multi_removal_results =
			Priorities::<T>::clear_prefix((collection_id, nft_id), T::MaxPriorities::get(), None);
		let mut priority_index = 0u32;
		for resource_id in priorities {
			Priorities::<T>::insert((collection_id, nft_id, resource_id), priority_index);
			priority_index += 1;
		}
		Self::deposit_event(Event::PrioritySet { collection_id, nft_id });
		Ok(Some(<T as pallet::Config>::WeightInfo::set_priority(
			priority_index,
			T::NestingBudget::get(),
		))
		.into())
	}
}

impl<T: Config> Property<KeyLimitOf<T>, ValueLimitOf<T>, T::AccountId, T::CollectionId, T::ItemId>
	for Pallet<T>
{
	fn property_set(
		sender: T::AccountId,
		collection_id: T::CollectionId,
		maybe_nft_id: Option<T::ItemId>,
		key: KeyLimitOf<T>,
		value: ValueLimitOf<T>,
	) -> DispatchResult {
		let collection =
			Collections::<T>::get(&collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		ensure!(collection.issuer == sender, Error::<T>::NoPermission);
		if let Some(nft_id) = &maybe_nft_id {
			// Check NFT lock status
			ensure!(
				!Pallet::<T>::is_locked(collection_id, *nft_id),
				pallet_uniques::Error::<T>::Locked
			);
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, *nft_id, &budget)?;
			ensure!(root_owner == collection.issuer, Error::<T>::NoPermission);
		}
		Properties::<T>::insert((&collection_id, maybe_nft_id, &key), &value);
		Ok(())
	}

	// Internal function to set a property for downstream `Origin::root()` calls.
	fn do_set_property(
		collection_id: T::CollectionId,
		maybe_nft_id: Option<T::ItemId>,
		key: KeyLimitOf<T>,
		value: ValueLimitOf<T>,
	) -> sp_runtime::DispatchResult {
		// Ensure collection exists
		Collections::<T>::get(&collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		Properties::<T>::insert((&collection_id, maybe_nft_id, &key), &value);

		Self::deposit_event(Event::PropertySet { collection_id, maybe_nft_id, key, value });
		Ok(())
	}

	// Internal function to remove a property for downstream `Origin::root()` calls.
	fn do_remove_property(
		collection_id: T::CollectionId,
		maybe_nft_id: Option<T::ItemId>,
		key: KeyLimitOf<T>,
	) -> sp_runtime::DispatchResult {
		Properties::<T>::remove((&collection_id, maybe_nft_id, &key));

		Self::deposit_event(Event::PropertyRemoved { collection_id, maybe_nft_id, key });
		Ok(())
	}

	// Internal function to remove all of the properties for downstream
	// `Origin::root()` calls.
	fn do_remove_properties(
		collection_id: T::CollectionId,
		maybe_nft_id: Option<T::ItemId>,
		limit: u32,
	) -> sp_runtime::DispatchResult {
		let _ = Properties::<T>::clear_prefix((&collection_id, maybe_nft_id), limit, None);

		Self::deposit_event(Event::PropertiesRemoved { collection_id, maybe_nft_id });
		Ok(())
	}
}

impl<T: Config>
	Resource<
		BoundedVec<u8, T::StringLimit>,
		T::AccountId,
		BoundedVec<PartId, T::PartsLimit>,
		T::CollectionId,
		T::ItemId,
	> for Pallet<T>
{
	fn resource_add(
		_sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		resource: ResourceTypes<BoundedVec<u8, T::StringLimit>, BoundedVec<PartId, T::PartsLimit>>,
		pending: bool,
		resource_id: ResourceId,
	) -> Result<ResourceId, DispatchError> {
		ensure!(
			Resources::<T>::get((collection_id, nft_id, resource_id)).is_none(),
			Error::<T>::ResourceAlreadyExists
		);

		match resource.clone() {
			ResourceTypes::Basic(_r) => (),
			ResourceTypes::Composable(r) => {
				EquippableBases::<T>::insert((collection_id, nft_id, r.base), ());
				if let Some((base, slot)) = r.slot {
					EquippableSlots::<T>::insert(
						(collection_id, nft_id, resource_id, base, slot),
						(),
					);
				}
			},
			ResourceTypes::Slot(r) => {
				EquippableSlots::<T>::insert(
					(collection_id, nft_id, resource_id, r.base, r.slot),
					(),
				);
			},
		}

		let res: ResourceInfo<BoundedVec<u8, T::StringLimit>, BoundedVec<PartId, T::PartsLimit>> =
			ResourceInfo::<BoundedVec<u8, T::StringLimit>, BoundedVec<PartId, T::PartsLimit>> {
				id: resource_id,
				pending,
				pending_removal: false,
				resource,
			};
		Resources::<T>::insert((collection_id, nft_id, resource_id), res);

		Self::deposit_event(Event::ResourceAdded { nft_id, resource_id, collection_id });

		Ok(resource_id)
	}

	fn accept(
		_sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		resource_id: ResourceId,
	) -> DispatchResult {
		ensure!(
			Resources::<T>::get((collection_id, nft_id, resource_id)).is_some(),
			Error::<T>::ResourceDoesntExist
		);
		Resources::<T>::try_mutate_exists(
			(collection_id, nft_id, resource_id),
			|resource| -> DispatchResult {
				if let Some(res) = resource.into_mut() {
					ensure!(res.pending, Error::<T>::ResourceNotPending);
					res.pending = false;
				}
				Ok(())
			},
		)?;

		Self::deposit_event(Event::ResourceAccepted { nft_id, resource_id, collection_id });

		Ok(())
	}

	fn resource_remove(
		_sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		resource_id: ResourceId,
		pending_resource: bool,
	) -> DispatchResult {
		ensure!(
			Resources::<T>::contains_key((collection_id, nft_id, resource_id)),
			Error::<T>::ResourceDoesntExist
		);
		if pending_resource {
			Resources::<T>::try_mutate_exists(
				(collection_id, nft_id, resource_id),
				|resource| -> DispatchResult {
					if let Some(res) = resource {
						res.pending_removal = true;
					}
					Ok(())
				},
			)?;
		} else {
			match Resources::<T>::get((collection_id, nft_id, &resource_id)) {
				None => (),
				Some(res) => {
					Self::do_remove_resource_from_bases_and_equippable_slots(
						res,
						collection_id,
						nft_id,
						resource_id,
					);
				},
			}
		}

		Self::deposit_event(Event::ResourceRemoval { nft_id, resource_id, collection_id });

		Ok(())
	}

	fn resource_replace(
		_sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		resource: ResourceTypes<BoundedVec<u8, T::StringLimit>, BoundedVec<PartId, T::PartsLimit>>,
		resource_id: ResourceId,
	) -> DispatchResult {
		ensure!(
			Resources::<T>::get((collection_id, nft_id, resource_id)).is_some(),
			Error::<T>::ResourceDoesntExist
		);

		Resources::<T>::try_mutate(
			(collection_id, nft_id, resource_id),
			|current_resource| -> DispatchResult {
				if let Some(res) = current_resource.into_mut() {
					res.resource = resource;
				}
				Ok(())
			},
		)?;

		Self::deposit_event(Event::ResourceReplaced { nft_id, resource_id, collection_id });

		Ok(())
	}

	fn accept_removal(
		_sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		resource_id: ResourceId,
	) -> DispatchResult {
		ensure!(
			Resources::<T>::contains_key((collection_id, nft_id, &resource_id)),
			Error::<T>::ResourceDoesntExist
		);
		match Resources::<T>::get((collection_id, nft_id, &resource_id)) {
			None => (),
			Some(res) => {
				ensure!(res.pending_removal, Error::<T>::ResourceNotPending);
				Self::do_remove_resource_from_bases_and_equippable_slots(
					res,
					collection_id,
					nft_id,
					resource_id,
				);
			},
		}

		Self::deposit_event(Event::ResourceRemovalAccepted { nft_id, resource_id, collection_id });

		Ok(())
	}
}

impl<T: Config>
	Collection<StringLimitOf<T>, BoundedCollectionSymbolOf<T>, T::AccountId, T::CollectionId>
	for Pallet<T>
{
	fn issuer(_collection_id: T::CollectionId) -> Option<T::AccountId> {
		None
	}
	fn collection_create(
		issuer: T::AccountId,
		collection_id: T::CollectionId,
		metadata: StringLimitOf<T>,
		max: Option<u32>,
		symbol: BoundedCollectionSymbolOf<T>,
	) -> Result<(), DispatchError> {
		let collection =
			CollectionInfo { issuer: issuer.clone(), metadata, max, symbol, nfts_count: 0 };

		// Call the pallet_uniques function to create collection
		pallet_uniques::Pallet::<T>::do_create_collection(
			collection_id,
			issuer.clone(),
			issuer.clone(),
			T::CollectionDeposit::get(),
			false,
			pallet_uniques::Event::Created {
				collection: collection_id,
				creator: issuer.clone(),
				owner: issuer.clone(),
			},
		)?;
		Collections::<T>::insert(collection_id, collection);
		Self::deposit_event(Event::CollectionCreated { issuer, collection_id });
		Ok(())
	}

	fn collection_burn(issuer: T::AccountId, collection_id: T::CollectionId) -> DispatchResult {
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		ensure!(collection.nfts_count == 0, Error::<T>::CollectionNotEmpty);
		// Get DestroyWitness for Uniques pallet
		let witness = pallet_uniques::Pallet::<T>::get_destroy_witness(&collection_id)
			.ok_or(Error::<T>::NoWitness)?;
		ensure!(witness.items == 0u32, Error::<T>::CollectionNotEmpty);
		// Remove from RMRK storage
		Collections::<T>::remove(collection_id);

		pallet_uniques::Pallet::<T>::do_destroy_collection(
			collection_id,
			witness,
			issuer.clone().into(),
		)?;

		Self::deposit_event(Event::CollectionDestroyed { issuer, collection_id });
		Ok(())
	}

	fn collection_change_issuer(
		collection_id: T::CollectionId,
		new_issuer: T::AccountId,
	) -> Result<(T::AccountId, T::CollectionId), DispatchError> {
		ensure!(Collections::<T>::contains_key(collection_id), Error::<T>::NoAvailableCollectionId);

		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			if let Some(col) = collection {
				col.issuer = new_issuer.clone();
			}
			Ok(())
		})?;

		Ok((new_issuer, collection_id))
	}

	fn collection_lock(
		sender: T::AccountId,
		collection_id: T::CollectionId,
	) -> Result<T::CollectionId, DispatchError> {
		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.max = Some(0);
			Ok(())
		})?;

		Self::deposit_event(Event::CollectionLocked { issuer: sender, collection_id });

		Ok(collection_id)
	}
}

impl<T: Config>
	Nft<T::AccountId, StringLimitOf<T>, BoundedResourceInfoTypeOf<T>, T::CollectionId, T::ItemId>
	for Pallet<T>
{
	fn nft_mint(
		sender: T::AccountId,
		owner: T::AccountId,
		nft_id: T::ItemId,
		collection_id: T::CollectionId,
		royalty_recipient: Option<T::AccountId>,
		royalty_amount: Option<Permill>,
		metadata: StringLimitOf<T>,
		transferable: bool,
		resources: Option<BoundedResourceInfoTypeOf<T>>,
	) -> sp_std::result::Result<(T::CollectionId, T::ItemId), DispatchError> {
		ensure!(!Self::nft_exists((collection_id, nft_id)), Error::<T>::NftAlreadyExists);
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;

		// Prevent minting when nfts_count is greater than the collection max.
		if let Some(max) = collection.max {
			ensure!(collection.nfts_count < max, Error::<T>::CollectionFullOrLocked);
		}

		// NFT should be pending if minting to another account
		let pending = owner != sender;

		let mut royalty = None;

		if let Some(amount) = royalty_amount {
			match royalty_recipient {
				Some(recipient) => {
					royalty = Some(RoyaltyInfo { recipient, amount });
				},
				None => {
					// If a royalty amount is passed but no recipient, defaults to the sender
					royalty = Some(RoyaltyInfo { recipient: owner.clone(), amount });
				},
			}
		};

		let nft = NftInfo {
			owner: AccountIdOrCollectionNftTuple::AccountId(owner.clone()),
			royalty,
			metadata,
			equipped: None,
			pending,
			transferable,
		};

		Nfts::<T>::insert(collection_id, nft_id, nft);

		// increment nfts counter
		let nfts_count = collection.nfts_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count = nfts_count;
			Ok(())
		})?;

		// Call do_mint for pallet_uniques
		pallet_uniques::Pallet::<T>::do_mint(collection_id, nft_id, owner.clone(), |_details| {
			Ok(())
		})?;

		// Add all at-mint resources
		if let Some(resources) = resources {
			for res in resources {
				Self::resource_add(
					sender.clone(),
					collection_id,
					nft_id,
					res.resource,
					false,
					res.id,
				)?;
			}
		}

		Self::deposit_event(Event::NftMinted {
			owner: AccountIdOrCollectionNftTuple::AccountId(owner),
			collection_id,
			nft_id,
		});

		Ok((collection_id, nft_id))
	}

	fn nft_mint_directly_to_nft(
		sender: T::AccountId,
		owner: (T::CollectionId, T::ItemId),
		nft_id: T::ItemId,
		collection_id: T::CollectionId,
		royalty_recipient: Option<T::AccountId>,
		royalty_amount: Option<Permill>,
		metadata: StringLimitOf<T>,
		transferable: bool,
		resources: Option<BoundedResourceInfoTypeOf<T>>,
	) -> sp_std::result::Result<(T::CollectionId, T::ItemId), DispatchError> {
		ensure!(!Self::nft_exists((collection_id, nft_id)), Error::<T>::NftAlreadyExists);
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;

		// Prevent minting when nfts_count is greater than the collection max.
		if let Some(max) = collection.max {
			ensure!(collection.nfts_count < max, Error::<T>::CollectionFullOrLocked);
		}

		// Calculate the rootowner of the intended owner of the minted NFT
		let budget = budget::Value::new(T::NestingBudget::get().saturating_sub(One::one()));
		let (rootowner, _) = Self::lookup_root_owner(owner.0, owner.1, &budget)?;

		// NFT should be pending if minting either to an NFT owned by another account
		let pending = rootowner != sender;

		let mut royalty = None;

		if let Some(amount) = royalty_amount {
			match royalty_recipient {
				Some(recipient) => {
					royalty = Some(RoyaltyInfo { recipient, amount });
				},
				None => {
					royalty = Some(RoyaltyInfo { recipient: rootowner, amount });
				},
			}
		};

		let nft = NftInfo {
			owner: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(owner.0, owner.1),
			royalty,
			metadata,
			equipped: None,
			pending,
			transferable,
		};

		Nfts::<T>::insert(collection_id, nft_id, nft);

		// For Uniques, we need to decode the "virtual account" ID to be the owner
		let uniques_owner = Self::nft_to_account_id(owner.0, owner.1);

		// increment nfts counter
		let nfts_count = collection.nfts_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count = nfts_count;
			Ok(())
		})?;

		Pallet::<T>::add_child((owner.0, owner.1), (collection_id, nft_id));

		pallet_uniques::Pallet::<T>::do_mint(collection_id, nft_id, uniques_owner, |_details| {
			Ok(())
		})?;

		// Add all at-mint resources
		if let Some(resources) = resources {
			for res in resources {
				Self::resource_add(
					sender.clone(),
					collection_id,
					nft_id,
					res.resource,
					false,
					res.id,
				)?;
			}
		}

		Self::deposit_event(Event::NftMinted {
			owner: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(owner.0, owner.1),
			collection_id,
			nft_id,
		});

		Ok((collection_id, nft_id))
	}

	fn nft_burn(
		owner: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		budget: &dyn Budget,
	) -> DispatchResultWithPostInfo {
		// Remove self from parent's Children storage
		if let Some(nft) = Self::nfts(collection_id, nft_id) {
			if let AccountIdOrCollectionNftTuple::CollectionAndNftTuple(parent_col, parent_nft) =
				nft.owner
			{
				Children::<T>::remove((parent_col, parent_nft), (collection_id, nft_id));
			}
		}

		Nfts::<T>::remove(collection_id, nft_id);

		// Remove all of the properties of the NFT
		Self::do_remove_properties(collection_id, Some(nft_id), T::PropertiesLimit::get())?;
		// Remove the lock from the NFT if it was locked
		Lock::<T>::remove((&collection_id, nft_id));

		let _multi_removal_results = Resources::<T>::clear_prefix(
			(collection_id, nft_id),
			T::MaxResourcesOnMint::get(),
			None,
		);

		for ((child_collection_id, child_nft_id), _) in
			Children::<T>::drain_prefix((collection_id, nft_id))
		{
			ensure!(budget.consume() != false, Error::<T>::TooManyRecursions);
			Self::nft_burn(owner.clone(), child_collection_id, child_nft_id, budget)?;
		}

		// decrement nfts counter
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count.saturating_dec();
			Ok(())
		})?;

		// Call pallet uniques to ensure NFT is burned
		pallet_uniques::Pallet::<T>::do_burn(collection_id, nft_id, |_, _| Ok(()))?;

		Self::deposit_event(Event::NFTBurned { owner, nft_id, collection_id });

		Ok(Some(<T as pallet::Config>::WeightInfo::burn_nft(
			budget.get_budget_consumed_value(),
			T::PropertiesLimit::get(),
		))
		.into())
	}

	fn nft_send(
		sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		new_owner: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
	) -> Result<(T::AccountId, bool), DispatchError> {
		// Get current owner for child removal later
		let parent = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id);
		// Check if parent returns None which indicates the NFT is not available
		ensure!(parent.is_some(), Error::<T>::NoAvailableNftId); // <- is this error wrong?

		let budget = budget::Value::new(T::NestingBudget::get());
		let (root_owner, _root_nft) =
			Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
		// Check ownership
		ensure!(sender == root_owner, Error::<T>::NoPermission);
		// Get NFT info
		let mut sending_nft =
			Nfts::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		// Defaults to true, but can be implemented downstream for custom logic
		ensure!(
			T::TransferHooks::pre_check(&sender, &collection_id, &nft_id),
			Error::<T>::FailedTransferHooksPreCheck
		);

		// Check NFT is transferable
		Self::check_is_transferable(&sending_nft)?;

		// NFT cannot be sent if it is equipped
		Self::check_is_not_equipped(&sending_nft)?;

		// Needs to be pending if the sending to an account or to a non-owned NFT
		let mut approval_required = true;

		// Prepare transfer
		let new_owner_account = match new_owner.clone() {
			AccountIdOrCollectionNftTuple::AccountId(id) => {
				approval_required = false;
				id
			},
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) => {
				// Check if NFT target exists
				ensure!(Nfts::<T>::contains_key(cid, nid), Error::<T>::NoAvailableNftId);
				// Check if sending to self
				ensure!(
					(collection_id, nft_id) != (cid, nid),
					Error::<T>::CannotSendToDescendentOrSelf
				);
				// Check if collection_id & nft_id are descendent of cid & nid
				ensure!(
					!Pallet::<T>::is_x_descendent_of_y(cid, nid, collection_id, nft_id),
					Error::<T>::CannotSendToDescendentOrSelf
				);
				let budget = budget::Value::new(T::NestingBudget::get().saturating_sub(One::one()));
				let (recipient_root_owner, _root_nft) =
					Pallet::<T>::lookup_root_owner(cid, nid, &budget)?;
				if recipient_root_owner == root_owner {
					approval_required = false;
				}

				// Convert to virtual account
				Pallet::<T>::nft_to_account_id::<T::AccountId>(cid, nid)
			},
		};

		sending_nft.owner = new_owner.clone();

		if approval_required {
			Nfts::<T>::try_mutate_exists(collection_id, nft_id, |nft| -> DispatchResult {
				if let Some(nft) = nft {
					nft.pending = true;
					nft.owner = new_owner.clone();
				}
				Ok(())
			})?;
		} else {
			Nfts::<T>::insert(collection_id, nft_id, sending_nft);
		}

		if let Some(current_owner) = parent {
			// Handle Children StorageMap for NFTs
			let current_owner_cid_nid =
				Pallet::<T>::decode_nft_account_id::<T::AccountId>(current_owner);
			if let Some(current_owner_cid_nid) = current_owner_cid_nid {
				// Remove child from parent
				Pallet::<T>::remove_child(current_owner_cid_nid, (collection_id, nft_id));
			}
		}

		// add child to new parent if NFT virtual address
		let new_owner_cid_nid =
			Pallet::<T>::decode_nft_account_id::<T::AccountId>(new_owner_account.clone());
		if let Some(new_owner_cid_nid) = new_owner_cid_nid {
			Pallet::<T>::add_child(new_owner_cid_nid, (collection_id, nft_id));
		}

		pallet_uniques::Pallet::<T>::do_transfer(
			collection_id,
			nft_id,
			new_owner_account.clone(),
			|_class_details, _details| Ok(()),
		)?;

		// Defaults to true, but can be implemented downstream for custom logic
		ensure!(
			T::TransferHooks::post_transfer(&sender, &new_owner_account, &collection_id, &nft_id),
			Error::<T>::FailedTransferHooksPostTransfer
		);

		Self::deposit_event(Event::NFTSent {
			sender,
			recipient: new_owner,
			collection_id,
			nft_id,
			approval_required,
		});

		Ok((new_owner_account, approval_required))
	}

	fn nft_accept(
		sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		new_owner: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
	) -> Result<(T::AccountId, T::CollectionId, T::ItemId), DispatchError> {
		// Check NFT exists
		ensure!(Pallet::<T>::nft_exists((collection_id, nft_id)), Error::<T>::NoAvailableNftId);

		let owner_account = match new_owner.clone() {
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) =>
				Pallet::<T>::nft_to_account_id(cid, nid),
			AccountIdOrCollectionNftTuple::AccountId(owner_account) => owner_account,
		};

		Nfts::<T>::try_mutate(collection_id, nft_id, |nft| -> DispatchResult {
			if let Some(nft) = nft {
				nft.pending = false;
			}
			Ok(())
		})?;

		Self::deposit_event(Event::NFTAccepted {
			sender,
			recipient: new_owner,
			collection_id,
			nft_id,
		});

		Ok((owner_account, collection_id, nft_id))
	}

	fn nft_reject(
		sender: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
	) -> DispatchResultWithPostInfo {
		// Look up root owner in Uniques to ensure permissions
		let budget = budget::Value::new(T::NestingBudget::get());
		let (root_owner, _root_nft) =
			Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;

		let nft = Nfts::<T>::get(collection_id, nft_id);

		// Ensure NFT is pending (cannot reject non-pending NFT) and exists in Nfts storage
		match nft {
			None => return Err(Error::<T>::NoAvailableNftId.into()),
			Some(nft) => ensure!(nft.pending, Error::<T>::CannotRejectNonPendingNft),
		}

		// Check ownership
		ensure!(sender == root_owner, Error::<T>::CannotRejectNonOwnedNft);

		// Get current owner, which we will use to remove the Children storage
		if let Some(parent_account_id) = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id) {
			// Decode the parent_account_id to extract the parent (T::CollectionId, T::ItemId)
			if let Some(parent) =
				Pallet::<T>::decode_nft_account_id::<T::AccountId>(parent_account_id)
			{
				// Remove the parent-child Children storage
				Self::remove_child(parent, (collection_id, nft_id));
			}
		}

		// Get NFT info
		let _rejecting_nft =
			Nfts::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		let result = Self::nft_burn(sender.clone(), collection_id, nft_id, &budget);

		Self::deposit_event(Event::NFTRejected { sender: sender.clone(), collection_id, nft_id });

		result
	}
}

impl<T: Config> Locker<T::CollectionId, T::ItemId> for Pallet<T> {
	fn is_locked(collection_id: T::CollectionId, nft_id: T::ItemId) -> bool {
		Lock::<T>::get((collection_id, nft_id))
	}
}

impl<T: Config> Pallet<T> {
	pub fn iterate_nft_children(
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
	) -> impl Iterator<Item = NftChild<T::CollectionId, T::ItemId>> {
		Children::<T>::iter_key_prefix((collection_id, nft_id)).into_iter().map(
			|(collection_id, nft_id)| NftChild::<T::CollectionId, T::ItemId> {
				collection_id,
				nft_id,
			},
		)
	}

	pub fn iterate_resources(
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
	) -> impl Iterator<Item = ResourceInfoOf<T>> {
		Resources::<T>::iter_prefix_values((collection_id, nft_id))
	}

	pub fn query_properties(
		collection_id: T::CollectionId,
		nft_id: Option<T::ItemId>,
		filter_keys: Option<BTreeSet<BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>>>,
	) -> impl Iterator<Item = PropertyInfoOf<T>> {
		Properties::<T>::iter_prefix((collection_id, nft_id))
			.filter(move |(key, _)| match &filter_keys {
				Some(filter_keys) => filter_keys.contains(key),
				None => true,
			})
			.map(|(key, value)| PropertyInfoOf::<T> { key, value })
	}

	/// Encodes a RMRK NFT with randomness + `collection_id` + `nft_id` into a virtual account
	/// then returning the `AccountId`. Note that we must be careful of the size of `AccountId`
	/// as it must be wide enough to keep the size of the prefix as well as the `collection_id`
	/// and `nft_id`.
	///
	/// Parameters:
	/// - `collection_id`: Collection ID that the NFT is contained in
	/// - `nft_id`: NFT ID to be encoded into a virtual account
	///
	/// Output:
	/// `AccountId`: Encoded virtual account that represents the NFT
	pub fn nft_to_account_id<AccountId: Codec>(
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
	) -> AccountId {
		(SALT_RMRK_NFT, collection_id, nft_id)
			.using_encoded(|b| AccountId::decode(&mut TrailingZeroInput::new(b)))
			.expect("Decoding with trailing zero never fails; qed.")
	}

	/// Decodes a RMRK NFT a suspected virtual account
	/// then returns an `Option<(T::CollectionId,  T::ItemId)>
	/// where `None` is returned when there is an actual account
	/// and `Some(tuple)` returns tuple of `CollectionId` & ` T::ItemId`
	///
	/// Parameters:
	/// - `account_id`: Encoded NFT virtual account or account owner
	///
	/// Output:
	/// `Option<(T::CollectionId,  T::ItemId)>`
	pub fn decode_nft_account_id<AccountId: Codec>(
		account_id: T::AccountId,
	) -> Option<(T::CollectionId, T::ItemId)> {
		let (prefix, tuple, suffix) = account_id
			.using_encoded(|mut b| {
				let slice = &mut b;
				let r = <([u8; 8], (T::CollectionId, T::ItemId))>::decode(slice);
				r.map(|(prefix, tuple)| (prefix, tuple, slice.to_vec()))
			})
			.ok()?;
		// Check prefix and suffix to avoid collision attack
		if &prefix == SALT_RMRK_NFT && suffix.iter().all(|&x| x == 0) {
			Some(tuple)
		} else {
			None
		}
	}

	/// Looks up the root owner of an NFT and returns a `Result` with an AccountId and
	/// a tuple of the root `(T::CollectionId,  T::ItemId)`
	/// or an `Error::<T>::NoAvailableNftId` in the case that the NFT is already burned
	///
	/// Parameters:
	/// - `collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `nft_id`: NFT ID that is to be looked up for the root owner
	///
	/// Output:
	/// - `Result<(T::AcccountId, (T::CollectionId,  T::ItemId)), Error<T>>`
	// #[allow(clippy::type_complexity)]
	pub fn lookup_root_owner(
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		budget: &dyn Budget,
	) -> Result<(T::AccountId, (T::CollectionId, T::ItemId)), DispatchError> {
		// Check if parent returns None which indicates the NFT is not available
		if let Some(parent) = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id) {
			match Self::decode_nft_account_id::<T::AccountId>(parent.clone()) {
				None => Ok((parent, (collection_id, nft_id))),
				Some((cid, nid)) => {
					ensure!(budget.consume() != false, Error::<T>::TooManyRecursions);
					Pallet::<T>::lookup_root_owner(cid, nid, budget)
				},
			}
		} else {
			Err(Error::<T>::NoAvailableNftId.into())
		}
	}

	/// Add a child to a parent NFT
	///
	/// Parameters:
	/// - `parent`: Tuple of (T::CollectionId,  T::ItemId) of the parent NFT
	/// - `child`: Tuple of (T::CollectionId,  T::ItemId) of the child NFT to be added
	///
	/// Output:
	/// - Adding a `child` to the Children StorageMap of the `parent`
	pub fn add_child(parent: (T::CollectionId, T::ItemId), child: (T::CollectionId, T::ItemId)) {
		Children::<T>::insert((parent.0, parent.1), (child.0, child.1), ());
	}

	/// Remove a child from a parent NFT
	///
	/// Parameters:
	/// - `parent`: Tuple of (T::CollectionId,  T::ItemId) of the parent NFT
	/// - `child`: Tuple of (T::CollectionId,  T::ItemId) of the child NFT to be removed
	///
	/// Output:
	/// - Removing a `child` from the Children StorageMap of the `parent`
	pub fn remove_child(parent: (T::CollectionId, T::ItemId), child: (T::CollectionId, T::ItemId)) {
		Children::<T>::remove((parent.0, parent.1), (child.0, child.1));
	}

	/// Check whether a NFT is descends from a suspected parent NFT
	/// and return a `bool` if NFT is or not
	///
	/// Parameters:
	/// - `child_collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `child_nft_id`: NFT ID that is to be looked up for the root owner
	/// - `parent_collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `parent_nft_id`: NFT ID that is to be looked up for the root owner
	/// Output:
	/// - `bool`
	pub fn is_x_descendent_of_y(
		child_collection_id: T::CollectionId,
		child_nft_id: T::ItemId,
		parent_collection_id: T::CollectionId,
		parent_nft_id: T::ItemId,
	) -> bool {
		let mut found_child = false;

		// Check if parent returns None which indicates the NFT is not available
		let parent = match pallet_uniques::Pallet::<T>::owner(child_collection_id, child_nft_id) {
			Some(parent) => parent,
			None => return found_child,
		};

		match Self::decode_nft_account_id::<T::AccountId>(parent) {
			None => found_child,
			Some((cid, nid)) => {
				if (cid, nid) == (parent_collection_id, parent_nft_id) {
					found_child = true
				} else {
					found_child = Pallet::<T>::is_x_descendent_of_y(
						cid,
						nid,
						parent_collection_id,
						parent_nft_id,
					)
				}
				found_child
			},
		}
	}

	pub fn set_lock(nft: (T::CollectionId, T::ItemId), lock_status: bool) -> bool {
		if lock_status {
			Lock::<T>::mutate(nft, |lock| {
				*lock = lock_status;
				*lock
			});
		} else {
			Lock::<T>::remove(nft);
		}
		lock_status
	}

	// Check NFT is transferable
	pub fn check_is_transferable(nft: &InstanceInfoOf<T>) -> DispatchResult {
		ensure!(nft.transferable, Error::<T>::NonTransferable);
		Ok(())
	}

	/// Helper function for checking if an NFT exists
	pub fn nft_exists(item: (T::CollectionId, T::ItemId)) -> bool {
		let (item_collection_id, item_nft_id) = item;
		Nfts::<T>::get(item_collection_id, item_nft_id).is_some()
	}

	// Check NFT is not equipped
	pub fn check_is_not_equipped(nft: &InstanceInfoOf<T>) -> DispatchResult {
		ensure!(nft.equipped.is_none(), Error::<T>::CannotSendEquippedItem);
		Ok(())
	}

	pub fn do_remove_resource_from_bases_and_equippable_slots(
		res: ResourceInfoOf<T>,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		resource_id: ResourceId,
	) -> () {
		match res.resource {
			ResourceTypes::Basic(_r) => {
				Resources::<T>::remove((collection_id, nft_id, resource_id));
			},
			ResourceTypes::Slot(r) => {
				EquippableSlots::<T>::remove((collection_id, nft_id, resource_id, r.base, r.slot));
				Resources::<T>::remove((collection_id, nft_id, resource_id));
			},
			ResourceTypes::Composable(r) => {
				EquippableBases::<T>::remove((collection_id, nft_id, r.base));
				if let Some((base, slot)) = r.slot {
					EquippableSlots::<T>::remove((collection_id, nft_id, resource_id, base, slot));
				};
				Resources::<T>::remove((collection_id, nft_id, resource_id));
			},
		}
	}
}
