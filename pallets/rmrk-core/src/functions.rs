#![allow(clippy::too_many_arguments)]

use super::*;
use codec::{Codec, Decode, Encode};
use sp_runtime::{
	traits::{Saturating, TrailingZeroInput},
	ArithmeticError,
};

// Randomness to generate NFT virtual accounts
pub const SALT_RMRK_NFT: &[u8; 8] = b"RmrkNft/";

impl<T: Config> Priority<StringLimitOf<T>, T::AccountId> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	fn priority_set(
		_sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		priorities: Vec<Vec<u8>>,
	) -> DispatchResult {
		// TODO : Check NFT lock status
		let mut bounded_priorities = Vec::<BoundedVec<u8, T::StringLimit>>::new();
		for priority in priorities {
			let bounded_priority = Self::to_bounded_string(priority)?;
			bounded_priorities.push(bounded_priority);
		}
		Priorities::<T>::insert(collection_id, nft_id, bounded_priorities);
		Self::deposit_event(Event::PrioritySet { collection_id, nft_id });
		Ok(())
	}
}

impl<T: Config> Property<KeyLimitOf<T>, ValueLimitOf<T>, T::AccountId> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	fn property_set(
		sender: T::AccountId,
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimitOf<T>,
		value: ValueLimitOf<T>,
	) -> DispatchResult {
		let collection =
			Collections::<T>::get(&collection_id).ok_or(Error::<T>::NoAvailableCollectionId)?;
		ensure!(collection.issuer == sender, Error::<T>::NoPermission);
		if let Some(nft_id) = &maybe_nft_id {
			// TODO: Check NFT lock status
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, *nft_id)?;
			ensure!(root_owner == collection.issuer, Error::<T>::NoPermission);
		}
		Properties::<T>::insert((&collection_id, maybe_nft_id, &key), &value);
		Self::deposit_event(Event::PropertySet { collection_id, maybe_nft_id, key, value });
		Ok(())
	}
}

impl<T: Config> Resource<BoundedVec<u8, T::StringLimit>, T::AccountId, BoundedResource<T::ResourceSymbolLimit>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	fn resource_add(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: BoundedResource<T::ResourceSymbolLimit>,
		base: Option<BaseId>,
		src: Option<BoundedVec<u8, T::StringLimit>>,
		metadata: Option<BoundedVec<u8, T::StringLimit>>,
		slot: Option<SlotId>,
		license: Option<BoundedVec<u8, T::StringLimit>>,
		thumb: Option<BoundedVec<u8, T::StringLimit>>,
		parts: Option<Vec<PartId>>,
	) -> DispatchResult {
		let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;

		let empty =
			base.is_none() &&
				src.is_none() && metadata.is_none() &&
				slot.is_none() && license.is_none() &&
				thumb.is_none();
		ensure!(!empty, Error::<T>::EmptyResource);

		let res = ResourceInfo::<BoundedVec<u8, T::ResourceSymbolLimit>, BoundedVec<u8, T::StringLimit>> {
			id: resource_id.clone(),
			base,
			src,
			metadata,
			slot,
			license,
			thumb,
			parts,
			pending: root_owner != sender,
		};
		Resources::<T>::insert((collection_id, nft_id, resource_id), res);

		Ok(())
	}

	fn accept(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: BoundedResource<T::ResourceSymbolLimit>,
	) -> DispatchResult {
		let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;
		ensure!(root_owner == sender, Error::<T>::NoPermission);
		// TODO: Check NFT lock status

		Resources::<T>::try_mutate_exists(
			(collection_id, nft_id, resource_id.clone()),
			|resource| -> DispatchResult {
				if let Some(res) = resource {
					res.pending = false;
				}
				Ok(())
			},
		)?;

		Self::deposit_event(Event::ResourceAccepted { nft_id, resource_id });
		Ok(())
	}
}

