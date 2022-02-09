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

impl<T: Config> Pallet<T> 
{
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
		parts: Vec<NewPartTypes<StringLimitOf<T>>>,
	) -> Result<BaseId, DispatchError> {
		let base_id = Self::get_next_base_id()?;
		for part in parts.clone() {
			let mut pid = 0;
			match part.clone() {
				NewPartTypes::SlotPart(p) => {
					pid = p.id;
					Parts::<T>::insert(base_id, pid, part);
				},
				NewPartTypes::FixedPart(p) => {
					pid = p.id;
					Parts::<T>::insert(base_id, pid, part);
				}
			}
			// let part_id = Self::get_next_part_id(base_id)?;
			// Parts::<T>::insert(base_id, pid, part);
		}
		let base = BaseInfo { issuer, base_type, symbol, parts };
		Bases::<T>::insert(base_id, base);
		Ok(base_id)
	}
	fn do_equip(
		issuer: T::AccountId, // Maybe don't need?
		item_collection_id: CollectionId,
		item_nft_id: NftId,
		equipper_collection_id: CollectionId,
		equipper_nft_id: NftId,
		base_id: u32, // Maybe BaseId ?
		slot_id: u32 // Maybe SlotId ?
	)-> Result<(), DispatchError> {
		

		let item_is_equipped = Equippings::<T>::get(((equipper_collection_id, equipper_nft_id), base_id, slot_id)).is_some();
		let item_exists = pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id).is_some();
		
		// If item doesn't exist, anyone can unequip it.
		if !item_exists && item_is_equipped {
			Equippings::<T>::remove(((equipper_collection_id, equipper_nft_id), base_id, slot_id));
			//TODO emit removal event
			return Ok(());
		}

		let item_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(item_collection_id, item_nft_id)?;
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(equipper_collection_id, equipper_nft_id)?;

		// If the item is equipped, and either the equipper or the item owner is the caller, it will be unequipped
		if item_is_equipped && (item_owner.0 == issuer || equipper_owner.0 == issuer) {
			Equippings::<T>::remove(((equipper_collection_id, equipper_nft_id), base_id, slot_id));
			//TODO emit removal event
			return Ok(());
		}

		// Equipper NFT must exist
		ensure!(
			pallet_rmrk_core::Pallet::<T>::nfts(equipper_collection_id, equipper_nft_id).is_some(),
			Error::<T>::EquipperDoesntExist
		);

		// Item must exist
		ensure!(
			item_exists,
			Error::<T>::ItemDoesntExist
		);

		// Caller must root-own equipper?
		ensure!(equipper_owner.0 == issuer, Error::<T>::PermissionError);

		// Caller must root-own item
		ensure!(item_owner.0 == issuer, Error::<T>::PermissionError);

		// Equipper must be direct parent of item
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::nfts(item_collection_id, item_nft_id).unwrap().owner;
		ensure!(
			equipper_owner == AccountIdOrCollectionNftTuple::CollectionAndNftTuple(equipper_collection_id, equipper_nft_id),
			Error::<T>::MustBeDirectParent
		);

		// Equipper must have a resource that is associated with the provided base ID?
		// First we iterate through the resources added to this NFT in search of the base ID
		let mut found_base_resource_on_nft = false;
		let mut resource_id = 999;
		let resources_matching_base_iter = pallet_rmrk_core::NewResources::<T>::iter_prefix_values(
			(
				equipper_collection_id,
				equipper_nft_id,
				// Some(base_id)
			)
		);
		for resource in resources_matching_base_iter {
			match resource {
				ResourceType::Base(_) => (),
				ResourceType::Slot(slot_resource) => {
					// println!("sr: {:?}", slot_resource.base);
					if slot_resource.base == base_id {
						found_base_resource_on_nft = true;
						resource_id = slot_resource.id;
					}
				},
			}
		};

		// If we don't find a matching base resource, we raise a NoResourceForThisBaseFoundOnNft error
		if !found_base_resource_on_nft {
			return Err(Error::<T>::NoResourceForThisBaseFoundOnNft.into())
		}

		// Find the specific Part to equip 
		match Self::parts(base_id, slot_id) {
			// Part must exist
			None => Err(Error::<T>::PartDoesntExist),
			Some(part_type) => {
				match part_type {
					NewPartTypes::FixedPart(v) => {
						// Part must be SlotPart type
						Err(Error::<T>::CantEquipFixedPart)
					},
					NewPartTypes::SlotPart(v) => {
						// Collection must be in item's equippable list?
						if !v.equippable.contains(&item_collection_id) {
							return Err(Error::<T>::CollectionNotEquippable.into())
						}

						// The item being equipped must be have a resource equippable into that base.slot
						let mut found_base_slot_resource_on_nft = false;
						let mut to_equip_resource_id = 999;
						let resources_matching_base_iter = pallet_rmrk_core::NewResources::<T>::iter_prefix_values(
							(
								item_collection_id,
								item_nft_id,
								// None::<BaseId>
							)
						);

						for resource in resources_matching_base_iter {
							match resource {
								ResourceType::Base(b) => {
									if b.base == base_id && b.slot_id == slot_id {
										found_base_slot_resource_on_nft = true;
										to_equip_resource_id = b.id;
									}
								},
								ResourceType::Slot(_) => (),
							}
						};

						// Item has no resource to equip into that slot
						if !found_base_slot_resource_on_nft {
							return Err(Error::<T>::ItemHasNoResourceToEquipThere.into());
						}

						// Equip item
						Equippings::<T>::insert(
							((equipper_collection_id, equipper_nft_id), base_id, slot_id),
							to_equip_resource_id
						);

						// Emit event
						Self::deposit_event(Event::SlotEquipped { 
							collection_id: equipper_collection_id,
							nft_id: equipper_nft_id,
							item_collection: item_collection_id,
							item_nft: item_nft_id,
							base_id: base_id,
							slot_id: slot_id,								
						});
						Ok(())
					}
				}
			}
		}?;
		Ok(())
	}
}
