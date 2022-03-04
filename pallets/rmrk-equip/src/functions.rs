use super::*;

impl<T: Config> Pallet<T> {
	pub fn get_next_base_id() -> Result<BaseId, Error<T>> {
		NextBaseId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableBaseId)?;
			Ok(current_id)
		})
	}
}

impl<T: Config> Pallet<T> {
	pub fn get_next_part_id(base_id: BaseId) -> Result<BaseId, Error<T>> {
		NextPartId::<T>::try_mutate(base_id, |id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailablePartId)?;
			Ok(current_id)
		})
	}
}

impl<T: Config> Base<T::AccountId, CollectionId, NftId, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	fn base_create(
		issuer: T::AccountId,
		base_type: StringLimitOf<T>,
		symbol: StringLimitOf<T>,
		parts: Vec<PartType<StringLimitOf<T>>>,
	) -> Result<BaseId, DispatchError> {
		let base_id = Self::get_next_base_id()?;
		for part in parts.clone() {
			match part.clone() {
				PartType::SlotPart(p) => {
					Parts::<T>::insert(base_id, p.id, part);
				},
				PartType::FixedPart(p) => {
					Parts::<T>::insert(base_id, p.id, part);
				},
			}
		}
		let base = BaseInfo { issuer, base_type, symbol, parts };
		Bases::<T>::insert(base_id, base);
		Ok(base_id)
	}

	fn do_equip(
		issuer: T::AccountId,
		item: (CollectionId, NftId),
		equipper: (CollectionId, NftId),
		base_id: BaseId,
		slot_id: SlotId,
	) -> Result<(CollectionId, NftId, BaseId, SlotId, bool), DispatchError> {
		let item_collection_id = item.0;
		let item_nft_id = item.1;
		let equipper_collection_id = equipper.0;
		let equipper_nft_id = equipper.1;

		let item_is_equipped =
			Equippings::<T>::get(((equipper_collection_id, equipper_nft_id), base_id, slot_id))
				.is_some();
		let item_exists =
			pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id).is_some();

		// If item doesn't exist, anyone can unequip it.
		if !item_exists && item_is_equipped {
			// Remove from Equippings nft/base/slot storage
			Equippings::<T>::remove(((equipper_collection_id, equipper_nft_id), base_id, slot_id));

			// Update item's equipped property
			pallet_rmrk_core::Nfts::<T>::try_mutate_exists(
				item_collection_id,
				item_nft_id,
				|nft| -> DispatchResult {
					if let Some(nft) = nft {
						nft.equipped = false;
					}
					Ok(())
				},
			)?;

			// Return unequip event details
			return Ok((item_collection_id, item_nft_id, base_id, slot_id, false))
		}

		let item_owner =
			pallet_rmrk_core::Pallet::<T>::lookup_root_owner(item_collection_id, item_nft_id)?;
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(
			equipper_collection_id,
			equipper_nft_id,
		)?;

		// If the item is equipped in this slot, and either the equipper or the item owner is the
		// caller, it will be unequipped
		if item_is_equipped && (item_owner.0 == issuer || equipper_owner.0 == issuer) {
			// Remove from Equippings nft/base/slot storage
			Equippings::<T>::remove(((equipper_collection_id, equipper_nft_id), base_id, slot_id));

			// Update item's equipped property
			pallet_rmrk_core::Nfts::<T>::try_mutate_exists(
				item_collection_id,
				item_nft_id,
				|nft| -> DispatchResult {
					if let Some(nft) = nft {
						nft.equipped = false;
					}
					Ok(())
				},
			)?;

			// Return unequip event details
			return Ok((item_collection_id, item_nft_id, base_id, slot_id, false))
		}

		// Equipper NFT must exist
		ensure!(
			pallet_rmrk_core::Pallet::<T>::nfts(equipper_collection_id, equipper_nft_id).is_some(),
			Error::<T>::EquipperDoesntExist
		);

		// Item must exist
		ensure!(item_exists, Error::<T>::ItemDoesntExist);

		// Is the item equipped anywhere?
		ensure!(
			!pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id)
				.unwrap()
				.equipped,
			Error::<T>::AlreadyEquipped
		);

		// Caller must root-own equipper?
		ensure!(equipper_owner.0 == issuer, Error::<T>::PermissionError);

		// Caller must root-own item
		ensure!(item_owner.0 == issuer, Error::<T>::PermissionError);

		// Equipper must be direct parent of item
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id)
			.unwrap()
			.owner;
		ensure!(
			equipper_owner ==
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(
					equipper_collection_id,
					equipper_nft_id
				),
			Error::<T>::MustBeDirectParent
		);

		// Equipper must have a resource that is associated with the provided base ID
		// First we iterate through the resources added to this NFT in search of the base ID
		let mut found_base_resource_on_nft = false;
		let resources_matching_base_iter = pallet_rmrk_core::Resources::<T>::iter_prefix_values((
			equipper_collection_id,
			equipper_nft_id,
		));
		for resource in resources_matching_base_iter {
			if resource.base.is_some() && resource.base.unwrap() == base_id {
				found_base_resource_on_nft = true;
			}
		}

		// If we don't find a matching base resource, we raise a NoResourceForThisBaseFoundOnNft
		// error
		ensure!(found_base_resource_on_nft, Error::<T>::NoResourceForThisBaseFoundOnNft);

		// The item being equipped must be have a resource that is equippable into that base.slot
		let mut found_base_slot_resource_on_nft = false;
		// initialized so the compiler doesn't complain, though it will be overwritten if it
		// resource exists
		let mut to_equip_resource_id: BoundedResource<T> = b"".to_vec().try_into().unwrap();
		let resources_matching_base_iter =
			pallet_rmrk_core::Resources::<T>::iter_prefix_values((item_collection_id, item_nft_id));

		for resource in resources_matching_base_iter {
			if resource.base.is_some() &&
				resource.slot.is_some() &&
				resource.base.unwrap() == base_id &&
				resource.slot.unwrap() == slot_id
			{
				found_base_slot_resource_on_nft = true;
				to_equip_resource_id = resource.id;
			}
		}
		ensure!(found_base_slot_resource_on_nft, Error::<T>::ItemHasNoResourceToEquipThere);

		// Part must exist
		ensure!(Self::parts(base_id, slot_id).is_some(), Error::<T>::PartDoesntExist);

		// Returns Result
		match Self::parts(base_id, slot_id).unwrap() {
			PartType::FixedPart(_) => {
				// Part must be SlotPart type
				Err(Error::<T>::CantEquipFixedPart.into())
			},
			PartType::SlotPart(slot_part) => {
				// Collection must be in item's equippable list
				match slot_part.equippable {
					EquippableList::Empty => return Err(Error::<T>::CollectionNotEquippable.into()),
					EquippableList::All => (),
					EquippableList::Custom(eq) =>
						if !eq.contains(&item_collection_id) {
							return Err(Error::<T>::CollectionNotEquippable.into())
						},
				}

				// Equip item (add to Equippings)
				Equippings::<T>::insert(
					((equipper_collection_id, equipper_nft_id), base_id, slot_id),
					to_equip_resource_id,
				);

				// Update item's equipped property
				pallet_rmrk_core::Nfts::<T>::try_mutate_exists(
					item_collection_id,
					item_nft_id,
					|nft| -> DispatchResult {
						if let Some(nft) = nft {
							nft.equipped = true;
						}
						Ok(())
					},
				)?;
				Ok((item_collection_id, item_nft_id, base_id, slot_id, true))
			},
		}
	}

	fn do_equippable(
		issuer: T::AccountId,
		base_id: BaseId,
		part_id: PartId,
		equippables: EquippableList,
	) -> Result<(BaseId, SlotId), DispatchError> {
		// Base must exist
		ensure!(Bases::<T>::get(base_id).is_some(), Error::<T>::BaseDoesntExist);

		// Caller must be issuer of base
		ensure!(Bases::<T>::get(base_id).unwrap().issuer == issuer, Error::<T>::PermissionError);

		// Part must exist
		ensure!(Parts::<T>::get(base_id, part_id).is_some(), Error::<T>::PartDoesntExist);

		match Parts::<T>::get(base_id, part_id).unwrap() {
			PartType::FixedPart(_) => {
				// Fixed part has no equippables
				Err(Error::<T>::NoEquippableOnFixedPart.into())
			},
			PartType::SlotPart(mut slot_part) => {
				// Update equippable value
				slot_part.equippable = equippables;
				// Overwrite Parts entry for this base_id.part_id
				Parts::<T>::insert(base_id, part_id, PartType::SlotPart(slot_part));
				Ok((base_id, part_id))
			},
		}
	}

	fn add_theme(
		issuer: T::AccountId,
		base_id: BaseId,
		theme: Theme<BoundedVec<u8, T::StringLimit>>,
	) -> Result<(), DispatchError> {
		// Base must exist
		ensure!(Bases::<T>::get(base_id).is_some(), Error::<T>::BaseDoesntExist);

		// Sender must be issuer of the base
		ensure!(Bases::<T>::get(base_id).unwrap().issuer == issuer, Error::<T>::PermissionError);

		// The string "default" as a BoundedVec
		let default_as_bv: BoundedVec<u8, T::StringLimit> =
			"default".as_bytes().to_vec().try_into().unwrap();

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
