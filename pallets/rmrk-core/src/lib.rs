// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-core.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::too_many_arguments)]

use frame_support::{
	dispatch::{DispatchResult, DispatchResultWithPostInfo},
	ensure,
	traits::tokens::{nonfungibles::*, Locker},
	transactional, BoundedVec,
};
use frame_system::ensure_signed;

use sp_runtime::{traits::StaticLookup, DispatchError, Permill};
use sp_std::convert::TryInto;

use rmrk_traits::{
	budget,
	misc::TransferHooks,
	primitives::{BaseId, PartId, ResourceId, SlotId},
	AccountIdOrCollectionNftTuple, BasicResource, Collection, CollectionInfo, ComposableResource,
	Nft, NftChild, NftInfo, PhantomType, Priority, Property, PropertyInfo, Resource, ResourceInfo,
	ResourceInfoMin, ResourceTypes, RoyaltyInfo, SlotResource,
};
use sp_std::result::Result;

mod functions;
pub mod weights;
pub use weights::WeightInfo;

#[cfg(any(feature = "runtime-benchmarks"))]
pub mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type CollectionInfoOf<T> = CollectionInfo<
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
	BoundedVec<u8, <T as Config>::CollectionSymbolLimit>,
	<T as frame_system::Config>::AccountId,
>;

pub type InstanceInfoOf<T> = NftInfo<
	<T as frame_system::Config>::AccountId,
	Permill,
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
	<T as pallet_uniques::Config>::CollectionId,
	<T as pallet_uniques::Config>::ItemId,
>;
pub type ResourceInfoOf<T> = ResourceInfo<
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
	BoundedVec<PartId, <T as Config>::PartsLimit>,
>;

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

pub type BoundedResourceInfoTypeOf<T> = BoundedVec<
	ResourceInfoMin<
		BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
		BoundedVec<PartId, <T as Config>::PartsLimit>,
	>,
	<T as Config>::MaxResourcesOnMint,
>;

pub type PropertyInfoOf<T> = PropertyInfo<KeyLimitOf<T>, ValueLimitOf<T>>;

pub mod types;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<CollectionId, ItemId> {
	fn collection(i: u32) -> CollectionId;
	fn item(i: u32) -> ItemId;
}
#[cfg(feature = "runtime-benchmarks")]
pub struct RmrkBenchmark;

