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
			let part_id = Self::get_next_part_id(base_id)?;
			// println!("partID: {:?}\npart: {:?}", part_id, part);
			Parts::<T>::insert(base_id, part_id, part);
		}
		let base = BaseInfo { issuer, base_type, symbol, parts };
		Bases::<T>::insert(base_id, base);
		Ok(base_id)
	}
	fn do_equip(
		issuer: T::AccountId, // Maybe don't need?
		equipping_item_collection_id: CollectionId,
		equipping_item_nft_id: NftId,
		equipper_collection_id: CollectionId,
		equipper_nft_id: NftId,
		base_id: u32, // Maybe BaseId ?
		slot: u32 // Maybe SlotId ?
	)-> Result<(), DispatchError> {

		// Does item exist?
		ensure!(
			pallet_rmrk_core::Pallet::<T>::nfts(equipping_item_collection_id, equipping_item_nft_id).is_some(),
			Error::<T>::ItemDoesntExist
		);
		// Does equipper nft exist?
		ensure!(
			pallet_rmrk_core::Pallet::<T>::nfts(equipper_collection_id, equipper_nft_id).is_some(),
			Error::<T>::EquipperDoesntExist
		);
		// Does caller own item?
		println!("issuer: {:?}", issuer);
		let item_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(equipping_item_collection_id, equipping_item_nft_id)?;
		ensure!(item_owner.0 == issuer, Error::<T>::PermissionError);

		// Does caller own equipper?
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(equipper_collection_id, equipper_nft_id)?;
		ensure!(equipper_owner.0 == issuer, Error::<T>::PermissionError);

		// Is equipper direct parent of item?
		let equipper_owner = pallet_rmrk_core::Pallet::<T>::nfts(equipping_item_collection_id, equipping_item_nft_id).unwrap().owner;
		ensure!(
			equipper_owner == AccountIdOrCollectionNftTuple::CollectionAndNftTuple(equipper_collection_id, equipper_nft_id),
			Error::<T>::MustBeDirectParent
		);

		match Self::parts(base_id, slot) {
			// Does base.slot exist?
			None => Err(Error::<T>::BaseSlotDoesntExist),
			Some(v) => {
				println!("ok");
				Ok(())
			}
		}?;
		// Is base.slot SlotPart?  Should fail on FixedPart
		// 

		// If it exists, then unequip.  Otherwise, equip it.

		// Is item collection in base.slot's equippable list?
		Equippings::<T>::insert((equipper_collection_id, equipper_nft_id), base_id, slot);
		Ok(())
	}
}
