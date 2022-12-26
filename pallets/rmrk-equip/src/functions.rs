// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-equip.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use super::*;
use frame_support::traits::{tokens::Locker, Get};
use rmrk_traits::budget;

use sp_std::collections::btree_set::BTreeSet;

impl<T: Config> Pallet<T> {
	/// Helper function for getting next base ID
	/// Currently, BaseId is auto-incremented from zero, may be worth changing
	/// to BoundedVec to allow arbitrary/unique naming, making cross-chain functionality
	/// more tenable
	pub fn get_next_base_id() -> Result<BaseId, Error<T>> {
		NextBaseId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableBaseId)?;
			Ok(current_id)
		})
	}

	/// Helper function for getting next part ID for a base
	/// Like BaseId, PartId is auto-incremented from zero, which similarly may be worth changing
	/// to BoundedVec to allow arbitrary/unique naming, making cross-chain functionality
	/// more tenable
	pub fn get_next_part_id(base_id: BaseId) -> Result<BaseId, Error<T>> {
		NextPartId::<T>::try_mutate(base_id, |id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailablePartId)?;
			Ok(current_id)
		})
	}

	/// Helper function for checking if an item is equipped
	/// If the Equippings storage contains the Base/Slot for the Collection+NFT ID, the item is
	/// already equipped
	pub fn slot_is_equipped(
		item: (T::CollectionId, T::ItemId),
		base_id: BaseId,
		slot_id: SlotId,
	) -> bool {
		let (equipper_collection_id, equipper_nft_id) = item;
		Equippings::<T>::get(((equipper_collection_id, equipper_nft_id), base_id, slot_id))
			.is_some()
	}

	pub fn iterate_part_types(base_id: BaseId) -> impl Iterator<Item = PartTypeOf<T>> {
		Parts::<T>::iter_prefix_values(base_id)
	}

	pub fn iterate_theme_names(base_id: BaseId) -> impl Iterator<Item = StringLimitOf<T>> {
		Themes::<T>::iter_key_prefix((base_id,)).map(|(theme_name, ..)| theme_name)
	}

	pub fn get_theme(
		base_id: BaseId,
		theme_name: StringLimitOf<T>,
		filter_keys: Option<BTreeSet<StringLimitOf<T>>>,
	) -> Result<Option<BoundedThemeOf<T>>, Error<T>> {
		let properties: BoundedThemePropertiesOf<T> =
			Self::query_theme_kv(base_id, &theme_name, filter_keys)?;

		if properties.is_empty() {
			Ok(None)
		} else {
			Ok(Some(BoundedThemeOf::<T> { name: theme_name, properties, inherit: false }))
		}
	}

	fn query_theme_kv(
		base_id: BaseId,
		theme_name: &StringLimitOf<T>,
		filter_keys: Option<BTreeSet<StringLimitOf<T>>>,
	) -> Result<BoundedThemePropertiesOf<T>, Error<T>> {
		BoundedVec::try_from(
			Themes::<T>::iter_prefix((base_id, theme_name.clone()))
				.filter(|(key, _)| match &filter_keys {
					Some(filter_keys) => filter_keys.contains(key),
					None => true,
				})
				.map(|(key, value)| ThemeProperty { key, value })
				.collect::<Vec<_>>(),
		)
		.or(Err(Error::<T>::TooManyProperties))
	}
}

