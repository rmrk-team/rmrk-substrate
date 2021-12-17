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
	traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd, One, StaticLookup, Zero},
	DispatchError, Permill,
};
use sp_std::{convert::TryInto, vec, vec::Vec};

use types::{AccountIdOrCollectionNftTuple, ClassInfo, InstanceInfo, ResourceInfo};

use rmrk_traits::{primitives::*, Collection, CollectionInfo};
use sp_std::result::Result;

mod functions;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as
// frame_system::Config>::AccountId>>::Balance;

pub type InstanceInfoOf<T> = InstanceInfo<
	<T as frame_system::Config>::AccountId,
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
	CollectionId,
	<T as pallet::Config>::NftId,
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

		type NftId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ AtLeast32BitUnsigned
			+ From<Self::InstanceId>
			+ Into<Self::InstanceId>;

		type MaxRecursions: Get<u32>;
	}

	/// Next available NFT ID.
	#[pallet::storage]
	#[pallet::getter(fn next_nft_id)]
	pub type NextNftId<T: Config> = StorageMap<_, Twox64Concat, CollectionId, T::NftId, ValueQuery>;

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
		StorageMap<_, Twox64Concat, T::AccountId, Vec<(CollectionId, T::NftId)>>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	/// Stores nft info
	pub type NFTs<T: Config> =
		StorageDoubleMap<_, Twox64Concat, CollectionId, Twox64Concat, T::NftId, InstanceInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn priorities)]
	/// Stores priority info
	pub type Priorities<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CollectionId,
		Twox64Concat,
		T::NftId,
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
		T::NftId,
		Vec<(CollectionId, T::NftId)>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn resources)]
	/// Stores resource info
	pub type Resources<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, T::NftId>,
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
			NMapKey<Blake2_128Concat, Option<T::NftId>>,
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
		CollectionCreated(T::AccountId, CollectionId),
		NftMinted(T::AccountId, CollectionId, T::NftId),
		NFTBurned(T::AccountId, T::NftId),
		CollectionDestroyed(T::AccountId, CollectionId),
		NFTSent(
			T::AccountId,
			AccountIdOrCollectionNftTuple<T::AccountId, CollectionId, T::NftId>,
			CollectionId,
			T::NftId,
		),
		IssuerChanged(T::AccountId, T::AccountId, CollectionId),
		PropertySet(
			CollectionId,
			Option<T::NftId>,
			BoundedVec<u8, T::KeyLimit>,
			BoundedVec<u8, T::ValueLimit>,
		),
		CollectionLocked(T::AccountId, CollectionId),
		ResourceAdded(T::NftId, ResourceId),
		ResourceAccepted(T::NftId, ResourceId),
		PrioritySet(CollectionId, T::NftId),
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
	impl<T: Config> Pallet<T> {
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
			recipient: T::AccountId,
			royalty: Permill,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let collection =
				Self::collections(collection_id).ok_or(Error::<T>::CollectionUnknown)?;

			let nfts_minted = NFTs::<T>::iter_prefix_values(collection_id).count();
			let max: u32 = collection.max.try_into().unwrap();

			ensure!(
				// Probably a better way to do "max == 0"
				nfts_minted < max.try_into().unwrap() || max == max - max,
				Error::<T>::CollectionFullOrLocked
			);

			let nft_id: T::NftId = Self::get_next_nft_id(collection_id)?;

			// let metadata_bounded = Self::to_bounded_string(metadata)?;
			// if let Some(r) = royalty {
			// 	ensure!(r < 1000, Error::<T>::NotInRange);
			// }

			// pallet_uniques::Pallet::<T>::do_mint(
			// 	collection_id.into(),
			// 	nft_id.into(),
			// 	sender.clone().unwrap_or_default(),
			// 	|_details| Ok(()),
			// )?;

			let rootowner = owner.clone();
			let owner_as_maybe_account = AccountIdOrCollectionNftTuple::AccountId(owner.clone());

			let nft = InstanceInfo {
				owner: owner_as_maybe_account,
				rootowner,
				recipient,
				royalty,
				metadata,
			};

			NFTs::<T>::insert(collection_id, nft_id, nft);
			NftsByOwner::<T>::append(owner, (collection_id, nft_id));

			Self::deposit_event(Event::NftMinted(
				sender.unwrap_or_default(),
				collection_id,
				nft_id,
			));

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

			// let metadata_bounded = Self::to_bounded_string(metadata)?;
			// let symbol_bounded = Self::to_bounded_string(symbol)?;
			// let id_bounded = Self::to_bounded_string(id)?;
			// let collection_id = Self::get_next_collection_id()?;

			let max = max.unwrap_or_default();

			let collection_id =
				<Self as Collection<StringLimitOf<T>, T::AccountId>>::create_collection(
					sender.clone().unwrap_or_default(),
					metadata,
					max,
					symbol,
				)?;

			// Collections::<T>::insert(
			// 	collection_id,
			// 	ClassInfo {
			// 		issuer: sender.clone().unwrap_or_default(),
			// 		metadata,
			// 		max,
			// 		symbol,
			// 	},
			// );

			Self::deposit_event(Event::CollectionCreated(
				sender.clone().unwrap_or_default(),
				collection_id,
			));
			Ok(())
		}

		/// burn nft
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn burn_nft(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: T::NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// NFTs::<T>::remove(collection_id, nft_id);
			// if let Some(kids) = Children::<T>::take(collection_id, nft_id) {
			// 	for child in kids {
			// 		Pallet::<T>::burn_nft(origin.clone(), child.0, child.1)?;
			// 	}
			// }

			// pallet_uniques::Pallet::<T>::do_burn(collection_id.into(), nft_id.into(), |_, _| {
			// 	Ok(())
			// })?;

			Pallet::<T>::recursive_burn(collection_id, nft_id, T::MaxRecursions::get())?;

			Self::deposit_event(Event::NFTBurned(sender, nft_id));
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

			Self::deposit_event(Event::CollectionDestroyed(
				sender.unwrap_or_default(),
				collection_id,
			));
			Ok(())
		}

		/// transfer NFT from account A to (account B or NFT)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn send(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: T::NftId,
			new_owner: AccountIdOrCollectionNftTuple<T::AccountId, CollectionId, T::NftId>,
		) -> DispatchResult {
			let sender = match T::ProtocolOrigin::try_origin(origin) {
				Ok(_) => None,
				Err(origin) => Some(ensure_signed(origin)?),
			};

			let mut sending_nft =
				NFTs::<T>::get(collection_id, nft_id).ok_or(Error::<T>::NoAvailableNftId)?;
			ensure!(
				sending_nft.rootowner == sender.clone().unwrap_or_default(),
				Error::<T>::NoPermission
			);

			match new_owner.clone() {
				AccountIdOrCollectionNftTuple::AccountId(account_id) => {
					// Remove previous parental relationship
					if let AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) =
						sending_nft.owner
					{
						if let Some(mut kids) = Children::<T>::take(cid, nid) {
							kids.retain(|&kid| kid != (collection_id, nft_id));
							Children::<T>::insert(cid, nid, kids);
						}
					}
					sending_nft.rootowner = account_id.clone();

					// Pallet::<T>::recursive_update_rootowner(
					// 	collection_id,
					// 	nft_id,
					// 	account_id.clone(),
					// 	T::MaxRecursions::get(),
					// )?;
				},
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) => {
					let recipient_nft =
						NFTs::<T>::get(cid, nid).ok_or(Error::<T>::NoAvailableNftId)?;
					// Check if sending NFT is already a child of recipient NFT
					ensure!(
						!Pallet::<T>::is_x_descendent_of_y(cid, nid, collection_id, nft_id),
						Error::<T>::CannotSendToDescendent
					);

					// Remove parent if exists: first we only care if the owner is a non-AccountId)
					if let AccountIdOrCollectionNftTuple::CollectionAndNftTuple(cid, nid) =
						sending_nft.owner
					{
						// second we only care if the parent has children (it should)
						if let Some(mut kids) = Children::<T>::take(cid, nid) {
							// third we only "retain" the other children
							kids.retain(|&kid| kid != (collection_id, nft_id));
							Children::<T>::insert(cid, nid, kids);
						}
					}
					if sending_nft.rootowner != recipient_nft.rootowner {
						// sending_nft.rootowner = recipient_nft.rootowner
						sending_nft.rootowner = recipient_nft.rootowner.clone();

						Pallet::<T>::recursive_update_rootowner(
							collection_id,
							nft_id,
							recipient_nft.rootowner,
							T::MaxRecursions::get(),
						)?;
					}
					match Children::<T>::take(cid, nid) {
						None => Children::<T>::insert(cid, nid, vec![(collection_id, nft_id)]),
						Some(mut kids) => {
							kids.push((collection_id, nft_id));
							Children::<T>::insert(cid, nid, kids);
						},
					}
				},
			};
			sending_nft.owner = new_owner.clone();

			NFTs::<T>::remove(collection_id, nft_id);
			NFTs::<T>::insert(collection_id, nft_id, sending_nft);

			Self::deposit_event(Event::NFTSent(
				sender.unwrap_or_default(),
				new_owner,
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

			Self::deposit_event(Event::IssuerChanged(
				sender.unwrap_or_default(),
				new_issuer,
				collection_id,
			));
			Ok(())
		}

		/// set a custom value on an NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_property(
			origin: OriginFor<T>,
			#[pallet::compact] collection_id: CollectionId,
			maybe_nft_id: Option<T::NftId>,
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

			Self::deposit_event(Event::PropertySet(collection_id, maybe_nft_id, key, value));
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

			Self::deposit_event(Event::CollectionLocked(sender.unwrap_or_default(), collection_id));
			Ok(())
		}

		/// Create resource
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn add_resource(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: T::NftId,
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

			let empty = base.is_none() &&
				src.is_none() && metadata.is_none() &&
				slot.is_none() && license.is_none() &&
				thumb.is_none();
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

			Self::deposit_event(Event::ResourceAdded(nft_id, resource_id));
			Ok(())
		}
		/// accept the addition of a new resource to an existing NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn accept(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: T::NftId,
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

			Self::deposit_event(Event::ResourceAccepted(nft_id, resource_id));
			Ok(())
		}

		/// set a different order of resource priority
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn set_priority(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: T::NftId,
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
			Self::deposit_event(Event::PrioritySet(collection_id, nft_id));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn to_bounded_string(
			name: Vec<u8>,
		) -> Result<BoundedVec<u8, T::StringLimit>, Error<T>> {
			name.try_into().map_err(|_| Error::<T>::TooLong)
		}
		pub fn to_optional_bounded_string(
			name: Option<Vec<u8>>,
		) -> Result<Option<BoundedVec<u8, T::StringLimit>>, Error<T>> {
			match name {
				Some(n) => {
					let bounded_string = Self::to_bounded_string(n)?;
					return Ok(Some(bounded_string))
				},
				None => return Ok(None),
			}
		}

		pub fn get_next_nft_id(collection_id: CollectionId) -> Result<T::NftId, Error<T>> {
			NextNftId::<T>::try_mutate(collection_id, |id| {
				let current_id = *id;
				*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableNftId)?;
				Ok(current_id)
			})
		}
		pub fn get_next_resource_id() -> Result<ResourceId, Error<T>> {
			NextResourceId::<T>::try_mutate(|id| {
				let current_id = *id;
				*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableCollectionId)?;
				Ok(current_id)
			})
		}
	}
}