impl<T: Config> Collection<StringLimitOf<T>, T::AccountId> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	fn issuer(_collection_id: CollectionId) -> Option<T::AccountId> {
		None
	}
	fn collection_create(
		issuer: T::AccountId,
		metadata: StringLimitOf<T>,
		max: u32,
		symbol: StringLimitOf<T>,
	) -> Result<CollectionId, DispatchError> {
		let collection = CollectionInfo { issuer, metadata, max, symbol, nfts_count: 0 };
		let collection_id =
			<CollectionIndex<T>>::try_mutate(|n| -> Result<CollectionId, DispatchError> {
				let id = *n;
				ensure!(id != CollectionId::max_value(), Error::<T>::NoAvailableCollectionId);
				*n += 1;
				Ok(id)
			})?;
		Collections::<T>::insert(collection_id, collection);
		Ok(collection_id)
	}

	fn collection_burn(_issuer: T::AccountId, collection_id: CollectionId) -> DispatchResult {
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		ensure!(collection.nfts_count == 0, Error::<T>::CollectionNotEmpty);
		Collections::<T>::remove(collection_id);
		Ok(())
	}

	fn collection_change_issuer(
		collection_id: CollectionId,
		new_issuer: T::AccountId,
	) -> Result<(T::AccountId, CollectionId), DispatchError> {
		ensure!(Collections::<T>::contains_key(collection_id), Error::<T>::NoAvailableCollectionId);

		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			if let Some(col) = collection {
				col.issuer = new_issuer.clone();
			}
			Ok(())
		})?;

		Ok((new_issuer, collection_id))
	}

	fn collection_lock(collection_id: CollectionId) -> Result<CollectionId, DispatchError> {
		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.max = collection.nfts_count;
			Ok(())
		})?;
		Ok(collection_id)
	}
}

