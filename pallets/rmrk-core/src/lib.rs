#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use codec::HasCompact;
use frame_support::{ensure, transactional, BoundedVec};
use frame_system::ensure_signed;

use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, One, StaticLookup, Zero};
use sp_std::{convert::TryInto, vec::Vec};

use types::{ClassInfo, InstanceInfo};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as
// frame_system::Config>::AccountId>>::Balance;
pub type ClassInfoOf<T> = ClassInfo<BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>>;
pub type InstanceInfoOf<T> = InstanceInfo<
	<T as frame_system::Config>::AccountId,
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
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

		type CollectionId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ AtLeast32BitUnsigned
			+ Into<Self::ClassId>;

		type ProtocolOrigin: EnsureOrigin<Self::Origin>;

		type NftId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ AtLeast32BitUnsigned
			+ From<Self::InstanceId>
			+ Into<Self::InstanceId>;

		type ResourceId: Member + Parameter + Default + Copy + HasCompact + AtLeast32BitUnsigned;
	}

	/// Next available collection ID.
	#[pallet::storage]
	#[pallet::getter(fn next_collection_id)]
	pub type NextCollectionId<T: Config> = StorageValue<_, T::CollectionId, ValueQuery>;

	/// Next available NFT ID.
	#[pallet::storage]
	#[pallet::getter(fn next_nft_id)]
	pub type NextNftId<T: Config> =
		StorageMap<_, Twox64Concat, T::CollectionId, T::NftId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collections)]
	/// Stores collections info
	pub type Collections<T: Config> = StorageMap<_, Twox64Concat, T::CollectionId, ClassInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	/// Stores nft info
	pub type NFTs<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CollectionId,
		Twox64Concat,
		T::NftId,
		InstanceInfoOf<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn resources)]
	/// Stores resource info
	pub type Resources<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::NftId, Twox64Concat, T::ResourceId, InstanceInfoOf<T>>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectionCreated(T::AccountId, T::CollectionId),
		NftMinted(T::AccountId, T::CollectionId, T::NftId),
		NFTBurned(T::AccountId, T::NftId),
		CollectionBurned(T::AccountId, T::CollectionId),
		NFTSent(T::AccountId, T::AccountId, T::CollectionId, T::NftId),
		IssuerChanged(T::AccountId, T::AccountId, T::CollectionId),
		PropertySet(
			T::CollectionId,
			Option<T::NftId>,
			BoundedVec<u8, T::KeyLimit>,
			BoundedVec<u8, T::ValueLimit>,
		),
		CollectionLocked(T::AccountId, T::CollectionId),
		ResourceAdded(T::NftId, T::ResourceId),
		ResourceAccepted(T::NftId, T::ResourceId),
		PrioritySet(T::CollectionId, T::NftId),
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
		AuthorNotSet,
		NoAvailableNftId,
		NotInRange,
		RoyaltyNotSet,
		CollectionUnknown
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mints an NFT in the specified collection
		/// Sets metadata and the royalty attribute
		///
		/// Parameters:
		/// - `collection_id`: The class of the asset to be minted.
		/// - `nft_id`: The nft value of the asset to be minted.
		/// - `author`: Receiver of the royalty
		/// - `royalty`: Percentage reward from each trade for the author
		/// - `metadata`: Arbitrary data about an nft, e.g. IPFS hash
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn mint_nft(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			author: Option<T::AccountId>,
			royalty: Option<u8>,
			metadata: Option<Vec<u8>>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let _ = Self::collections(collection_id)
				.ok_or(Error::<T>::CollectionUnknown)?;

			if let Some(r) = royalty {
				ensure!(r < 100, Error::<T>::NotInRange);
			}

			let nft_id: T::NftId = NextNftId::<T>::try_mutate(
				collection_id,
				|id| -> Result<T::NftId, DispatchError> {
					let current_id = *id;
					*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableNftId)?;
					Ok(current_id)
				},
			)?;

			pallet_uniques::Pallet::<T>::do_mint(
				collection_id.into(),
				nft_id.into(),
				sender.clone().unwrap_or_default(),
				|_details| Ok(()),
			)?;

			let metadata_bounded =
				Self::to_bounded_string(metadata.ok_or(Error::<T>::MetadataNotSet)?)?;
			let author = author.ok_or(Error::<T>::AuthorNotSet)?;
			let royalty = royalty.ok_or(Error::<T>::RoyaltyNotSet)?;

			NFTs::<T>::insert(
				collection_id,
				nft_id,
				InstanceInfo { author, royalty, metadata: metadata_bounded },
			);

			Self::deposit_event(Event::NftMinted(
				sender.unwrap_or_default(),
				collection_id,
				nft_id,
			));

			Ok(())
		}

		/// Mint a collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn create_collection(origin: OriginFor<T>, metadata: Vec<u8>) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let collection_id = NextCollectionId::<T>::try_mutate(
				|id| -> Result<T::CollectionId, DispatchError> {
					let current_id = *id;
					*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableCollectionId)?;
					Ok(current_id)
				},
			)?;

			let metadata_bounded = Self::to_bounded_string(metadata)?;

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
			)?;

			Collections::<T>::insert(collection_id, ClassInfo { metadata: metadata_bounded });

			Self::deposit_event(Event::CollectionCreated(
				sender.unwrap_or_default(),
				collection_id,
			));
			Ok(())
		}

		/// burn nft
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn burn_nft(origin: OriginFor<T>, nft_id: T::NftId) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			// TODO
			// pallet_uniques::Pallet::<T>::burn
			Self::deposit_event(Event::NFTBurned(sender.unwrap_or_default(), nft_id));
			Ok(())
		}

		/// burn collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn burn_collection(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			// TODO
			Self::deposit_event(Event::CollectionBurned(sender.unwrap_or_default(), collection_id));
			Ok(())
		}

		/// transfer NFT from account A to account B
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn send(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::NftId,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			let dest = T::Lookup::lookup(dest)?;
			// TODO
			Self::deposit_event(Event::NFTSent(
				sender.unwrap_or_default(),
				dest,
				collection_id,
				nft_id,
			));
			Ok(())
		}

		/// changing the issuer of a collection or a base
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn change_issuer(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			let dest = T::Lookup::lookup(dest)?;
			// TODO
			Self::deposit_event(Event::IssuerChanged(
				sender.unwrap_or_default(),
				dest,
				collection_id,
			));
			Ok(())
		}

		/// set a custom value on an NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_property(
			origin: OriginFor<T>,
			#[pallet::compact] collection_id: T::CollectionId,
			maybe_nft_id: Option<T::NftId>,
			key: BoundedVec<u8, T::KeyLimit>,
			value: BoundedVec<u8, T::ValueLimit>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			// TODO
			Self::deposit_event(Event::PropertySet(collection_id, maybe_nft_id, key, value));
			Ok(())
		}

		/// lock collection
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn lock_collection(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			// TODO
			Self::deposit_event(Event::CollectionLocked(sender.unwrap_or_default(), collection_id));
			Ok(())
		}

		/// add resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn add_resource(
			origin: OriginFor<T>,
			nft_id: T::NftId,
			resource_id: T::ResourceId,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			// TODO, add resource_id
			Self::deposit_event(Event::ResourceAdded(nft_id, resource_id));
			Ok(())
		}

		/// accept the addition of a new resource to an existing NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn accept(
			origin: OriginFor<T>,
			nft_id: T::NftId,
			resource_id: T::ResourceId,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			Self::deposit_event(Event::ResourceAccepted(nft_id, resource_id));
			Ok(())
		}

		/// set a different order of resource priority
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_priority(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::NftId,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};
			Self::deposit_event(Event::PrioritySet(collection_id, nft_id));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn to_bounded_string(name: Vec<u8>) -> Result<BoundedVec<u8, T::StringLimit>, Error<T>> {
			name.try_into().map_err(|_| Error::<T>::TooLong)
		}
	}
}
