// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-equip.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure, BoundedVec,
};

use sp_runtime::traits::StaticLookup;

pub use pallet::*;

pub mod weights;
pub use weights::WeightInfo;

use rmrk_traits::{
	base::EquippableOperation,
	primitives::{BaseId, PartId, ResourceId, SlotId},
	AccountIdOrCollectionNftTuple, Base, BaseInfo, EquippableList, PartType, Theme, ThemeProperty,
};

use sp_std::vec::Vec;

mod functions;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(feature = "runtime-benchmarks")]
use pallet_rmrk_core::BenchmarkHelper;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub type StringLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

pub type BoundedResource<T> = BoundedVec<u8, <T as pallet_rmrk_core::Config>::ResourceSymbolLimit>;

pub type BaseInfoOf<T> = BaseInfo<<T as frame_system::Config>::AccountId, StringLimitOf<T>>;

pub type PartTypeOf<T> = PartType<
	StringLimitOf<T>,
	BoundedVec<
		<T as pallet_uniques::Config>::CollectionId,
		<T as Config>::MaxCollectionsEquippablePerPart,
	>,
>;

pub type ThemePropertyOf<T> = ThemeProperty<StringLimitOf<T>>;

pub type BoundedThemePropertiesOf<T> =
	BoundedVec<ThemePropertyOf<T>, <T as Config>::MaxPropertiesPerTheme>;

