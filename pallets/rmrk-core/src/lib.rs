#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use codec::HasCompact;
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{
		tokens::nonfungibles::*, BalanceStatus, Currency, NamedReservableCurrency,
		ReservableCurrency,
	},
	transactional, BoundedVec,
};
use frame_system::ensure_signed;

use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd, StaticLookup, Zero},
	DispatchError, Permill,
};
use sp_std::{convert::TryInto, vec, vec::Vec};

use types::{ClassInfo, ResourceInfo};

use rmrk_traits::{
	primitives::*, AccountIdOrCollectionNftTuple, Collection, CollectionInfo, Nft, NftInfo,
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
	CollectionId,
	NftId,
>;
pub type ResourceOf<T> =
	ResourceInfo<ResourceId, BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>>;

pub type StringLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

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
	pub type NextResourceId<T: Config> = StorageValue<_, ResourceId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collections)]
	/// Stores collections info
	pub type Collections<T: Config> =
		StorageMap<_, Twox64Concat, CollectionId, CollectionInfo<StringLimitOf<T>, T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn get_nfts_by_owner)]
	/// Stores collections info
	pub type NftsByOwner<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Vec<(CollectionId, NftId)>>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	/// Stores nft info
	pub type NFTs<T: Config> =
		StorageDoubleMap<_, Twox64Concat, CollectionId, Twox64Concat, NftId, InstanceInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn priorities)]
	/// Stores priority info
	pub type Priorities<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CollectionId,
		Twox64Concat,
		NftId,
		Vec<BoundedVec<u8, T::StringLimit>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn children)]
	/// Stores nft info
	pub type Children<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CollectionId,
		Twox64Concat,
		NftId,
		Vec<(CollectionId, NftId)>,
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
		ResourceOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn properties)]
	/// Metadata of an asset class.
	pub(super) type Properties<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, Option<NftId>>,
			NMapKey<Blake2_128Concat, BoundedVec<u8, T::KeyLimit>>,
		),
		BoundedVec<u8, T::ValueLimit>,
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
			collection_id: CollectionId,
		},
		// NftMinted(T::AccountId, CollectionId, NftId),
		NftMinted {
			owner: T::AccountId,
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
			recipient: AccountIdOrCollectionNftTuple<T::AccountId, CollectionId, NftId>,
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
			key: BoundedVec<u8, T::KeyLimit>,
			value: BoundedVec<u8, T::ValueLimit>,
		},
		CollectionLocked {
			issuer: T::AccountId,
			collection_id: CollectionId,
		},
		ResourceAdded {
			nft_id: NftId,
			resource_id: ResourceId,
		},
		ResourceAccepted {
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
		CannotSendToDescendent,
		ResourceAlreadyExists,
		EmptyResource,
		TooManyRecursions,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
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
			owner: T::AccountId,
			collection_id: CollectionId,
			recipient: Option<T::AccountId>,
			royalty: Option<Permill>,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let rootowner = owner.clone();

			let (collection_id, nft_id) = <Self as Nft<T::AccountId, StringLimitOf<T>>>::mint_nft(
				sender.clone().unwrap_or_default(),
				owner,
				collection_id,
				recipient,
				royalty,
				metadata,
			)?;

			pallet_uniques::Pallet::<T>::do_mint(
				collection_id.into(),
				nft_id.into(),
				sender.clone().unwrap_or_default(),
				|_details| Ok(()),
			)?;

			Self::deposit_event(Event::NftMinted {
				owner: sender.unwrap_or_default(),
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
			symbol: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let max = max.unwrap_or_default();

			let collection_id =
				<Self as Collection<StringLimitOf<T>, T::AccountId>>::create_collection(
					sender.clone().unwrap_or_default(),
					metadata,
					max,
					symbol,
				)?;

			pallet_uniques::Pallet::<T>::do_create_class(
				collection_id.into(),
				sender.clone().unwrap_or_default(),
				sender.clone().unwrap_or_default(),
				T::ClassDeposit::get(),
				false,
				pallet_uniques::Event::Created(
					collection_id.into(),
					sender.clone().unwrap_or_default(),
					sender.clone().unwrap_or_default(),
				),
			);

			Self::deposit_event(Event::CollectionCreated {
				issuer: sender.clone().unwrap_or_default(),
				collection_id,
			});
			Ok(())
		}

		/// burn nft
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn burn_nft(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			let max_recursions = T::MaxRecursions::get();
			let (_collection_id, nft_id) = <Self as Nft<T::AccountId, StringLimitOf<T>>>::burn_nft(
				collection_id,
				nft_id,
				max_recursions,
			)?;

			pallet_uniques::Pallet::<T>::do_burn(collection_id.into(), nft_id.into(), |_, _| {
				Ok(())
			})?;

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
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			<Self as Collection<StringLimitOf<T>, T::AccountId>>::burn_collection(
				sender.clone().unwrap_or_default(),
				collection_id,
			)?;

			let witness = pallet_uniques::Pallet::<T>::get_destroy_witness(&collection_id.into())
				.ok_or(Error::<T>::NoWitness)?;
			ensure!(witness.instances == 0u32, Error::<T>::CollectionNotEmpty);

			pallet_uniques::Pallet::<T>::do_destroy_class(
				collection_id.into(),
				witness,
				sender.clone(),
			);

			Self::deposit_event(Event::CollectionDestroyed {
				issuer: sender.unwrap_or_default(),
				collection_id,
			});
			Ok(())
		}

		/// transfer NFT from account A to (account B or NFT)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn send(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			new_owner: AccountIdOrCollectionNftTuple<T::AccountId, CollectionId, NftId>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			}
			.unwrap_or_default();

			let max_recursions = T::MaxRecursions::get();
			<Self as Nft<T::AccountId, StringLimitOf<T>>>::send(
				sender.clone(),
				collection_id,
				nft_id,
				new_owner.clone(),
				max_recursions,
			)?;

			Self::deposit_event(Event::NFTSent {
				sender,
				recipient: new_owner,
				collection_id,
				nft_id,
			});
			Ok(())
		}

		/// changing the issuer of a collection or a base
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn change_issuer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			new_issuer: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			let new_issuer = T::Lookup::lookup(new_issuer)?;

			ensure!(
				Collections::<T>::contains_key(collection_id),
				Error::<T>::NoAvailableCollectionId
			);

			let (new_issuer, collection_id) = <Self as Collection<
				StringLimitOf<T>,
				T::AccountId,
			>>::change_issuer(collection_id, new_issuer)?;

			Self::deposit_event(Event::IssuerChanged {
				old_issuer: sender.unwrap_or_default(),
				new_issuer,
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
			key: BoundedVec<u8, T::KeyLimit>,
			value: BoundedVec<u8, T::ValueLimit>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let collection =
				Collections::<T>::get(&collection_id).ok_or(Error::<T>::NoAvailableCollectionId)?;
			ensure!(collection.issuer == sender.unwrap_or_default(), Error::<T>::NoPermission);

			if let Some(nft_id) = &maybe_nft_id {
				ensure!(
					NFTs::<T>::contains_key(collection_id, nft_id),
					Error::<T>::NoAvailableNftId
				);
				if let Some(nft) = NFTs::<T>::get(collection_id, nft_id) {
					ensure!(nft.rootowner == collection.issuer, Error::<T>::NoPermission);
				}
			}
			Properties::<T>::insert((&collection_id, maybe_nft_id, &key), &value);

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
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let collection_id =
				<Self as Collection<StringLimitOf<T>, T::AccountId>>::lock_collection(
					collection_id,
				)?;

			Self::deposit_event(Event::CollectionLocked {
				issuer: sender.unwrap_or_default(),
				collection_id,
			});
			Ok(())
		}

		/// Create resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn add_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			base: Option<BoundedVec<u8, T::StringLimit>>,
			src: Option<BoundedVec<u8, T::StringLimit>>,
			metadata: Option<BoundedVec<u8, T::StringLimit>>,
			slot: Option<BoundedVec<u8, T::StringLimit>>,
			license: Option<BoundedVec<u8, T::StringLimit>>,
			thumb: Option<BoundedVec<u8, T::StringLimit>>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let mut pending = false;
			let nft = NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;
			if nft.rootowner != sender.unwrap_or_default() {
				pending = true;
			}

			let resource_id = Self::get_next_resource_id()?;
			ensure!(
				Resources::<T>::get((collection_id, nft_id, resource_id)).is_none(),
				Error::<T>::ResourceAlreadyExists
			);

			let empty = base.is_none()
				&& src.is_none() && metadata.is_none()
				&& slot.is_none()
				&& license.is_none()
				&& thumb.is_none();
			ensure!(!empty, Error::<T>::EmptyResource);

			let res = ResourceInfo {
				id: resource_id,
				base,
				src,
				metadata,
				slot,
				license,
				thumb,
				pending,
			};
			Resources::<T>::insert((collection_id, nft_id, resource_id), res);

			Self::deposit_event(Event::ResourceAdded { nft_id, resource_id });
			Ok(())
		}
		/// accept the addition of a new resource to an existing NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn accept(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let nft = NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;
			ensure!(nft.rootowner == sender.unwrap_or_default(), Error::<T>::NoPermission);

			Resources::<T>::try_mutate_exists(
				(collection_id, nft_id, resource_id),
				|resource| -> DispatchResult {
					if let Some(res) = resource.into_mut() {
						res.pending = false;
					}
					Ok(())
				},
			)?;

			Self::deposit_event(Event::ResourceAccepted { nft_id, resource_id });
			Ok(())
		}

		/// set a different order of resource priority
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_priority(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			priorities: Vec<Vec<u8>>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
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
}
