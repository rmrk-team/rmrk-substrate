use super::*;

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
}

impl<T: Config> Pallet<T> {
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
}

impl<T: Config> Base<
	T::AccountId,
	CollectionId,
	NftId,
	StringLimitOf<T>,
	BoundedVec<PartType<StringLimitOf<T>, BoundedVec<CollectionId, T::MaxCollectionsEquippablePerPart>>,
	T::PartsLimit>,
	BoundedVec<CollectionId, T::MaxCollectionsEquippablePerPart>
	> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
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
		parts: BoundedVec<PartType<StringLimitOf<T>, BoundedVec<CollectionId, T::MaxCollectionsEquippablePerPart>>, T::PartsLimit>,
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

	/// Implementation of the base_change_issuer function for the Base trait
	/// Called by the change_base_issuer extrinsic to change the issuer of a base
	///
	/// Parameters:
	/// - base_id: The Base ID to change the issuer of
	/// - new_issuer: The Account to become the new issuer
	fn base_change_issuer(
		base_id: BaseId,
		new_issuer: T::AccountId,
	) -> Result<(T::AccountId, CollectionId), DispatchError> {
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
	/// Called by the equip extrinsic to equip a child NFT's resource to a parent's slot, if all are available.
	/// Also can be called to unequip, which can be successful if
	/// - Item has beeen burned
	/// - Item is equipped and extrinsic called by equipping item owner
	/// - Item is equipped and extrinsic called by equipper NFT owner
	/// Equipping operations are maintained inside the Equippings storage.
	/// Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
	///
	/// Parameters:
	/// - issuer: The caller of the function, not necessarily anything else
	/// - item: Child NFT being equipped (or unequipped)
	/// - equipper: Parent NFT which will equip (or unequip) the item
	/// - base_id: ID of the base which the item and equipper must each have a resource referencing
	/// - slot_id: ID of the slot which the item and equipper must each have a resource referencing
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
		equippables: EquippableList<BoundedVec<CollectionId, T::MaxCollectionsEquippablePerPart>>,
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