#[cfg(feature = "runtime-benchmarks")]
impl<CollectionId: From<u32>, ItemId: From<u32>> BenchmarkHelper<CollectionId, ItemId>
	for RmrkBenchmark
{
	fn collection(i: u32) -> CollectionId {
		i.into()
	}
	fn item(i: u32) -> ItemId {
		i.into()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ProtocolOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The maximum resource symbol length
		#[pallet::constant]
		type ResourceSymbolLimit: Get<u32>;

		/// The maximum number of parts each resource may have
		#[pallet::constant]
		type PartsLimit: Get<u32>;

		/// The maximum number of resources that can be included in a setpriority extrinsic
		#[pallet::constant]
		type MaxPriorities: Get<u32>;

		/// The maximum number of properties each can have
		#[pallet::constant]
		type PropertiesLimit: Get<u32>;

		/// The maximum nesting allowed in the pallet extrinsics.
		#[pallet::constant]
		type NestingBudget: Get<u32>;

		type CollectionSymbolLimit: Get<u32>;

		type MaxResourcesOnMint: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: BenchmarkHelper<Self::CollectionId, Self::ItemId>;

		type TransferHooks: TransferHooks<Self::AccountId, Self::CollectionId, Self::ItemId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn collections)]
	/// Stores collections info
	pub type Collections<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::CollectionId,
		CollectionInfo<StringLimitOf<T>, BoundedCollectionSymbolOf<T>, T::AccountId>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	/// Stores nft info
	pub type Nfts<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CollectionId,
		Twox64Concat,
		T::ItemId,
		InstanceInfoOf<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn priorities)]
	/// Stores priority info
	pub type Priorities<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::CollectionId>,
			NMapKey<Blake2_128Concat, T::ItemId>,
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
		(T::CollectionId, T::ItemId),
		Twox64Concat,
		(T::CollectionId, T::ItemId),
		(),
	>;

	#[pallet::storage]
	#[pallet::getter(fn resources)]
	/// Stores resource info
	pub type Resources<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::CollectionId>,
			NMapKey<Blake2_128Concat, T::ItemId>,
			NMapKey<Blake2_128Concat, ResourceId>,
		),
		ResourceInfoOf<T>,
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
			NMapKey<Blake2_128Concat, T::CollectionId>,
			NMapKey<Blake2_128Concat, T::ItemId>,
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
			NMapKey<Blake2_128Concat, T::CollectionId>,
			NMapKey<Blake2_128Concat, T::ItemId>,
			NMapKey<Blake2_128Concat, ResourceId>,
			NMapKey<Blake2_128Concat, BaseId>,
			NMapKey<Blake2_128Concat, SlotId>,
		),
		(),
	>;

	#[pallet::storage]
	#[pallet::getter(fn properties)]
	/// Arbitrary properties / metadata of an asset.
	pub type Properties<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::CollectionId>,
			NMapKey<Blake2_128Concat, Option<T::ItemId>>,
			NMapKey<Blake2_128Concat, KeyLimitOf<T>>,
		),
		ValueLimitOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn lock)]
	/// Lock for NFTs
	pub type Lock<T: Config> =
		StorageMap<_, Twox64Concat, (T::CollectionId, T::ItemId), bool, ValueQuery>;

	/// This storage is not used by the chain.
	/// It is need only for PolkadotJS types generation.
	///
	/// The stored types are use in the RPC interface only,
	/// PolkadotJS won't generate TS types for them without this storage.
	#[pallet::storage]
	pub type DummyStorage<T: Config> = StorageValue<
		_,
		(NftChild<T::CollectionId, T::ItemId>, PhantomType<PropertyInfoOf<T>>),
		OptionQuery,
	>;

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
			collection_id: T::CollectionId,
		},
		NftMinted {
			owner: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		},
		NFTBurned {
			owner: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		},
		CollectionDestroyed {
			issuer: T::AccountId,
			collection_id: T::CollectionId,
		},
		NFTSent {
			sender: T::AccountId,
			recipient: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			approval_required: bool,
		},
		NFTAccepted {
			sender: T::AccountId,
			recipient: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		},
		NFTRejected {
			sender: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		},
		IssuerChanged {
			old_issuer: T::AccountId,
			new_issuer: T::AccountId,
			collection_id: T::CollectionId,
		},
		PropertySet {
			collection_id: T::CollectionId,
			maybe_nft_id: Option<T::ItemId>,
			key: KeyLimitOf<T>,
			value: ValueLimitOf<T>,
		},
		PropertyRemoved {
			collection_id: T::CollectionId,
			maybe_nft_id: Option<T::ItemId>,
			key: KeyLimitOf<T>,
		},
		PropertiesRemoved {
			collection_id: T::CollectionId,
			maybe_nft_id: Option<T::ItemId>,
		},
		CollectionLocked {
			issuer: T::AccountId,
			collection_id: T::CollectionId,
		},
		ResourceAdded {
			nft_id: T::ItemId,
			resource_id: ResourceId,
			collection_id: T::CollectionId,
		},
		ResourceReplaced {
			nft_id: T::ItemId,
			resource_id: ResourceId,
			collection_id: T::CollectionId,
		},
		ResourceAccepted {
			nft_id: T::ItemId,
			resource_id: ResourceId,
			collection_id: T::CollectionId,
		},
		ResourceRemoval {
			nft_id: T::ItemId,
			resource_id: ResourceId,
			collection_id: T::CollectionId,
		},
		ResourceRemovalAccepted {
			nft_id: T::ItemId,
			resource_id: ResourceId,
			collection_id: T::CollectionId,
		},
		PrioritySet {
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
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
		NftAlreadyExists,
		EmptyResource,
		/// The recursion limit has been reached.
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
		CannotAcceptToNewOwner,
		FailedTransferHooksPreCheck,
		FailedTransferHooksPostTransfer,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mints an NFT in the specified collection
		/// Sets metadata and the royalty attribute
		///
		/// Parameters:
		/// - `collection_id`: The collection of the asset to be minted.
		/// - `nft_id`: The nft value of the asset to be minted.
		/// - `recipient`: Receiver of the royalty
		/// - `royalty`: Permillage reward from each trade for the Recipient
		/// - `metadata`: Arbitrary data about an nft, e.g. IPFS hash
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::mint_nft())]
		#[transactional]
		pub fn mint_nft(
			origin: OriginFor<T>,
			owner: Option<T::AccountId>,
			nft_id: T::ItemId,
			collection_id: T::CollectionId,
			royalty_recipient: Option<T::AccountId>,
			royalty: Option<Permill>,
			metadata: BoundedVec<u8, T::StringLimit>,
			transferable: bool,
			resources: Option<BoundedResourceInfoTypeOf<T>>,
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
			Self::nft_mint(
				sender,
				nft_owner,
				nft_id,
				collection_id,
				royalty_recipient,
				royalty,
				metadata,
				transferable,
				resources,
			)?;

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
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::mint_nft_directly_to_nft(T::NestingBudget::get()))]
		#[transactional]
		pub fn mint_nft_directly_to_nft(
			origin: OriginFor<T>,
			owner: (T::CollectionId, T::ItemId),
			nft_id: T::ItemId,
			collection_id: T::CollectionId,
			royalty_recipient: Option<T::AccountId>,
			royalty: Option<Permill>,
			metadata: BoundedVec<u8, T::StringLimit>,
			transferable: bool,
			resources: Option<BoundedResourceInfoTypeOf<T>>,
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
			Self::nft_mint_directly_to_nft(
				sender,
				owner,
				nft_id,
				collection_id,
				royalty_recipient,
				royalty,
				metadata,
				transferable,
				resources,
			)?;

			Ok(())
		}

		/// Create a collection
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::create_collection())]
		#[transactional]
		pub fn create_collection(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			metadata: BoundedVec<u8, T::StringLimit>,
			max: Option<u32>,
			symbol: BoundedCollectionSymbolOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::collection_create(sender, collection_id, metadata, max, symbol)?;

			Ok(())
		}

		/// burn nft
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::burn_nft(T::NestingBudget::get(), T::PropertiesLimit::get()))]
		#[transactional]
		pub fn burn_nft(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			// Check ownership
			ensure!(sender == root_owner, Error::<T>::NoPermission);
			Self::nft_burn(root_owner, collection_id, nft_id, &budget)
		}

		/// destroy collection
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::destroy_collection())]
		#[transactional]
		pub fn destroy_collection(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::collection_burn(sender, collection_id)?;

			Ok(())
		}

		/// Transfers a NFT from an Account or NFT A to another Account or NFT B
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to be transferred
		/// - `nft_id`: nft id of the nft to be transferred
		/// - `new_owner`: new owner of the nft which can be either an account or a NFT
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::send_to_account(T::NestingBudget::get()).max(<T as
		pallet::Config>::WeightInfo::send_to_nft(T::NestingBudget::get())))]
		#[transactional]
		pub fn send(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			new_owner: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (_new_owner_account, _approval_required) =
				Self::nft_send(sender.clone(), collection_id, nft_id, new_owner.clone())?;

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
		#[pallet::call_index(6)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::accept_nft(T::NestingBudget::get()))]
		#[transactional]
		pub fn accept_nft(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			new_owner: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _root_nft) =
				Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			// Check ownership
			ensure!(sender == root_owner, Error::<T>::NoPermission);

			let _owner = match pallet_uniques::Pallet::<T>::owner(collection_id, nft_id) {
				Some(owner) => {
					let owner_account =
						match Pallet::<T>::decode_nft_account_id::<T::AccountId>(owner.clone()) {
							Some((cid, nid)) =>
								AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid),
							None => AccountIdOrCollectionNftTuple::AccountId(owner),
						};
					ensure!(new_owner == owner_account, Error::<T>::CannotAcceptToNewOwner)
				},
				None => return Err(Error::<T>::NoAvailableNftId.into()),
			};

			let (_owner_account, _collection_id, _nft_id) =
				Self::nft_accept(sender.clone(), collection_id, nft_id, new_owner)?;

			Ok(())
		}

		/// Rejects an NFT sent from another account to self or owned NFT
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to be accepted
		/// - `nft_id`: nft id of the nft to be accepted
		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::reject_nft(T::NestingBudget::get()))]
		#[transactional]
		pub fn reject_nft(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			Self::nft_reject(sender, collection_id, nft_id)
		}

		/// Change the issuer of a collection
		///
		/// Parameters:
		/// - `origin`: sender of the transaction
		/// - `collection_id`: collection id of the nft to change issuer of
		/// - `new_issuer`: Collection's new issuer
		#[pallet::call_index(8)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::change_collection_issuer())]
		pub fn change_collection_issuer(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
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
		#[pallet::call_index(9)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_property())]
		#[transactional]
		pub fn set_property(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			maybe_nft_id: Option<T::ItemId>,
			key: KeyLimitOf<T>,
			value: ValueLimitOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::property_set(sender, collection_id, maybe_nft_id, key.clone(), value.clone())?;

			Self::deposit_event(Event::PropertySet { collection_id, maybe_nft_id, key, value });
			Ok(())
		}
		/// lock collection
		#[pallet::call_index(10)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::lock_collection())]
		#[transactional]
		pub fn lock_collection(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
			ensure!(collection.issuer == sender, Error::<T>::NoPermission);

			let _collection_id = Self::collection_lock(sender.clone(), collection_id)?;

			Ok(())
		}

		/// Create basic resource
		#[pallet::call_index(11)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_basic_resource(T::NestingBudget::get()))]
		#[transactional]
		pub fn add_basic_resource(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource: BasicResource<StringLimitOf<T>>,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;

			ensure!(collection.issuer == sender, Error::<T>::NoPermission);
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			// Check NFT lock status
			ensure!(
				!Pallet::<T>::is_locked(collection_id, nft_id),
				pallet_uniques::Error::<T>::Locked
			);

			let pending = root_owner != sender;

			Self::resource_add(
				sender,
				collection_id,
				nft_id,
				ResourceTypes::Basic(resource),
				pending,
				resource_id,
			)?;

			Ok(())
		}

		/// Create composable resource
		#[pallet::call_index(12)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_composable_resource(T::NestingBudget::get()))]
		#[transactional]
		pub fn add_composable_resource(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource: ComposableResource<StringLimitOf<T>, BoundedVec<PartId, T::PartsLimit>>,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;

			ensure!(collection.issuer == sender, Error::<T>::NoPermission);
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			// Check NFT lock status
			ensure!(
				!Pallet::<T>::is_locked(collection_id, nft_id),
				pallet_uniques::Error::<T>::Locked
			);

			let pending = root_owner != sender;

			Self::resource_add(
				sender,
				collection_id,
				nft_id,
				ResourceTypes::Composable(resource),
				pending,
				resource_id,
			)?;

			Ok(())
		}

		/// Create slot resource
		#[pallet::call_index(13)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_slot_resource(T::NestingBudget::get()))]
		#[transactional]
		pub fn add_slot_resource(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource: SlotResource<StringLimitOf<T>>,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;

			ensure!(collection.issuer == sender, Error::<T>::NoPermission);
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			// Check NFT lock status
			ensure!(
				!Pallet::<T>::is_locked(collection_id, nft_id),
				pallet_uniques::Error::<T>::Locked
			);

			let pending = root_owner != sender;

			Self::resource_add(
				sender,
				collection_id,
				nft_id,
				ResourceTypes::Slot(resource),
				pending,
				resource_id,
			)?;

			Ok(())
		}

		/// Replace resource by id
		#[pallet::call_index(14)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::replace_resource())]
		#[transactional]
		pub fn replace_resource(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource: ResourceTypes<StringLimitOf<T>, BoundedVec<PartId, T::PartsLimit>>,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::resource_replace(sender, collection_id, nft_id, resource, resource_id)?;

			Ok(())
		}

		/// accept the addition of a new resource to an existing NFT
		#[pallet::call_index(15)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::accept_resource(T::NestingBudget::get()))]
		#[transactional]
		pub fn accept_resource(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let budget = budget::Value::new(T::NestingBudget::get());
			let (owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			ensure!(owner == sender, Error::<T>::NoPermission);
			// Check NFT lock status
			ensure!(!Self::is_locked(collection_id, nft_id), pallet_uniques::Error::<T>::Locked);

			Self::accept(sender, collection_id, nft_id, resource_id)?;

			Ok(())
		}

		/// remove resource
		#[pallet::call_index(16)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::remove_resource(T::NestingBudget::get()))]
		#[transactional]
		pub fn remove_resource(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			ensure!(collection.issuer == sender, Error::<T>::NoPermission);

			// Pending resource if sender is not root owner
			let pending_resource = !(sender == root_owner);

			Self::resource_remove(sender, collection_id, nft_id, resource_id, pending_resource)?;

			Ok(())
		}

		/// accept the removal of a resource of an existing NFT
		#[pallet::call_index(17)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::accept_resource_removal(T::NestingBudget::get()))]
		#[transactional]
		pub fn accept_resource_removal(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			ensure!(root_owner == sender, Error::<T>::NoPermission);

			Self::accept_removal(sender, collection_id, nft_id, resource_id)?;

			Ok(())
		}

		/// set a different order of resource priority
		#[pallet::call_index(18)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_priority(T::MaxPriorities::get(), T::NestingBudget::get()))]
		pub fn set_priority(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			priorities: BoundedVec<ResourceId, T::MaxPriorities>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let budget = budget::Value::new(T::NestingBudget::get());
			let (root_owner, _) = Pallet::<T>::lookup_root_owner(collection_id, nft_id, &budget)?;
			ensure!(sender == root_owner, Error::<T>::NoPermission);
			// Check NFT lock status
			ensure!(!Self::is_locked(collection_id, nft_id), pallet_uniques::Error::<T>::Locked);

			Self::priority_set(sender, collection_id, nft_id, priorities)
		}
	}
}