impl<T: Config> Nft<T::AccountId, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	type MaxRecursions = T::MaxRecursions;

	fn nft_mint(
		_sender: T::AccountId,
		owner: T::AccountId,
		collection_id: CollectionId,
		recipient: Option<T::AccountId>,
		royalty: Option<Permill>,
		metadata: StringLimitOf<T>,
	) -> sp_std::result::Result<(CollectionId, NftId), DispatchError> {
		let nft_id = Self::get_next_nft_id(collection_id)?;
		let collection = Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
		let max: u32 = collection.max;

		// Prevent minting when next NFT id is greater than the collection max.
		ensure!(nft_id < max, Error::<T>::CollectionFullOrLocked);

		let recipient = recipient.unwrap_or_else(|| owner.clone());
		let royalty = royalty.unwrap_or_default();

		let owner_as_maybe_account = AccountIdOrCollectionNftTuple::AccountId(owner.clone());

		let nft = NftInfo { owner: owner_as_maybe_account, recipient, royalty, metadata, equipped: false };

		Nfts::<T>::insert(collection_id, nft_id, nft);
		NftsByOwner::<T>::append(owner, (collection_id, nft_id));

		// increment nfts counter
		let nfts_count = collection.nfts_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count = nfts_count;
			Ok(())
		})?;

		Ok((collection_id, nft_id))
	}

	fn nft_burn(
		collection_id: CollectionId,
		nft_id: NftId,
		max_recursions: u32,
	) -> sp_std::result::Result<(CollectionId, NftId), DispatchError> {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		Nfts::<T>::remove(collection_id, nft_id);
		let kids = Children::<T>::take((collection_id, nft_id));
		for (child_collection_id, child_nft_id) in kids {
			// Remove child from Children StorageMap
			Pallet::<T>::remove_child((collection_id, nft_id), (child_collection_id, child_nft_id));
			Self::nft_burn(child_collection_id, child_nft_id, max_recursions - 1)?;
		}

		// decrement nfts counter
		Collections::<T>::try_mutate(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			collection.nfts_count.saturating_dec();
			Ok(())
		})?;

		Ok((collection_id, nft_id))
	}

	fn nft_send(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple<T::AccountId>,
	) -> sp_std::result::Result<(T::AccountId, bool), DispatchError> {
		// Get current owner for child removal later
		let parent = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id);
		// Check if parent returns None which indicates the NFT is not available
		ensure!(parent.is_some(), Error::<T>::NoAvailableNftId); // <- is this error wrong?

		let (root_owner, _root_nft) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;
		// Check ownership
		ensure!(sender == root_owner, Error::<T>::NoPermission);
		// Get NFT info
		let mut sending_nft =
			Nfts::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		// TODO: Check NFT lock status
		
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
				let (recipient_root_owner, _root_nft) = Pallet::<T>::lookup_root_owner(cid, nid)?;
				if recipient_root_owner == root_owner {
					approval_required = false;
				}

				// Convert to virtual account
				Pallet::<T>::nft_to_account_id::<T::AccountId>(cid, nid)
			},
		};

		sending_nft.owner = new_owner;
		// Nfts::<T>::insert(collection_id, nft_id, sending_nft);

		if approval_required {
			PendingNfts::<T>::insert(collection_id, nft_id, sending_nft);
			Nfts::<T>::remove(collection_id, nft_id);
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
		if !approval_required {
			let new_owner_cid_nid =
				Pallet::<T>::decode_nft_account_id::<T::AccountId>(new_owner_account.clone());
			if let Some(new_owner_cid_nid) = new_owner_cid_nid {
				Pallet::<T>::add_child(new_owner_cid_nid, (collection_id, nft_id));
			}
		}

		Ok((new_owner_account, approval_required))
	}

	fn nft_accept(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple<T::AccountId>,
	) -> Result<(T::AccountId, CollectionId, NftId), DispatchError> {
		let (root_owner, _root_nft) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;

		// Check ownership
		ensure!(sender == root_owner, Error::<T>::NoPermission);

		// Get NFT info
		let mut sending_nft =
			PendingNfts::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		// Prepare acceptance
		let new_owner_account = match new_owner.clone() {
			AccountIdOrCollectionNftTuple::AccountId(id) => id,
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

				let (recipient_root_owner, _root_nft) = Pallet::<T>::lookup_root_owner(cid, nid)?;
				ensure!(
					recipient_root_owner == root_owner,
					Error::<T>::CannotAcceptNonOwnedNft
				);

				// Convert to virtual account
				Pallet::<T>::nft_to_account_id::<T::AccountId>(cid, nid)
			},
		};

		sending_nft.owner = new_owner;
		PendingNfts::<T>::remove(collection_id, nft_id);
		Nfts::<T>::insert(collection_id, nft_id, sending_nft);	

		// Add child to new parent if NFT virtual address
		let new_owner_cid_nid =
			Pallet::<T>::decode_nft_account_id::<T::AccountId>(new_owner_account.clone());
		if let Some(new_owner_cid_nid) = new_owner_cid_nid {
			Pallet::<T>::add_child(new_owner_cid_nid, (collection_id, nft_id));
		}

		Ok((new_owner_account, collection_id, nft_id))
	}

	fn nft_reject(
		sender: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
	) -> Result<(T::AccountId, CollectionId, NftId), DispatchError> {
		let (root_owner, _root_nft) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;

		// Check ownership
		ensure!(sender == root_owner, Error::<T>::CannotRejectNonOwnedNft);

		// Get NFT info
		let mut rejecting_nft =
			PendingNfts::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;

		PendingNfts::<T>::remove(collection_id, nft_id);

		Ok((sender, collection_id, nft_id))
	}
}

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
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
		collection_id: CollectionId,
		nft_id: NftId,
	) -> AccountId {
		(SALT_RMRK_NFT, collection_id, nft_id)
			.using_encoded(|b| AccountId::decode(&mut TrailingZeroInput::new(b)))
			.expect("Decoding with trailing zero never fails; qed.")
	}

	/// Decodes a RMRK NFT a suspected virtual account
	/// then returns an `Option<(CollectionId, NftId)>
	/// where `None` is returned when there is an actual account
	/// and `Some(tuple)` returns tuple of `CollectionId` & `NftId`
	///
	/// Parameters:
	/// - `account_id`: Encoded NFT virtual account or account owner
	///
	/// Output:
	/// `Option<(CollectionId, NftId)>`
	pub fn decode_nft_account_id<AccountId: Codec>(
		account_id: T::AccountId,
	) -> Option<(CollectionId, NftId)> {
		let (prefix, tuple, suffix) = account_id
			.using_encoded(|mut b| {
				let slice = &mut b;
				let r = <([u8; 8], (CollectionId, NftId))>::decode(slice);
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
	/// a tuple of the root `(CollectionId, NftId)`
	/// or an `Error::<T>::NoAvailableNftId` in the case that the NFT is already burned
	///
	/// Parameters:
	/// - `collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `nft_id`: NFT ID that is to be looked up for the root owner
	///
	/// Output:
	/// - `Result<(T::AcccountId, (CollectionId, NftId)), Error<T>>`
	#[allow(clippy::type_complexity)]
	pub fn lookup_root_owner(
		collection_id: CollectionId,
		nft_id: NftId,
	) -> Result<(T::AccountId, (CollectionId, NftId)), Error<T>> {
		let parent = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id);
		// Check if parent returns None which indicates the NFT is not available
		parent.as_ref().ok_or(Error::<T>::NoAvailableNftId)?;
		let owner = parent.unwrap();
		match Self::decode_nft_account_id::<T::AccountId>(owner.clone()) {
			None => Ok((owner, (collection_id, nft_id))),
			Some((cid, nid)) => Pallet::<T>::lookup_root_owner(cid, nid),
		}
	}

	/// Add a child to a parent NFT
	///
	/// Parameters:
	/// - `parent`: Tuple of (CollectionId, NftId) of the parent NFT
	/// - `child`: Tuple of (CollectionId, NftId) of the child NFT to be added
	///
	/// Output:
	/// - Adding a `child` to the Children StorageMap of the `parent`
	pub fn add_child(parent: (CollectionId, NftId), child: (CollectionId, NftId)) {
		Children::<T>::mutate(parent, |v| v.push(child));
	}

	/// Remove a child from a parent NFT
	///
	/// Parameters:
	/// - `parent`: Tuple of (CollectionId, NftId) of the parent NFT
	/// - `child`: Tuple of (CollectionId, NftId) of the child NFT to be removed
	///
	/// Output:
	/// - Removing a `child` from the Children StorageMap of the `parent`
	pub fn remove_child(parent: (CollectionId, NftId), child: (CollectionId, NftId)) {
		Children::<T>::mutate(parent, |v| {
			*v = v.iter().filter(|&nft| *nft != child).cloned().collect();
		});
	}

	/// Has a child NFT present in the Children StorageMap of the parent NFT
	///
	/// Parameters:
	/// - `collection_id`: Collection ID of the NFT to lookup the root owner
	/// - `nft_id`: NFT ID that is to be looked up for the root owner
	///
	/// Output:
	/// - `bool`
	pub fn has_child(parent: (CollectionId, NftId)) -> bool {
		!Children::<T>::get(parent).is_empty()
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
		child_collection_id: CollectionId,
		child_nft_id: NftId,
		parent_collection_id: CollectionId,
		parent_nft_id: NftId,
	) -> bool {
		let mut found_child = false;

		let parent = pallet_uniques::Pallet::<T>::owner(child_collection_id, child_nft_id);
		// Check if parent returns None which indicates the NFT is not available
		if parent.is_none() {
			return found_child
		}
		let owner = parent.as_ref().unwrap();
		match Self::decode_nft_account_id::<T::AccountId>(owner.clone()) {
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

	/// `recursive_burn` function will recursively call itself to burn the NFT and all the children
	/// of the NFT. Any caller functions must be #[transactional]
	///
	/// Parameters:
	/// - `collection_id`: Collection ID of the NFT to be burned
	/// - `nft_id`: NFT ID that is to be burned
	/// - `max_recursion`: Maximum number of recursion allowed
	pub fn recursive_burn(
		collection_id: CollectionId,
		nft_id: NftId,
		max_recursions: u32,
	) -> DispatchResult {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		Nfts::<T>::remove(collection_id, nft_id);
		let kids = Children::<T>::take((collection_id, nft_id));
		for (child_collection_id, child_nft_id) in kids {
			Pallet::<T>::recursive_burn(child_collection_id, child_nft_id, max_recursions - 1)?;
		}
		Ok(())
	}

	pub fn to_bounded_string(name: Vec<u8>) -> Result<BoundedVec<u8, T::StringLimit>, Error<T>> {
		name.try_into().map_err(|_| Error::<T>::TooLong)
	}

	pub fn to_optional_bounded_string(
		name: Option<Vec<u8>>,
	) -> Result<Option<BoundedVec<u8, T::StringLimit>>, Error<T>> {
		Ok(match name {
			Some(n) => Some(Self::to_bounded_string(n)?),
			None => None,
		})
	}

	pub fn get_next_nft_id(collection_id: CollectionId) -> Result<NftId, Error<T>> {
		NextNftId::<T>::try_mutate(collection_id, |id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableNftId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_resource_id() -> Result<ResourceId, Error<T>> {
		NextResourceId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableCollectionId)?;
			Ok(current_id)
		})
	}
}