impl<T: Config>
	Base<
		T::AccountId,
		T::CollectionId,
		T::ItemId,
		StringLimitOf<T>,
		BoundedVec<
			PartType<
				StringLimitOf<T>,
				BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
			>,
			T::PartsLimit,
		>,
		BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
		BoundedVec<ThemeProperty<BoundedVec<u8, T::StringLimit>>, T::MaxPropertiesPerTheme>,
	> for Pallet<T>
{
	/// Implementation of the base_create function for the Base trait
	/// Called by the create_base extrinsic to create a new Base.
	/// Modeled after [base interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/base.md)
	///
	/// Parameters:
	/// - issuer: The issuer of the Base, implied as the caller/creator
	/// - base_type: media type, e.g. "svg"
	/// - symbol: arbitrary client-chosen symbol, e.g. "kanaria_superbird"
	/// - parts: array of Fixed and Slot parts composing the base, confined in length by PartsLimit
	fn base_create(
		issuer: T::AccountId,
		base_type: StringLimitOf<T>,
		symbol: StringLimitOf<T>,
		parts: BoundedVec<
			PartType<
				StringLimitOf<T>,
				BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
			>,
			T::PartsLimit,
		>,
	) -> Result<BaseId, DispatchError> {
		let base_id = Self::get_next_base_id()?;
		for part in parts {
			match part.clone() {
				PartType::SlotPart(p) => {
					Parts::<T>::insert(base_id, p.id, part);
				},
				PartType::FixedPart(p) => {
					Parts::<T>::insert(base_id, p.id, part);
				},
			}
		}
		let base = BaseInfo { issuer, base_type, symbol };
		Bases::<T>::insert(base_id, base);
		Ok(base_id)
	}

	/// Implementation of the base_change_issuer function for the Base trait
	/// called by the change_base_issuer extrinsic to change the issuer of a base
	///
	/// Parameters:
	/// - base_id: The Base ID to change the issuer of
	/// - new_issuer: The Account to become the new issuer
	fn base_change_issuer(
		base_id: BaseId,
		new_issuer: T::AccountId,
	) -> Result<(T::AccountId, BaseId), DispatchError> {
		ensure!(Bases::<T>::contains_key(base_id), Error::<T>::NoAvailableBaseId);

		Bases::<T>::try_mutate_exists(base_id, |base| -> DispatchResult {
			if let Some(b) = base {
				b.issuer = new_issuer.clone();
			}
			Ok(())
		})?;

		Ok((new_issuer, base_id))
	}

	/// Implementation of the do_equip function for the Base trait
	/// Called by the equip extrinsic to equip a child NFT's resource to a parent's slot, if all are
	/// available.
	/// Equipping operations are maintained inside the Equippings storage.
	/// Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
	///
	/// Parameters:
	/// - issuer: The caller of the function, not necessarily anything else
	/// - item: Child NFT being equipped
	/// - equipper: Parent NFT which will equip the item
	/// - base_id: ID of the base which the item and equipper must each have a resource referencing
	/// - slot_id: ID of the slot which the item and equipper must each have a resource referencing
	fn do_equip(
		issuer: T::AccountId,
		item: (T::CollectionId, T::ItemId),
		equipper: (T::CollectionId, T::ItemId),
		resource_id: ResourceId,
		base_id: BaseId,
		slot_id: SlotId,
	) -> Result<(T::CollectionId, T::ItemId, BaseId, SlotId), DispatchError> {
		let item_collection_id = item.0;
		let item_nft_id = item.1;
		let equipper_collection_id = equipper.0;
		let equipper_nft_id = equipper.1;

		// Equipper must exist
		ensure!(
			pallet_rmrk_core::Pallet::<T>::nfts(equipper_collection_id, equipper_nft_id).is_some(),
			Error::<T>::EquipperDoesntExist
		);

		// Check item NFT lock status
		ensure!(
			!pallet_rmrk_core::Pallet::<T>::is_locked(item_collection_id, item_nft_id),
			pallet_uniques::Error::<T>::Locked
		);
		// Check equipper NFT lock status
		ensure!(
			!pallet_rmrk_core::Pallet::<T>::is_locked(equipper_collection_id, equipper_nft_id),
			pallet_uniques::Error::<T>::Locked
		);

		match pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id) {
			None => {
				// Item must exist
				return Err(Error::<T>::ItemDoesntExist.into())
			},
			Some(nft) => {
				// Item must not already be equipped
				if nft.equipped.is_some() {
					return Err(Error::<T>::ItemAlreadyEquipped.into())
				}
			},
		}

		// If the Equippings storage contains the Base/Slot for the Collection+NFT ID, the item is
		// already equipped
		ensure!(
			!Self::slot_is_equipped((equipper_collection_id, equipper_nft_id), base_id, slot_id),
			Error::<T>::SlotAlreadyEquipped
		);

		// Caller must root-own item
		let budget = budget::Value::new(T::NestingBudget::get());
		let item_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(
			item_collection_id,
			item_nft_id,
			&budget,
		)?;
		ensure!(item_owner.0 == issuer, Error::<T>::PermissionError);

		// Caller must root-own equipper
		let budget = budget::Value::new(T::NestingBudget::get());
		let (equipper_root_owner, _) = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(
			equipper_collection_id,
			equipper_nft_id,
			&budget,
		)?;
		ensure!(equipper_root_owner == issuer, Error::<T>::PermissionError);

		// Equipper must be direct parent of item
		let equipper_direct_owner =
			match pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id) {
				None => {
					// Item must exist (shouldn't happen here, already checked above)
					return Err(Error::<T>::ItemDoesntExist.into())
				},
				Some(nft) => nft.owner,
			};

		ensure!(
			equipper_direct_owner ==
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(
					equipper_collection_id,
					equipper_nft_id
				),
			Error::<T>::MustBeDirectParent
		);

		// Equipper must have a resource that is associated with the provided base ID
		// First we iterate through the resources added to this NFT in search of the base ID
		ensure!(
			pallet_rmrk_core::Pallet::<T>::equippable_bases((
				equipper_collection_id,
				equipper_nft_id,
				// resource_id,
				base_id
			))
			.is_some(),
			Error::<T>::NoResourceForThisBaseFoundOnNft
		);

		// The item being equipped must have a resource that is equippable into that base.slot
		ensure!(
			pallet_rmrk_core::Pallet::<T>::equippable_slots((
				item_collection_id,
				item_nft_id,
				resource_id,
				base_id,
				slot_id
			))
			.is_some(),
			Error::<T>::ItemHasNoResourceToEquipThere
		);

		// Check the PartType stored for this Base + Slot
		match Self::parts(base_id, slot_id) {
			// Part must exist
			None => return Err(Error::<T>::PartDoesntExist.into()),
			Some(part) => {
				match part {
					PartType::FixedPart(_) => {
						// Part must be SlotPart type
						Err(Error::<T>::CantEquipFixedPart.into())
					},
					PartType::SlotPart(slot_part) => {
						// Collection must be in item's equippable list
						match slot_part.equippable {
							EquippableList::Empty =>
								return Err(Error::<T>::CollectionNotEquippable.into()),
							EquippableList::All => (),
							EquippableList::Custom(eq) =>
								if !eq.contains(&item_collection_id) {
									return Err(Error::<T>::CollectionNotEquippable.into())
								},
						}

						// Equip item (add to Equippings)
						Equippings::<T>::insert(
							((equipper_collection_id, equipper_nft_id), base_id, slot_id),
							resource_id,
						);

						// Update item's equipped property
						pallet_rmrk_core::Nfts::<T>::try_mutate_exists(
							item_collection_id,
							item_nft_id,
							|nft| -> DispatchResult {
								if let Some(nft) = nft {
									nft.equipped = Some((resource_id, slot_id));
								}
								Ok(())
							},
						)?;
						Ok((item_collection_id, item_nft_id, base_id, slot_id))
					},
				}
			},
		}
	}

	/// Implementation of the do_unequip function for the Base trait
	/// Called by the equip extrinsic to unequip a child NFT's resource from a parent's slot, if it
	/// is equipped. Unequip can be successful if
	/// - Item has been burned
	/// - Item is equipped and extrinsic called by equipping item owner
	/// - Item is equipped and extrinsic called by equipper NFT owner
	/// Equipping operations are maintained inside the Equippings storage.
	/// Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
	///
	/// Parameters:
	/// - issuer: The caller of the function, not necessarily anything else
	/// - item: Child NFT being equipped (or unequipped)
	/// - equipper: Parent NFT which will equip (or unequip) the item
	/// - base_id: ID of the equipped item's base
	/// - slot_id: ID of the equipped item's slot
	fn do_unequip(
		issuer: T::AccountId,
		item: (T::CollectionId, T::ItemId),
		equipper: (T::CollectionId, T::ItemId),
		base_id: BaseId,
		slot_id: SlotId,
	) -> Result<(T::CollectionId, T::ItemId, BaseId, SlotId), DispatchError> {
		let item_collection_id = item.0;
		let item_nft_id = item.1;
		let equipper_collection_id = equipper.0;
		let equipper_nft_id = equipper.1;
		// Check item NFT lock status
		ensure!(
			!pallet_rmrk_core::Pallet::<T>::is_locked(item_collection_id, item_nft_id),
			pallet_uniques::Error::<T>::Locked
		);
		// Check equipper NFT lock status
		ensure!(
			!pallet_rmrk_core::Pallet::<T>::is_locked(equipper_collection_id, equipper_nft_id),
			pallet_uniques::Error::<T>::Locked
		);

		ensure!(
			Self::slot_is_equipped((equipper_collection_id, equipper_nft_id), base_id, slot_id),
			Error::<T>::SlotNotEquipped
		);

		// Check if the item already exists
		let item_exists =
			pallet_rmrk_core::Pallet::<T>::nft_exists((item_collection_id, item_nft_id));

		// If item doesn't exist, anyone can unequip it.  This can happen because burn_nft can
		// happen in rmrk-core, which doesn't know about rmrk-equip.
		if !item_exists {
			// Remove from Equippings nft/base/slot storage
			Equippings::<T>::remove(((equipper_collection_id, equipper_nft_id), base_id, slot_id));

			// Update item's equipped property
			pallet_rmrk_core::Nfts::<T>::try_mutate_exists(
				item_collection_id,
				item_nft_id,
				|nft| -> DispatchResult {
					if let Some(nft) = nft {
						nft.equipped = None;
					}
					Ok(())
				},
			)?;

			// Return unequip event details
			return Ok((item_collection_id, item_nft_id, base_id, slot_id))
		}

		let budget = budget::Value::new(T::NestingBudget::get());
		let item_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(
			item_collection_id,
			item_nft_id,
			&budget,
		)?;
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(
			equipper_collection_id,
			equipper_nft_id,
			&budget,
		)?;

		let issuer_owns_either_equipper_or_item =
			item_owner.0 == issuer || equipper_owner.0 == issuer;
		ensure!(
			issuer_owns_either_equipper_or_item,
			Error::<T>::UnequipperMustOwnEitherItemOrEquipper
		);

		// Remove from Equippings nft/base/slot storage
		Equippings::<T>::remove(((equipper_collection_id, equipper_nft_id), base_id, slot_id));

		// Update item's equipped property
		pallet_rmrk_core::Nfts::<T>::try_mutate_exists(
			item_collection_id,
			item_nft_id,
			|nft| -> DispatchResult {
				if let Some(nft) = nft {
					nft.equipped = None;
				}
				Ok(())
			},
		)?;
		Ok((item_collection_id, item_nft_id, base_id, slot_id))
	}

	/// Implementation of the equippable function for the Base trait
	/// Called by the equippable extrinsic to update the array of Collections allowed
	/// to be equipped to a Base's specified Slot Part.
	/// Modeled after [equippable interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equippable.md)
	///
	/// Parameters:
	/// - issuer: The caller of the function, must be issuer of the base
	/// - base_id: The Base containing the Slot Part to be updated
	/// - part_id: The Slot Part whose Equippable List is being updated
	/// - equippables: The list of equippables that will override the current Equippaables list
	/// TODO: address https://github.com/rmrk-team/rmrk-substrate/issues/97
	/// Should be able to handle additions/deletions, not just overrides
	fn do_equippable(
		issuer: T::AccountId,
		base_id: BaseId,
		part_id: PartId,
		operation: EquippableOperation<
			T::CollectionId,
			BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
		>,
	) -> Result<(BaseId, SlotId), DispatchError> {
		// Caller must be issuer of base
		match Bases::<T>::get(base_id) {
			// Base must exist
			None => return Err(Error::<T>::BaseDoesntExist.into()),
			Some(base) => {
				// Issuer must be Base issuer
				ensure!(base.issuer == issuer, Error::<T>::PermissionError)
			},
		}

		match Parts::<T>::get(base_id, part_id) {
			// Part must exist
			None => return Err(Error::<T>::PartDoesntExist.into()),
			Some(part) => {
				match part {
					PartType::FixedPart(_) => {
						// Fixed part has no equippables
						Err(Error::<T>::NoEquippableOnFixedPart.into())
					},
					PartType::SlotPart(mut slot_part) => {
						match operation {
							EquippableOperation::Add(equippable) => {
								if let EquippableList::Custom(mut equippables) =
									slot_part.equippable
								{
									let _ = equippables
										.try_push(equippable)
										.map_err(|_| Error::<T>::TooManyEquippables)?;
									slot_part.equippable = EquippableList::Custom(equippables);
								}
							},
							EquippableOperation::Remove(equippable) =>
								if let EquippableList::Custom(mut equippables) =
									slot_part.equippable
								{
									equippables.retain(|e| *e != equippable);
									slot_part.equippable = EquippableList::Custom(equippables);
								},
							EquippableOperation::Override(equippables) => {
								slot_part.equippable = equippables;
							},
						};
						// Overwrite Parts entry for this base_id.part_id
						Parts::<T>::insert(base_id, part_id, PartType::SlotPart(slot_part));
						Ok((base_id, part_id))
					},
				}
			},
		}
	}

	/// Implementation of the add_theme function for the Base trait
	/// Called by the theme_add extrinsic to add a theme to a Base.
	/// Modeled after [themeadd interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/themeadd.md)
	/// Themes are stored in the Themes storage
	/// A "default" theme is required prior to adding other Themes.
	///
	/// Parameters:
	/// - issuer: The caller of the function, must be issuer of the base
	/// - base_id: The Base containing the Theme to be updated
	/// - theme: The Theme to add to the Base.  A Theme has a name and properties, which are an
	///   array of [key, value, inherit].  This array is bounded by MaxPropertiesPerTheme.
	///   - key: arbitrary BoundedString, defined by client
	///   - value: arbitrary BoundedString, defined by client
	///   - inherit: optional bool
	fn add_theme(
		issuer: T::AccountId,
		base_id: BaseId,
		theme: Theme<
			BoundedVec<u8, T::StringLimit>,
			BoundedVec<ThemeProperty<BoundedVec<u8, T::StringLimit>>, T::MaxPropertiesPerTheme>,
		>,
	) -> Result<(), DispatchError> {
		match Bases::<T>::get(base_id) {
			// Base must exist
			None => return Err(Error::<T>::BaseDoesntExist.into()),
			Some(base) => {
				// Sender must be issuer of the base
				ensure!(base.issuer == issuer, Error::<T>::PermissionError);
			},
		}

		// The string "default" as a BoundedVec
		let default_as_bv: BoundedVec<u8, T::StringLimit> =
			match "default".as_bytes().to_vec().try_into() {
				Err(_e) => return Err(Error::<T>::UnexpectedVecConversionError.into()),
				Ok(val) => val,
			};

		// Check for existence of default theme (default theme cannot be empty)
		let def_count = Themes::<T>::iter_prefix_values((base_id, default_as_bv.clone())).count();

		// If either the default theme doesn't already exist, nor is it currently being passed, we
		// fail
		ensure!(def_count >= 1 || theme.name == default_as_bv, Error::<T>::NeedsDefaultThemeFirst);

		// Iterate through each property
		for prop in theme.properties {
			Themes::<T>::insert((base_id, theme.name.clone(), prop.key), prop.value)
		}
		Ok(())
	}
}