pub type BoundedThemeOf<T> = Theme<StringLimitOf<T>, BoundedThemePropertiesOf<T>>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum allowed Parts (either Fixed or Slot) per Base
		#[pallet::constant]
		type MaxPropertiesPerTheme: Get<u32>;

		/// Maximum number of Properties allowed for any Theme
		#[pallet::constant]
		type MaxCollectionsEquippablePerPart: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: BenchmarkHelper<Self::CollectionId, Self::ItemId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn bases)]
	/// Stores Bases info (issuer, base_type, symbol, parts)
	/// TODO https://github.com/rmrk-team/rmrk-substrate/issues/98
	/// Delete Parts from Bases info, as it's kept in Parts storage
	pub type Bases<T: Config> =
		StorageMap<_, Twox64Concat, BaseId, BaseInfo<T::AccountId, StringLimitOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn parts)]
	/// Stores Parts (either FixedPart or SlotPart)
	/// - SlotPart: id, equippable (list), src, z
	/// - FixedPart: id, src, z
	pub type Parts<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		BaseId,
		Twox64Concat,
		PartId,
		PartType<StringLimitOf<T>, BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_base_id)]
	/// Stores the incrementing NextBaseId
	pub type NextBaseId<T: Config> = StorageValue<_, BaseId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_part_id)]
	/// Stores the incrementing NextPartId
	pub type NextPartId<T: Config> = StorageMap<_, Twox64Concat, BaseId, PartId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn equippings)]
	/// Stores Equippings info ((equipper, base, slot), equipped_resource)
	pub type Equippings<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, (T::CollectionId, T::ItemId)>, // Equipper
			NMapKey<Blake2_128Concat, BaseId>,                       // Base ID
			NMapKey<Blake2_128Concat, SlotId>,                       // Slot ID
		),
		ResourceId, // Equipped Resource
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn themes)]
	/// Stores Theme info ((base, theme name, property key), property value)
	pub type Themes<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, BaseId>,           // Base ID
			NMapKey<Blake2_128Concat, StringLimitOf<T>>, // Theme name
			NMapKey<Blake2_128Concat, StringLimitOf<T>>, // Property name (key)
		),
		StringLimitOf<T>, // Property value
		OptionQuery,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A Base was created
		BaseCreated {
			issuer: T::AccountId,
			base_id: BaseId,
		},
		// A Resource was equipped to a base+slot
		SlotEquipped {
			item_collection: T::CollectionId,
			item_nft: T::ItemId,
			base_id: BaseId,
			slot_id: SlotId,
		},
		// A Resource was unequipped
		SlotUnequipped {
			item_collection: T::CollectionId,
			item_nft: T::ItemId,
			base_id: BaseId,
			slot_id: SlotId,
		},
		// A base+slot equippables list was updated
		EquippablesUpdated {
			base_id: BaseId,
			slot_id: SlotId,
		},
		// Base's issuer has changed
		BaseIssuerChanged {
			old_issuer: T::AccountId,
			new_issuer: T::AccountId,
			base_id: BaseId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		// Caller doesn't have permission to perform this operation
		PermissionError,
		// Equipping item NFT doesn't exist
		ItemDoesntExist,
		// Equipper NFT doesn't exist
		EquipperDoesntExist,
		// BaseID exceeds max value
		NoAvailableBaseId,
		// The `MaxCollectionsEquippablePerPart` limit was reached.
		TooManyEquippables,
		// PartId exceeds max value
		NoAvailablePartId,
		// Equipper is not direct parent of item, cannot equip
		MustBeDirectParent,
		// Part (Slot or Fixed) doesn't exist
		PartDoesntExist,
		// Base doesn't exist
		BaseDoesntExist,
		// Only Slot parts can equip, Fixed parts cannot equip
		// TODO redundant w ItemHasNoResourceToEquipThere?
		CantEquipFixedPart,
		// Equipper does not have a Resource associated with this Base
		NoResourceForThisBaseFoundOnNft,
		// Item NFT belongs to a Collection not in Slot Part's equippable list
		CollectionNotEquippable,
		// Item NFT doesn't have a resource for this base+slot
		ItemHasNoResourceToEquipThere,
		// Only Slot parts can equip, Fixed parts cannot equip
		// TODO redundant w CantEquipFixedPart?
		NoEquippableOnFixedPart,
		// No "default" Theme is defined, required prior to defining other themes
		NeedsDefaultThemeFirst,
		// Item is trying to be equipped but is already equipped
		ItemAlreadyEquipped,
		// Slot to which an item is being equipped already has something equipped there
		SlotAlreadyEquipped,
		// Slot from which an item is being unequipped has nothing equipped there
		SlotNotEquipped,
		// Error that should not occur
		// TODO is this being used?
		UnknownError,
		// Attempting to define more Parts than capacity allows
		// TODO confirm this is being used (after https://github.com/rmrk-team/rmrk-substrate/pull/95)
		ExceedsMaxPartsPerBase,
		// Attempting to define more Properties than capacity allows
		// TODO confirm this is being used (after https://github.com/rmrk-team/rmrk-substrate/pull/95)
		TooManyProperties,
		// Cannot unequip an item that isn't equipped yet
		ItemNotEquipped,
		// Cannot unequip an item when caller owns neither the item nor equipper
		UnequipperMustOwnEitherItemOrEquipper,
		// Failure to unwrap as integer value
		UnexpectedTryFromIntError,
		// Converting string to vec error
		UnexpectedVecConversionError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Change the issuer of a Base
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `base_id`: base_id to change issuer of
		/// - `new_issuer`: Base's new issuer
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::change_base_issuer())]
		pub fn change_base_issuer(
			origin: OriginFor<T>,
			base_id: BaseId,
			new_issuer: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let base = Self::bases(base_id).ok_or(Error::<T>::BaseDoesntExist)?;
			ensure!(base.issuer == sender, Error::<T>::PermissionError);
			let new_owner = T::Lookup::lookup(new_issuer)?;

			let (new_owner, base_id) = Self::base_change_issuer(base_id, new_owner)?;

			Self::deposit_event(Event::BaseIssuerChanged {
				old_issuer: sender,
				new_issuer: new_owner,
				base_id,
			});
			Ok(())
		}

		/// Equips a child NFT's resource to a parent's slot, if all are available.
		/// Equipping operations are maintained inside the Equippings storage.
		/// Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
		///
		/// Parameters:
		/// - origin: The caller of the function, not necessarily anything else
		/// - item: Child NFT being equipped
		/// - equipper: Parent NFT which will equip the item
		/// - base: ID of the base which the item and equipper must each have a resource referencing
		/// - slot: ID of the slot which the item and equipper must each have a resource referencing
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::equip())]
		pub fn equip(
			origin: OriginFor<T>,
			item: (T::CollectionId, T::ItemId),
			equipper: (T::CollectionId, T::ItemId),
			resource_id: ResourceId,
			base: BaseId,
			slot: SlotId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (collection_id, nft_id, base_id, slot_id) =
				Self::do_equip(sender, item, equipper, resource_id, base, slot)?;

			// Send Equip event
			Self::deposit_event(Event::SlotEquipped {
				item_collection: collection_id,
				item_nft: nft_id,
				base_id,
				slot_id,
			});

			Ok(())
		}

		/// Unequips a child NFT's resource from its parent's slot.
		/// Can be successful if
		/// - Item has beeen burned
		/// - Item is equipped and extrinsic called by equipping item owner
		/// - Item is equipped and extrinsic called by equipper NFT owner
		/// Equipping operations are maintained inside the Equippings storage.
		/// Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
		///
		/// Parameters:
		/// - origin: The caller of the function, not necessarily anything else
		/// - item: Child NFT being unequipped
		/// - unequipper: Parent NFT which will unequip the item
		/// - base: ID of the base which the item and equipper must each have a resource referencing
		/// - slot: ID of the slot which the item and equipper must each have a resource referencing
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unequip())]
		pub fn unequip(
			origin: OriginFor<T>,
			item: (T::CollectionId, T::ItemId),
			unequipper: (T::CollectionId, T::ItemId),
			base: BaseId,
			slot: SlotId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (collection_id, nft_id, base_id, slot_id) =
				Self::do_unequip(sender, item, unequipper, base, slot)?;

			Self::deposit_event(Event::SlotUnequipped {
				item_collection: collection_id,
				item_nft: nft_id,
				base_id,
				slot_id,
			});

			Ok(())
		}

		/// Updates the array of Collections allowed to be equipped to a Base's specified Slot Part.
		/// Modeled after [equippable interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equippable.md)
		///
		/// Parameters:
		/// - origin: The caller of the function, must be issuer of the base
		/// - base_id: The Base containing the Slot Part to be updated
		/// - part_id: The Slot Part whose Equippable List is being updated
		/// - equippables: The list of equippables that will override the current Equippaables list
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::equippable())]
		pub fn equippable(
			origin: OriginFor<T>,
			base_id: BaseId,
			slot_id: SlotId,
			equippables: EquippableList<
				BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
			>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (base_id, slot_id) = Self::do_equippable(
				sender,
				base_id,
				slot_id,
				EquippableOperation::Override(equippables),
			)?;

			Self::deposit_event(Event::EquippablesUpdated { base_id, slot_id });
			Ok(())
		}

		/// Adds a new collection that is allowed to be equipped to a Base's specified Slot Part.
		///
		/// Parameters:
		/// - origin: The caller of the function, must be issuer of the base
		/// - base_id: The Base containing the Slot Part to be updated
		/// - part_id: The Slot Part whose Equippable List is being updated
		/// - equippable: The equippable that will be added to the current Equippaables list
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::equippable_add())]
		pub fn equippable_add(
			origin: OriginFor<T>,
			base_id: BaseId,
			slot_id: SlotId,
			equippable: T::CollectionId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (base_id, slot_id) = Self::do_equippable(
				sender,
				base_id,
				slot_id,
				EquippableOperation::Add(equippable),
			)?;

			Self::deposit_event(Event::EquippablesUpdated { base_id, slot_id });
			Ok(())
		}

		/// Remove a collection from the equippables list.
		///
		/// Parameters:
		/// - origin: The caller of the function, must be issuer of the base
		/// - base_id: The Base containing the Slot Part to be updated
		/// - part_id: The Slot Part whose Equippable List is being updated
		/// - equippable: The equippable that will be removed from the current Equippaables list
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::equippable_remove())]
		pub fn equippable_remove(
			origin: OriginFor<T>,
			base_id: BaseId,
			slot_id: SlotId,
			equippable: T::CollectionId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (base_id, slot_id) = Self::do_equippable(
				sender,
				base_id,
				slot_id,
				EquippableOperation::Remove(equippable),
			)?;

			Self::deposit_event(Event::EquippablesUpdated { base_id, slot_id });
			Ok(())
		}

		/// Adds a Theme to a Base.
		/// Modeled after [themeadd interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/themeadd.md)
		/// Themes are stored in the Themes storage
		/// A Theme named "default" is required prior to adding other Themes.
		///
		/// Parameters:
		/// - origin: The caller of the function, must be issuer of the base
		/// - base_id: The Base containing the Theme to be updated
		/// - theme: The Theme to add to the Base.  A Theme has a name and properties, which are an
		///   array of [key, value, inherit].  This array is bounded by MaxPropertiesPerTheme.
		///   - key: arbitrary BoundedString, defined by client
		///   - value: arbitrary BoundedString, defined by client
		///   - inherit: optional bool
		#[pallet::call_index(6)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::theme_add())]
		pub fn theme_add(
			origin: OriginFor<T>,
			base_id: BaseId,
			theme: BoundedThemeOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let number_of_properties: u32 = match theme.properties.len().try_into() {
				Ok(num) => num,
				Err(_e) => return Err(Error::<T>::UnexpectedTryFromIntError.into()),
			};

			ensure!(
				number_of_properties <= T::MaxPropertiesPerTheme::get(),
				Error::<T>::TooManyProperties
			);

			let _theme_id = Self::add_theme(sender, base_id, theme)?;

			// Self::deposit_event(Event::SomethingStored(something, sender));
			Ok(())
		}

		/// Creates a new Base.
		/// Modeled after [base interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/base.md)
		///
		/// Parameters:
		/// - origin: Caller, will be assigned as the issuer of the Base
		/// - base_type: media type, e.g. "svg"
		/// - symbol: arbitrary client-chosen symbol, e.g. "kanaria_superbird"
		/// - parts: array of Fixed and Slot parts composing the base, confined in length by
		///   PartsLimit
		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::create_base())]
		pub fn create_base(
			origin: OriginFor<T>,
			base_type: BoundedVec<u8, T::StringLimit>,
			symbol: BoundedVec<u8, T::StringLimit>,
			parts: BoundedVec<
				PartType<
					StringLimitOf<T>,
					BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
				>,
				T::PartsLimit,
			>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let base_id = Self::base_create(sender.clone(), base_type, symbol, parts)?;

			Self::deposit_event(Event::BaseCreated { issuer: sender, base_id });
			Ok(())
		}
	}
}