impl<T: Config> Collection<StringLimitOf<T>, T::AccountId> for Pallet<T> {
	fn issuer(collection_id: CollectionId) -> Option<T::AccountId> {
		None
	}
	fn create_collection(
		issuer: T::AccountId,
		metadata: StringLimitOf<T>,
		max: u32,
		symbol: StringLimitOf<T>,
	) -> Result<CollectionId, DispatchError> {
		let collection = CollectionInfo { issuer: issuer.clone(), metadata, max, symbol };
		let collection_id =
			<CollectionIndex<T>>::try_mutate(|n| -> Result<CollectionId, DispatchError> {
				let id = *n;
				ensure!(id != CollectionId::max_value(), Error::<T>::NoAvailableCollectionId);
				*n += 1;
				Ok(id)
			})?;
		Collections::<T>::insert(collection_id, collection);
		Ok(collection_id)
	}

	fn burn_collection(issuer: T::AccountId, collection_id: CollectionId) -> DispatchResult {
		ensure!(
			NFTs::<T>::iter_prefix_values(collection_id).count() == 0,
			Error::<T>::CollectionNotEmpty
		);
		Collections::<T>::remove(collection_id);
		Ok(())
	}

	fn change_issuer(
		collection_id: CollectionId,
		new_issuer: T::AccountId,
	) -> Result<(T::AccountId, CollectionId), DispatchError> {
		ensure!(Collections::<T>::contains_key(collection_id), Error::<T>::NoAvailableCollectionId);

		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			if let Some(col) = collection {
				col.issuer = new_issuer.clone();
			}
			Ok(())
		})?;

		Ok((new_issuer, collection_id))
	}

	fn lock_collection(collection_id: CollectionId) -> Result<CollectionId, DispatchError> {
		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			let collection = collection.as_mut().ok_or(Error::<T>::CollectionUnknown)?;
			let currently_minted = NFTs::<T>::iter_prefix_values(collection_id).count();
			collection.max = currently_minted.try_into().unwrap();
			Ok(())
		})?;
		Ok(collection_id)
	}
}
