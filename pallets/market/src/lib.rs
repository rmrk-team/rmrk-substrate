#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use frame_support::{
	traits::Currency,
};


use rmrk_traits::{
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

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config + pallet_uniques::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type ProtocolOrigin: EnsureOrigin<Self::Origin>;

		// The market currency mechanism.
		type Currency: Currency<Self::AccountId>;

		// TODO: Weight values for this pallet
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// #[pallet::storage]
	// #[pallet::getter(fn listed_nfts)]
	// TODO: Stores listed NFT info
	// pub type ListedNfts<T: Config> = StorageMap<_, Twox64Concat, (CollectionId, NftId), ListingInfoOf<T>>;

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
		// TODO: Implement buy(origin, collection_id, nft_id)

		// TODO: Implement list(origin, collection_id, nft_id, amount: BalanceOf<T>)

		// TODO: Implement unlist(origin, collection_id, nft_id)

		// TODO: Implement make_offer(origin, collection_id, nft_id, amount: BalanceOf<T>, expires: T::BlockNumber)

		// TODO: withdraw_offer(origin, collection_id, nft_id)

	}
}
