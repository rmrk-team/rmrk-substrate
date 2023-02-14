// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-market.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
	transactional, BoundedVec,
};
use frame_system::{ensure_signed, RawOrigin};
use sp_runtime::{Permill, Saturating};

use sp_std::prelude::*;

pub use pallet::*;

pub mod weights;
pub use weights::WeightInfo;

use rmrk_traits::{AccountIdOrCollectionNftTuple, NftInfo, RoyaltyInfo};

pub mod types;

#[cfg(any(feature = "runtime-benchmarks"))]
pub mod benchmarking;

#[cfg(feature = "runtime-benchmarks")]
use pallet_rmrk_core::BenchmarkHelper;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use crate::types::{MarketplaceHooks, Offer};
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::types::{ListInfo, MarketplaceHooks};
	use frame_support::{pallet_prelude::*, traits::tokens::nonfungibles::Inspect};
	use frame_system::pallet_prelude::*;
	use sp_runtime::Permill;

	pub type InstanceInfoOf<T> = NftInfo<
		<T as frame_system::Config>::AccountId,
		Permill,
		BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
		<T as pallet_uniques::Config>::CollectionId,
		<T as pallet_uniques::Config>::ItemId,
	>;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub type ListInfoOf<T> = ListInfo<
		<T as frame_system::Config>::AccountId,
		BalanceOf<T>,
		<T as frame_system::Config>::BlockNumber,
	>;

	pub type OfferOf<T> = Offer<
		<T as frame_system::Config>::AccountId,
		BalanceOf<T>,
		<T as frame_system::Config>::BlockNumber,
	>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type ProtocolOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The market currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Minimum offer amount as a valid offer
		#[pallet::constant]
		type MinimumOfferAmount: Get<BalanceOf<Self>>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: BenchmarkHelper<Self::CollectionId, Self::ItemId>;

		/// Marketplace hooks to be implemented downstream.
		type MarketplaceHooks: MarketplaceHooks<BalanceOf<Self>, Self::CollectionId, Self::ItemId>;

		/// Market fee to be implemented downstream.
		#[pallet::constant]
		type MarketFee: Get<Permill>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn listed_nfts)]
	/// Stores listed NFT price info
	pub type ListedNfts<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectionId,
		Blake2_128Concat,
		T::ItemId,
		ListInfoOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers)]
	/// Stores offer on a NFT info
	pub type Offers<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(T::CollectionId, T::ItemId),
		Blake2_128Concat,
		T::AccountId,
		OfferOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn marketplace_owner)]
	/// Stores the marketplace owner account
	pub type MarketplaceOwner<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The price for a token was updated
		TokenPriceUpdated {
			owner: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			price: Option<BalanceOf<T>>,
		},
		/// Token was sold to a new owner
		TokenSold {
			owner: T::AccountId,
			buyer: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			price: BalanceOf<T>,
		},
		/// Token listed on Marketplace
		TokenListed {
			owner: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			price: BalanceOf<T>,
		},
		/// Token unlisted on Marketplace
		TokenUnlisted { owner: T::AccountId, collection_id: T::CollectionId, nft_id: T::ItemId },
		/// Offer was placed on a token
		OfferPlaced {
			offerer: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			price: BalanceOf<T>,
		},
		/// Offer was withdrawn
		OfferWithdrawn { sender: T::AccountId, collection_id: T::CollectionId, nft_id: T::ItemId },
		/// Offer was accepted
		OfferAccepted {
			owner: T::AccountId,
			buyer: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		},
		/// Royalty fee paid to royalty owner
		RoyaltyFeePaid {
			sender: T::AccountId,
			royalty_owner: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			amount: BalanceOf<T>,
		},
		/// Market fee paid to marketplace owner
		MarketFeePaid {
			sender: T::AccountId,
			marketplace_owner: T::AccountId,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			amount: BalanceOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// No permissions for account to interact with NFT
		NoPermission,
		/// Token cannot be bought
		TokenNotForSale,
		/// Offer already accepted and cannot withdraw
		CannotWithdrawOffer,
		/// Cannot unlist NFT as it has already been unlisted or sold
		CannotUnlistToken,
		/// Cannot make offer on NFT on own NFT
		CannotOfferOnOwnToken,
		/// Cannot buy NFT that is already owned
		CannotBuyOwnToken,
		/// Offer is unknown
		UnknownOffer,
		/// Cannot list NFT owned by a NFT
		CannotListNftOwnedByNft,
		/// Cannot list a non-existing NFT
		TokenDoesNotExist,
		/// Offer is below the OfferMinimumAmount threshold
		OfferTooLow,
		/// Account cannot offer on a NFT again with an active offer
		AlreadyOffered,
		/// Accepted offer has expired and cannot be accepted
		OfferHasExpired,
		/// Listing has expired and cannot be bought
		ListingHasExpired,
		/// Price differs from when `buy` was executed
		PriceDiffersFromExpected,
		/// Not possible to list non-transferable NFT
		NonTransferable,
		/// Marketplace owner not configured
		MarketplaceOwnerNotSet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Buy a listed NFT. Ensure that the NFT is available for purchase and has not recently
		/// been purchased, sent, or burned.
		///
		/// Parameters:
		/// 	- `origin` - Account of the potential buyer
		/// 	- `collection_id` - Collection id of the RMRK NFT
		/// 	- `nft_id` - NFT id of the RMRK NFT
		/// 	- `amount` - Optional price at which buyer purchased at
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::buy())]
		#[transactional]
		pub fn buy(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			amount: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::do_buy(sender, collection_id, nft_id, amount, false)
		}

		/// List a RMRK NFT on the Marketplace for purchase. A listing can be cancelled, and is
		/// automatically considered cancelled when a `buy` is executed on top of a given listing.
		/// An NFT that has another NFT as its owner CANNOT be listed. An NFT owned by a NFT must
		/// first be sent to an account before being listed.
		///
		/// Parameters:
		/// 	- `origin` - Account of owner of the RMRK NFT to be listed
		/// 	- `collection_id` - Collection id of the RMRK NFT
		/// 	- `nft_id` - NFT id of the RMRK NFT
		/// 	- `amount` - Price of the RMRK NFT
		/// 	- `expires` - Optional BlockNumber for when the listing expires
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::list())]
		#[transactional]
		pub fn list(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			amount: BalanceOf<T>,
			expires: Option<T::BlockNumber>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id)
				.ok_or(Error::<T>::TokenDoesNotExist)?;

			// Ensure that the NFT is not owned by an NFT
			ensure!(
				!Self::is_nft_owned_by_nft(collection_id, nft_id),
				Error::<T>::CannotListNftOwnedByNft
			);
			// Ensure sender is the owner
			ensure!(sender == owner, Error::<T>::NoPermission);

			let nft = pallet_rmrk_core::Pallet::<T>::nfts(collection_id, nft_id)
				.ok_or(Error::<T>::TokenDoesNotExist)?;

			// Check NFT is transferable
			pallet_rmrk_core::Pallet::<T>::check_is_transferable(&nft)?;

			// Check if NFT is frozen
			ensure!(
				pallet_uniques::Pallet::<T>::can_transfer(&collection_id, &nft_id),
				pallet_uniques::Error::<T>::Frozen
			);

			// Lock NFT to prevent transfers or interactions with the NFT
			pallet_rmrk_core::Pallet::<T>::set_lock((collection_id, nft_id), true);

			// Add new ListInfo with listed_by, amount, Option<BlockNumber>
			ListedNfts::<T>::insert(
				collection_id,
				nft_id,
				ListInfo { listed_by: sender, amount, expires },
			);

			Self::deposit_event(Event::TokenListed { owner, collection_id, nft_id, price: amount });

			Ok(())
		}

		/// Unlist a RMRK NFT on the Marketplace and remove from storage in `Listings`.
		///
		/// Parameters:
		/// - `origin` - Account owner of the listed RMRK NFT
		/// - `collection_id` - Collection id of the RMRK NFT
		/// - `nft_id` - NFT id of the RMRK NFT
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unlist())]
		#[transactional]
		pub fn unlist(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Check if NFT is still in ListedNfts storage
			ensure!(Self::is_nft_listed(collection_id, nft_id), Error::<T>::CannotUnlistToken);
			let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id)
				.ok_or(Error::<T>::TokenDoesNotExist)?;
			// Ensure owner of NFT is performing call to unlist
			ensure!(sender == owner, Error::<T>::NoPermission);
			// Set the NFT lock to false to allow interactions with the NFT
			pallet_rmrk_core::Pallet::<T>::set_lock((collection_id, nft_id), false);
			// Remove from storage
			ListedNfts::<T>::remove(collection_id, nft_id);
			// Emit TokenUnlisted Event
			Self::deposit_event(Event::TokenUnlisted { owner, collection_id, nft_id });

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
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::make_offer())]
		#[transactional]
		pub fn make_offer(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			amount: BalanceOf<T>,
			expires: Option<T::BlockNumber>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Ensure amount is above the minimum threshold
			ensure!(amount >= T::MinimumOfferAmount::get(), Error::<T>::OfferTooLow);
			// Ensure NFT exists & sender is not owner
			let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id)
				.ok_or(Error::<T>::TokenDoesNotExist)?;

			ensure!(sender != owner, Error::<T>::CannotOfferOnOwnToken);
			// If offer has already been made, must withdraw_offer first before making a new offer
			ensure!(
				!Self::has_active_offer(collection_id, nft_id, sender.clone()),
				Error::<T>::AlreadyOffered
			);

			// Reserve currency from offerer account
			<T as pallet::Config>::Currency::reserve(&sender, amount)?;

			let token_id = (collection_id, nft_id);
			// Insert new offer into Offers storage
			Offers::<T>::insert(
				token_id,
				sender.clone(),
				Offer { maker: sender.clone(), amount, expires },
			);

			// Emit OfferPlaced event
			Self::deposit_event(Event::OfferPlaced {
				offerer: sender,
				collection_id,
				nft_id,
				price: amount,
			});

			Ok(())
		}

		/// Withdraw an offer on a RMRK NFT, such that it is no longer available to be accepted by
		/// the NFT owner
		///
		/// Parameters:
		/// - `origin` - Account that wants to withdraw their offer
		/// - `collection_id` - Collection id of the RMRK NFT
		/// - `nft_id` - NFT id of the RMRK NFT
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::withdraw_offer())]
		#[transactional]
		pub fn withdraw_offer(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let token_id = (collection_id, nft_id);
			// Ensure that offer exists from sender that is withdrawing their offer
			Offers::<T>::try_mutate_exists(
				token_id,
				sender.clone(),
				|maybe_offer| -> DispatchResult {
					let offer = maybe_offer.take().ok_or(Error::<T>::UnknownOffer)?;
					// Ensure NFT exists & sender is not owner
					let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id)
						.ok_or(Error::<T>::TokenDoesNotExist)?;
					// Cannot withdraw offer on own token
					ensure!(
						sender == owner || sender == offer.maker,
						Error::<T>::CannotWithdrawOffer
					);

					// Unreserve currency from offerer account
					<T as pallet::Config>::Currency::unreserve(&offer.maker, offer.amount);
					// Emit OfferWithdrawn Event
					Self::deposit_event(Event::OfferWithdrawn { sender, collection_id, nft_id });

					Ok(())
				},
			)
		}

		/// Accept an offer on a RMRK NFT from a potential buyer.
		///
		/// Parameters:
		/// - `origin` - Account of the current owner that is accepting the offerer's offer
		/// - `collection_id` - Collection id of the RMRK NFT
		/// - `nft_id` - NFT id of the RMRK NFT
		/// - `offerer` - Account that made the offer
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::accept_offer())]
		#[transactional]
		pub fn accept_offer(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			nft_id: T::ItemId,
			offerer: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Ensure NFT exists & sender is not owner
			let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id)
				.ok_or(Error::<T>::TokenDoesNotExist)?;
			// Cannot accept offer if not the owner
			ensure!(sender == owner, Error::<T>::NoPermission);

			let token_id = (collection_id, nft_id);
			Offers::<T>::try_mutate_exists(
				token_id,
				offerer.clone(),
				|maybe_offer| -> DispatchResult {
					let offer = maybe_offer.take().ok_or(Error::<T>::UnknownOffer)?;

					if let Some(expires) = offer.expires {
						if expires <= <frame_system::Pallet<T>>::block_number() {
							return Err(Error::<T>::OfferHasExpired.into())
						}
					}

					<T as pallet::Config>::Currency::unreserve(&offer.maker, offer.amount);
					Self::do_buy(offer.maker, collection_id, nft_id, None, true)?;
					// Emit OfferAccepted event
					Self::deposit_event(Event::OfferAccepted {
						owner,
						buyer: offerer,
						collection_id,
						nft_id,
					});

					Ok(())
				},
			)
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Buy the NFT helper funciton logic to handle both transactional calls of `buy` and
	/// `accept_offer`
	///
	/// Parameters:
	/// - `buyer`: The account that is buying the RMRK NFT
	/// - `collection_id`: The collection id of the RMRK NFT
	/// - `nft_id`: The id of the RMRK NFT
	/// - `amount`: Optional amount at which the buyer purchased a RMRK NFT
	/// - `is_offer`: Whether the call is from `accept_offer` or `buy`
	fn do_buy(
		buyer: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		amount: Option<BalanceOf<T>>,
		is_offer: bool,
	) -> DispatchResult {
		// Ensure buyer is not the root owner
		let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id)
			.ok_or(Error::<T>::TokenDoesNotExist)?;
		ensure!(buyer != owner, Error::<T>::CannotBuyOwnToken);

		let owner_origin = T::RuntimeOrigin::from(RawOrigin::Signed(owner.clone()));
		let token_id = (collection_id, nft_id);

		let list_price = if is_offer {
			Offers::<T>::get(token_id, buyer.clone())
				.map(|o| o.amount)
				.ok_or(Error::<T>::UnknownOffer)?
		} else {
			let list_info =
				ListedNfts::<T>::take(collection_id, nft_id).ok_or(Error::<T>::TokenNotForSale)?;
			// Ensure that the current owner is the one that listed the NFT
			ensure!(list_info.listed_by == owner, Error::<T>::TokenNotForSale);
			// Ensure the listing has not expired if Some(expires)
			// if None then there is no expiration
			if let Some(expires) = list_info.expires {
				ensure!(
					expires > <frame_system::Pallet<T>>::block_number(),
					Error::<T>::ListingHasExpired
				);
			}
			list_info.amount
		};

		// Check if list_price is equal to amount to prevent front running a buy
		if let Some(amount) = amount {
			ensure!(list_price == amount, Error::<T>::PriceDiffersFromExpected);
		}

		// Get NFT info for RoyaltyInfo and get the market fee constant
		let nft_info = pallet_rmrk_core::Nfts::<T>::get(collection_id, nft_id)
			.ok_or(pallet_rmrk_core::Error::<T>::NoAvailableNftId)?;
		let royalty_info = nft_info.royalty;
		let market_fee = T::MarketFee::get();

		// Set NFT Lock status to false to facilitate the purchase
		pallet_rmrk_core::Pallet::<T>::set_lock((collection_id, nft_id), false);

		// Calculate and finalize transfer of fees and payment then transfer the NFT
		Self::calculate_and_finalize_purchase_and_fees(
			buyer.clone(),
			owner.clone(),
			collection_id,
			nft_id,
			list_price,
			market_fee,
			royalty_info,
		)?;

		let new_owner = AccountIdOrCollectionNftTuple::AccountId(buyer.clone());
		pallet_rmrk_core::Pallet::<T>::send(owner_origin, collection_id, nft_id, new_owner)?;

		Self::deposit_event(Event::TokenSold {
			owner,
			buyer,
			collection_id,
			nft_id,
			price: list_price,
		});

		Ok(())
	}

	/// Helper function to check if a RMRK NFT is listed
	///
	/// Parameters:
	/// - collection_id: The collection id of the RMRK NFT
	/// - nft_id: The nft id of the RMRK NFT
	fn is_nft_listed(collection_id: T::CollectionId, nft_id: T::ItemId) -> bool {
		ListedNfts::<T>::contains_key(collection_id, nft_id)
	}

	/// Helper function to check if an account has already submitted an offer on a RMRK NFT
	///
	/// Parameters:
	/// - collection_id: The collection id of the RMRK NFT
	/// - nft_id: The nft id of the RMRK NFT
	/// - sender: The account that may or may not have already sent an offer
	fn has_active_offer(
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		sender: T::AccountId,
	) -> bool {
		Offers::<T>::contains_key((collection_id, nft_id), sender)
	}

	/// Helper function to check if the NFT's parent is a User Account
	///
	/// Parameters:
	/// - collection_id: The collection id of the RMRK NFT
	/// - nft_id: The nft id of the RMRK NFT
	fn is_nft_owned_by_nft(collection_id: T::CollectionId, nft_id: T::ItemId) -> bool {
		let owner = pallet_uniques::Pallet::<T>::owner(collection_id.into(), nft_id);
		if let Some(current_owner) = owner {
			let current_owner_cid_nid =
				pallet_rmrk_core::Pallet::<T>::decode_nft_account_id::<T::AccountId>(current_owner);
			if let Some(_current_owner_cid_nid) = current_owner_cid_nid {
				return true
			}
		}
		false
	}

	/// Helper function to handle market fees and royalty payments that are
	/// implemented downstream. By default, no market fees or royalties
	/// are paid out.
	///
	/// Parameters:
	/// - buyer: Account ID of the buyer.
	/// - seller: Account ID of the seller.
	/// - amount: Amount the NFT is being sold for.
	/// - market_fee: Percentage to calculate the market free implemented in `MarketplaceHooks`
	///   trait.
	/// - royalty_info: Royalty account and royalty fee to be calculated in the `MarketplaceHooks`
	///   trait.
	fn calculate_and_finalize_purchase_and_fees(
		buyer: T::AccountId,
		seller: T::AccountId,
		collection_id: T::CollectionId,
		nft_id: T::ItemId,
		amount: BalanceOf<T>,
		market_fee: Permill,
		royalty_info: Option<RoyaltyInfo<T::AccountId, Permill>>,
	) -> DispatchResult {
		let mut final_amount_after_fees = amount.clone();
		// Calculate market fee and update final amount
		if let Some(calculated_market_fee) =
			T::MarketplaceHooks::calculate_market_fee(amount, market_fee)
		{
			let marketplace_owner =
				MarketplaceOwner::<T>::get().ok_or(Error::<T>::MarketplaceOwnerNotSet)?;
			final_amount_after_fees = final_amount_after_fees.saturating_sub(calculated_market_fee);
			<T as pallet::Config>::Currency::transfer(
				&buyer,
				&marketplace_owner,
				calculated_market_fee,
				ExistenceRequirement::KeepAlive,
			)?;
			Self::deposit_event(Event::MarketFeePaid {
				sender: buyer.clone(),
				marketplace_owner,
				collection_id,
				nft_id,
				amount: calculated_market_fee,
			})
		}
		// Calculate royalty fees and update the final amount
		if let Some(royalty_info) = royalty_info {
			if let Some(calculated_royalty_fee) =
				T::MarketplaceHooks::calculate_royalty_fee(amount, royalty_info.amount)
			{
				let royalty_owner = royalty_info.recipient;
				final_amount_after_fees =
					final_amount_after_fees.saturating_sub(calculated_royalty_fee);
				<T as pallet::Config>::Currency::transfer(
					&buyer,
					&royalty_owner,
					calculated_royalty_fee,
					ExistenceRequirement::KeepAlive,
				)?;
				Self::deposit_event(Event::RoyaltyFeePaid {
					sender: buyer.clone(),
					royalty_owner,
					collection_id,
					nft_id,
					amount: calculated_royalty_fee,
				})
			}
		}
		// Finalize payment
		<T as pallet::Config>::Currency::transfer(
			&buyer,
			&seller,
			final_amount_after_fees,
			ExistenceRequirement::KeepAlive,
		)?;

		Ok(())
	}
}
