#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use frame_support::{
	dispatch::DispatchResult, ensure, traits::{Currency, tokens::nonfungibles::*}, transactional, BoundedVec,
};
use frame_system::ensure_signed;

use sp_std::prelude::*;

pub use pallet::*;
pub use pallet_rmrk_core::types::*;

use rmrk_traits::{
	ListInfo,
	primitives::*,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	type ListInfoOf<T> =
		ListInfo <ListId, CollectionId, NftId, BalanceOf<T>>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type ProtocolOrigin: EnsureOrigin<Self::Origin>;

		/// The market currency mechanism.
		type Currency: Currency<Self::AccountId>;

		// TODO: Weight values for this pallet
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn listed_nfts)]
	/// Stores listed NFT info
	pub type Listings<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, ListId>,
		),
		ListInfoOf<T>,
		OptionQuery,
	>;

	// TODO: Storage for offers on an NFT. Need to create offer trait

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The price for a token was updated \[owner, collection_id, nft_id, price\]
		TokenPriceUpdated {
			owner: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
			price: Option<BalanceOf<T>>,
		},
		/// Token was sold to a new owner
		/// \[owner, buyer, collection_id, nft_id, price, author, royalty, royalty_amount\]
		TokenSold {
			owner: T::AccountId,
			buyer: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
			price: BalanceOf<T>,
			royalty: Option<(T::AccountId, u8)>,
			royalty_amount: Option<BalanceOf<T>>,
		},
		/// Token listed on Marketplace \[owner, collection_id, nft_id, author royalty\]
		TokenListed {
			owner: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
			price: BalanceOf<T>,
			royalty: Option<(T::AccountId, u8)>,
		},
		/// Token unlisted on Marketplace \[owner, collection_id, nft_id\]
		TokenUnlisted {
			owner: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
		},
		/// Offer was placed on a token \[offerer, collection_id, nft_id, price\]
		OfferPlaced {
			offerer: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
			price: BalanceOf<T>,
		},
		/// Offer was withdrawn \[sender, collection_id, nft_id\]
		OfferWithdrawn {
			sender: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
		},
		/// Offer was accepted \[owner, buyer, collection_id, nft_id\]
		OfferAccepted {
			owner: T::AccountId,
			buyer: T::AccountId,
			collection_id: CollectionId,
			nft_id: NftId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// No permissions for account to interact with NFT
		NoPermission,
		/// Token cannot be bought
		CannotBuyToken,
		/// Offer already accepted and cannot withdraw
		CannotWithdrawOffer,
		/// Cannot unlist NFT as it has already been sold
		CannotUnlistToken,
		/// Cannot make offer on NFT on own NFT
		CannotOfferOnOwnToken,
		/// Cannot buy NFT that is already owned
		CannotBuyOwnToken,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Buy a listed NFT. Ensure that the NFT is available for purchase and has not recently
		/// been purchased, sent, or burned.
		///
		/// Parameters:
		///	- `origin` - Account of the potential buyer
		///	- `collection_id` - Collection id of the RMRK NFT
		///	- `nft_id` - NFT id of the RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn buy_nft(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// TODO: Logic and create function to handle in functions.rs

			Ok(())
		}

		/// List a RMRK NFT on the Marketplace for purchase.
		///
		/// Parameters:
		///	- `origin` - Account of owner of the RMRK NFT to be listed
		///	- `collection_id` - Collection id of the RMRK NFT
		///	- `nft_id` - NFT id of the RMRK NFT
		/// - `amount` - Price of the RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn list(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// TODO: Logic and create function to handle in functions.rs

			Ok(())
		}

		/// Unlist a RMRK NFT on the Marketplace and remove from storage in `Listings`.
		///
		/// Parameters:
		/// - `origin` - Account owner of the listed RMRK NFT
		/// - `collection_id` - Collection id of the RMRK NFT
		/// - `nft_id` - NFT id of the RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn unlist(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// TODO: Logic and create function to handle in functions.rs

			Ok(())
		}

		/// Make an offer on a RMRK NFT for purchase. An offer can be set with an expiration where
		/// the offer can no longer be accepted by the RMRK NFT owner
		///
		/// Parameters:
		/// - `origin` - Account of the potential buyer
		/// - `collection_id` - Collection id of the RMRK NFT
		/// - `nft_id` - NFT id of the RMRK NFT
		/// - `amount` - Price of the RMRK NFT
		/// - `expiration` - Expiration of the offer
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn make_offer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			amount: BalanceOf<T>,
			expiration: Option<T::BlockNumber>,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// TODO: Logic and create function to handle in functions.rs

			Ok(())
		}

		/// Withdraw an offer on a RMRK NFT, such that it is no longer available to be accepted by
		/// the NFT owner
		///
		/// Parameters:
		/// - `origin` - Account that wants to withdraw their offer
		/// - `collection_id` - Collection id of the RMRK NFT
		/// - `nft_id` - NFT id of the RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn withdraw_offer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			// TODO: Logic and create function to handle in functions.rs

			Ok(())
		}

		// TODO: Accept an offer on a RMRK NFT from a potential buyer.
		//
		// Parameters:
		// - `origin` - Account of the potential buyer
		// - `collection_id` - Collection id of the RMRK NFT
		// - `nft_id` - NFT id of the RMRK NFT
		// - `offer_id` - Offer id of the offer to be accepted
	}
}
