// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-core.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::too_many_arguments)]

use frame_support::{
	dispatch::DispatchResult, ensure, traits::tokens::nonfungibles::*, transactional, BoundedVec,
};
use frame_system::ensure_signed;

use sp_runtime::{traits::StaticLookup, DispatchError, Permill};
use sp_std::convert::TryInto;

use rmrk_traits::{
	primitives::*, AccountIdOrCollectionNftTuple, BasicResource, Collection, CollectionInfo,
	ComposableResource, Nft, NftInfo, Priority, Property, Resource, ResourceInfo, ResourceTypes,
	RoyaltyInfo, SlotResource,
};
use sp_std::result::Result;

mod functions;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type InstanceInfoOf<T> = NftInfo<
	<T as frame_system::Config>::AccountId,
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
>;
pub type ResourceOf<T, P> =
	ResourceInfo<BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>, BoundedVec<PartId, P>>;

pub type BoundedCollectionSymbolOf<T> = BoundedVec<u8, <T as Config>::CollectionSymbolLimit>;

pub type StringLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

pub type BoundedResource<R> = BoundedVec<u8, R>;

pub type KeyLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>;

pub type ValueLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>;

pub type BoundedResourceTypeOf<T> = BoundedVec<
	ResourceTypes<
		BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
		BoundedVec<PartId, <T as Config>::PartsLimit>,
	>,
	<T as Config>::MaxResourcesOnMint,
>;

pub mod types;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ProtocolOrigin: EnsureOrigin<Self::Origin>;
		type MaxRecursions: Get<u32>;

		/// The maximum resource symbol length
		#[pallet::constant]
		type ResourceSymbolLimit: Get<u32>;

		/// The maximum number of parts each resource may have
		#[pallet::constant]
		type PartsLimit: Get<u32>;

		/// The maximum number of resources that can be included in a setpriority extrinsic
		#[pallet::constant]
		type MaxPriorities: Get<u32>;

		type CollectionSymbolLimit: Get<u32>;

		type MaxResourcesOnMint: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_nft_id)]
	pub type NextNftId<T: Config> = StorageMap<_, Twox64Concat, CollectionId, NftId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collection_index)]
	pub type CollectionIndex<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	/// Next available Resource ID.
	#[pallet::storage]
	#[pallet::getter(fn next_resource_id)]
	pub type NextResourceId<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CollectionId,
		Twox64Concat,
		NftId,
		ResourceId,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn collections)]
	/// Stores collections info
	pub type Collections<T: Config> = StorageMap<
		_,
		Twox64Concat,
		CollectionId,
		CollectionInfo<StringLimitOf<T>, BoundedCollectionSymbolOf<T>, T::AccountId>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	/// Stores nft info
	pub type Nfts<T: Config> =
		StorageDoubleMap<_, Twox64Concat, CollectionId, Twox64Concat, NftId, InstanceInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn priorities)]
	/// Stores priority info
	pub type Priorities<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, ResourceId>,
		),
		u32,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn children)]
	/// Stores nft children info
	pub type Children<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		(CollectionId, NftId),
		Twox64Concat,
		(CollectionId, NftId),
		(),
	>;

	#[pallet::storage]
	#[pallet::getter(fn resources)]
	/// Stores resource info
	pub type Resources<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, ResourceId>,
		),
		ResourceOf<T, T::PartsLimit>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn equippable_bases)]
	/// Stores the existence of a base for a particular NFT
	/// This is populated on `add_composable_resource`, and is
	/// used in the rmrk-equip pallet when equipping a resource.
	pub type EquippableBases<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, BaseId>,
		),
		(),
	>;

	#[pallet::storage]
	#[pallet::getter(fn equippable_slots)]
	/// Stores the existence of a Base + Slot for a particular
	/// NFT's particular resource.  This is populated on
	/// `add_slot_resource`, and is used in the rmrk-equip
	/// pallet when equipping a resource.
	pub type EquippableSlots<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, ResourceId>,
			NMapKey<Blake2_128Concat, BaseId>,
			NMapKey<Blake2_128Concat, SlotId>,
		),
		(),
	>;

	#[pallet::storage]
	#[pallet::getter(fn properties)]
	/// Metadata of an asset class.
	pub(super) type Properties<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, Option<NftId>>,
			NMapKey<Blake2_128Concat, KeyLimitOf<T>>,
		),
		ValueLimitOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn lock)]
	/// Lock for NFTs
	pub type Lock<T: Config> = StorageMap<_, Twox64Concat, (CollectionId, NftId), bool, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectionCreated {
			issuer: T::AccountId,
			collection_id: CollectionId,
		},
		// NftMinted(T::AccountId, CollectionId, NftId),
		NftMinted {
			owner: AccountIdOrCollectionNftTuple<T::AccountId>,
			collection_id: CollectionId,
			nft_id: NftId,
		},
		NFTBurned {
			owner: T::AccountId,
			nft_id: NftId,
		},
		CollectionDestroyed {
			issuer: T::AccountId,
			collection_id: CollectionId,
		},
		NFTSent {
			sender: T::AccountId,
			recipient: AccountIdOrCollectionNftTuple<T::AccountId>,
			collection_id: CollectionId,
			nft_id: NftId,
			approval_required: bool,
		},
		NFTAccepted {
			sender: T::AccountId,
			recipient: AccountIdOrCollectionNftTuple<T::AccountId>,
			collection_id: CollectionId,
			nft_id: NftId,
		},
		NFTRejected {
			sender: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
		},
		IssuerChanged {
			old_issuer: T::AccountId,
			new_issuer: T::AccountId,
			collection_id: CollectionId,
		},
		PropertySet {
			collection_id: CollectionId,
			maybe_nft_id: Option<NftId>,
			key: KeyLimitOf<T>,
			value: ValueLimitOf<T>,
		},
		CollectionLocked {
			issuer: T::AccountId,
			collection_id: CollectionId,
		},
		ResourceAdded {
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		},
		ResourceAccepted {
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		},
		ResourceRemoval {
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		},
		ResourceRemovalAccepted {
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		},
		PrioritySet {
			collection_id: CollectionId,
			nft_id: NftId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		TooLong,
		NoAvailableCollectionId,
		NoAvailableResourceId,
		MetadataNotSet,
		RecipientNotSet,
		NoAvailableNftId,
		NotInRange,
		RoyaltyNotSet,
		CollectionUnknown,
		NoPermission,
		NoWitness,
		CollectionNotEmpty,
		CollectionFullOrLocked,
		CannotSendToDescendentOrSelf,
		ResourceAlreadyExists,
		EmptyResource,
		TooManyRecursions,
		NftIsLocked,
		CannotAcceptNonOwnedNft,
		CannotRejectNonOwnedNft,
		CannotRejectNonPendingNft,
		ResourceDoesntExist,
		/// Accepting a resource that is not pending should fail
		ResourceNotPending,
		NonTransferable,
		// Must unequip an item before sending (this only applies to the
		// rmrk-equip pallet but the send operation lives in rmrk-core)
		CannotSendEquippedItem,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<CollectionId = CollectionId, ItemId = NftId>,
	{
		/// Mints an NFT in the specified collection
		/// Sets metadata and the royalty attribute
		///
		/// Parameters:
		/// - `collection_id`: The class of the asset to be minted.
		/// - `nft_id`: The nft value of the asset to be minted.
		/// - `recipient`: Receiver of the royalty
		/// - `royalty`: Permillage reward from each trade for the Recipient
		/// - `metadata`: Arbitrary data about an nft, e.g. IPFS hash
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn mint_nft(
			origin: OriginFor<T>,
			owner: Option<T::AccountId>,
			collection_id: CollectionId,
			royalty_recipient: Option<T::AccountId>,
			royalty: Option<Permill>,
			metadata: BoundedVec<u8, T::StringLimit>,
			transferable: bool,
			resources: Option<BoundedResourceTypeOf<T>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			if let Some(collection_issuer) =
				pallet_uniques::Pallet::<T>::collection_owner(collection_id)
			{
				ensure!(collection_issuer == sender, Error::<T>::NoPermission);
			} else {
				return Err(Error::<T>::CollectionUnknown.into())
			}

			// Extract intended owner or default to sender
			let nft_owner = match owner {
				Some(owner) => owner,
				None => sender.clone(),
			};

			// Mint NFT for RMRK storage
			let (collection_id, nft_id) = Self::nft_mint(
				sender.clone(),
				nft_owner.clone(),
				collection_id,
				royalty_recipient,
				royalty,
				metadata,
				transferable,
			)?;

			pallet_uniques::Pallet::<T>::do_mint(
				collection_id,
				nft_id,
				nft_owner.clone(),
				|_details| Ok(()),
			)?;

			// Add all at-mint resources
			if let Some(resources) = resources {
				for res in resources {
					Self::resource_add(sender.clone(), collection_id, nft_id, res, true)?;
				}
			}

			Self::deposit_event(Event::NftMinted {
				owner: AccountIdOrCollectionNftTuple::AccountId(nft_owner),
				collection_id,
				nft_id,
			});

			Ok(())
		}

		/// Mints an NFT in the specified collection directly to another NFT
		/// Sets metadata and the royalty attribute
		///
		/// Parameters:
		/// - `collection_id`: The class of the asset to be minted.
		/// - `nft_id`: The nft value of the asset to be minted.
		/// - `recipient`: Receiver of the royalty
		/// - `royalty`: Permillage reward from each trade for the Recipient
		/// - `metadata`: Arbitrary data about an nft, e.g. IPFS hash
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn mint_nft_directly_to_nft(
			origin: OriginFor<T>,
			owner: (CollectionId, NftId),
			collection_id: CollectionId,
			royalty_recipient: Option<T::AccountId>,
			royalty: Option<Permill>,
			metadata: BoundedVec<u8, T::StringLimit>,
			transferable: bool,
			resources: Option<BoundedResourceTypeOf<T>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// Collection must exist and sender must be issuer of collection
			if let Some(collection_issuer) =
				pallet_uniques::Pallet::<T>::collection_owner(collection_id)
			{
				ensure!(collection_issuer == sender, Error::<T>::NoPermission);
			} else {
				return Err(Error::<T>::CollectionUnknown.into())
			}

			// Mint NFT for RMRK storage
			let (collection_id, nft_id) = Self::nft_mint_directly_to_nft(
				sender.clone(),
				owner,
				collection_id,
				royalty_recipient,
				royalty,
				metadata,
				transferable,
			)?;

			// For Uniques, we need to decode the "virtual account" ID to be the owner
			let uniques_owner = Self::nft_to_account_id(owner.0, owner.1);

			pallet_uniques::Pallet::<T>::do_mint(
				collection_id,
				nft_id,
				uniques_owner,
				|_details| Ok(()),
			)?;

			// Add all at-mint resources
			if let Some(resources) = resources {
				for res in resources {
					Self::resource_add(sender.clone(), collection_id, nft_id, res, true)?;
				}
			}

			Self::deposit_event(Event::NftMinted {
				owner: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(owner.0, owner.1),
				collection_id,
				nft_id,
			});

			Ok(())
		}

		/// Create a collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn create_collection(
			origin: OriginFor<T>,
			metadata: BoundedVec<u8, T::StringLimit>,
			max: Option<u32>,
			symbol: BoundedCollectionSymbolOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let collection_id = Self::collection_create(sender.clone(), metadata, max, symbol)?;

			pallet_uniques::Pallet::<T>::do_create_collection(
				collection_id,
				sender.clone(),
				sender.clone(),
				T::CollectionDeposit::get(),
				false,
				pallet_uniques::Event::Created {
					collection: collection_id,
					creator: sender.clone(),
					owner: sender.clone(),
				},
			)?;

			Self::deposit_event(Event::CollectionCreated { issuer: sender, collection_id });
			Ok(())
		}

		/// burn nft
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn burn_nft(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			max_burns: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;
			// Check ownership
			ensure!(sender == root_owner, Error::<T>::NoPermission);
			let (_collection_id, nft_id) = Self::nft_burn(collection_id, nft_id, max_burns)?;

			pallet_uniques::Pallet::<T>::do_burn(collection_id, nft_id, |_, _| Ok(()))?;

			Self::deposit_event(Event::NFTBurned { owner: sender, nft_id });
			Ok(())
		}

		/// destroy collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn destroy_collection(
			origin: OriginFor<T>,
			collection_id: CollectionId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::collection_burn(sender.clone(), collection_id)?;

			let witness = pallet_uniques::Pallet::<T>::get_destroy_witness(&collection_id)
				.ok_or(Error::<T>::NoWitness)?;
			ensure!(witness.items == 0u32, Error::<T>::CollectionNotEmpty);

			pallet_uniques::Pallet::<T>::do_destroy_collection(
				collection_id,
				witness,
				sender.clone().into(),
			)?;

			Self::deposit_event(Event::CollectionDestroyed { issuer: sender, collection_id });
			Ok(())
		}

		/// Transfers a NFT from an Account or NFT A to another Account or NFT B
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to be transferred
		/// - `nft_id`: nft id of the nft to be transferred
		/// - `new_owner`: new owner of the nft which can be either an account or a NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn send(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			new_owner: AccountIdOrCollectionNftTuple<T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (new_owner_account, approval_required) =
				Self::nft_send(sender.clone(), collection_id, nft_id, new_owner.clone())?;

			pallet_uniques::Pallet::<T>::do_transfer(
				collection_id,
				nft_id,
				new_owner_account,
				|_class_details, _details| Ok(()),
			)?;

			Self::deposit_event(Event::NFTSent {
				sender,
				recipient: new_owner.clone(),
				collection_id,
				nft_id,
				approval_required,
			});

			Ok(())
		}
		/// Accepts an NFT sent from another account to self or owned NFT
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to be accepted
		/// - `nft_id`: nft id of the nft to be accepted
		/// - `new_owner`: either origin's account ID or origin-owned NFT, whichever the NFT was
		///   sent to
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn accept_nft(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			new_owner: AccountIdOrCollectionNftTuple<T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (new_owner_account, collection_id, nft_id) =
				Self::nft_accept(sender.clone(), collection_id, nft_id, new_owner.clone())?;

			pallet_uniques::Pallet::<T>::do_transfer(
				collection_id,
				nft_id,
				new_owner_account,
				|_class_details, _details| Ok(()),
			)?;

			Self::deposit_event(Event::NFTAccepted {
				sender,
				recipient: new_owner.clone(),
				collection_id,
				nft_id,
			});
			Ok(())
		}

		/// Rejects an NFT sent from another account to self or owned NFT
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to be accepted
		/// - `nft_id`: nft id of the nft to be accepted
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn reject_nft(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let max_recursions = T::MaxRecursions::get();
			let (sender, collection_id, nft_id) =
				Self::nft_reject(sender, collection_id, nft_id, max_recursions)?;

			Self::deposit_event(Event::NFTRejected { sender, collection_id, nft_id });
			Ok(())
		}

		/// Change the issuer of a collection
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to change issuer of
		/// - `new_issuer`: Collection's new issuer
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn change_collection_issuer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			new_issuer: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
			ensure!(collection.issuer == sender, Error::<T>::NoPermission);
			let new_owner = T::Lookup::lookup(new_issuer.clone())?;

			ensure!(
				Collections::<T>::contains_key(collection_id),
				Error::<T>::NoAvailableCollectionId
			);

			let (new_owner, collection_id) =
				Self::collection_change_issuer(collection_id, new_owner)?;

			pallet_uniques::Pallet::<T>::transfer_ownership(origin, collection_id, new_issuer)?;

			Self::deposit_event(Event::IssuerChanged {
				old_issuer: sender,
				new_issuer: new_owner,
				collection_id,
			});
			Ok(())
		}

		/// set a custom value on an NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_property(
			origin: OriginFor<T>,
			#[pallet::compact] collection_id: CollectionId,
			maybe_nft_id: Option<NftId>,
			key: KeyLimitOf<T>,
			value: ValueLimitOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::property_set(sender, collection_id, maybe_nft_id, key.clone(), value.clone())?;

			Self::deposit_event(Event::PropertySet { collection_id, maybe_nft_id, key, value });
			Ok(())
		}
		/// lock collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn lock_collection(
			origin: OriginFor<T>,
			collection_id: CollectionId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let collection_id = Self::collection_lock(sender.clone(), collection_id)?;

			Self::deposit_event(Event::CollectionLocked { issuer: sender, collection_id });
			Ok(())
		}

		/// Create basic resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn add_basic_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource: BasicResource<StringLimitOf<T>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let resource_id = Self::resource_add(
				sender,
				collection_id,
				nft_id,
				ResourceTypes::Basic(resource),
				false,
			)?;

			Self::deposit_event(Event::ResourceAdded { collection_id, nft_id, resource_id });
			Ok(())
		}

		/// Create composable resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn add_composable_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource: ComposableResource<StringLimitOf<T>, BoundedVec<PartId, T::PartsLimit>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let resource_id = Self::resource_add(
				sender,
				collection_id,
				nft_id,
				ResourceTypes::Composable(resource),
				false,
			)?;

			Self::deposit_event(Event::ResourceAdded { collection_id, nft_id, resource_id });
			Ok(())
		}

		/// Create slot resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn add_slot_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource: SlotResource<StringLimitOf<T>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let resource_id = Self::resource_add(
				sender,
				collection_id,
				nft_id,
				ResourceTypes::Slot(resource),
				false,
			)?;

			Self::deposit_event(Event::ResourceAdded { collection_id, nft_id, resource_id });
			Ok(())
		}

		/// accept the addition of a new resource to an existing NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn accept_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(
				Resources::<T>::get((collection_id, nft_id, resource_id)).is_some(),
				Error::<T>::ResourceDoesntExist
			);

			let (owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;
			ensure!(owner == sender, Error::<T>::NoPermission);

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

			Self::deposit_event(Event::ResourceAccepted { collection_id, nft_id, resource_id });
			Ok(())
		}

		/// remove resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn remove_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::resource_remove(sender, collection_id, nft_id, resource_id)?;

			Self::deposit_event(Event::ResourceRemoval { collection_id, nft_id, resource_id });
			Ok(())
		}

		/// accept the removal of a resource of an existing NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn accept_resource_removal(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::accept_removal(sender, collection_id, nft_id, resource_id)?;

			Self::deposit_event(Event::ResourceRemovalAccepted { collection_id, nft_id, resource_id });
			Ok(())
		}

		/// set a different order of resource priority
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_priority(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			priorities: BoundedVec<ResourceId, T::MaxPriorities>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::priority_set(sender, collection_id, nft_id, priorities)?;
			Self::deposit_event(Event::PrioritySet { collection_id, nft_id });
			Ok(())
		}
	}
}
