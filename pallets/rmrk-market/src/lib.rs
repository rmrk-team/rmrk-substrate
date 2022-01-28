#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use frame_support::{
	dispatch::DispatchResult, ensure,
	traits::{Currency, BalanceStatus::Reserved, ReservableCurrency},
	transactional, BoundedVec,
};
use frame_support::traits::ExistenceRequirement;
use frame_system::{ensure_signed, RawOrigin};
use sp_runtime::Permill;

use sp_std::prelude::*;

pub use pallet::*;

use rmrk_traits::{AccountIdOrCollectionNftTuple, NftInfo, primitives::*};

pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use crate::types::Offer;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type InstanceInfoOf<T> = NftInfo<
		<T as frame_system::Config>::AccountId,
		BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>, >;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub type OfferOf<T> = Offer<
		<T as frame_system::Config>::AccountId, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type ProtocolOrigin: EnsureOrigin<Self::Origin>;

		/// The market currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Minimum offer amount as a valid offer
		#[pallet::constant]
		type MinimumOfferAmount: Get<BalanceOf<Self>>;

		// TODO: Weight values for this pallet
		// type WeightInfo: WeightInfo;
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
		CollectionId,
		Blake2_128Concat,
		NftId,
		BalanceOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers)]
	/// Stores offer on a NFT info
	pub type Offers<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(CollectionId, NftId),
		Blake2_128Concat,
		T::AccountId,
		OfferOf<T>,
		OptionQuery,
	>;

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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
		/// Buy a listed NFT. Ensure that the NFT is available for purchase and has not recently
		/// been purchased, sent, or burned.
		///
		/// Parameters:
		///	- `origin` - Account of the potential buyer
		///	- `collection_id` - Collection id of the RMRK NFT
		///	- `nft_id` - NFT id of the RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn buy(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			Self::do_buy(sender, collection_id, nft_id, false)
		}

		/// List a RMRK NFT on the Marketplace for purchase. A listing can be cancelled, and is
		/// automatically considered cancelled when a `buy` is executed on top of a given listing.
		/// An NFT that has another NFT as its owner CANNOT be listed. An NFT owned by a NFT must
		/// first be sent to an account before being listed.
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

			let owner = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id);
			// Ensure that the NFT is not owned by an NFT
			if let Some(current_owner) = owner {
				let current_owner_cid_nid =
					pallet_rmrk_core::Pallet::<T>::decode_nft_account_id::<T::AccountId>(current_owner.clone());
				if let Some(_current_owner_cid_nid) = current_owner_cid_nid {
					Err(Error::<T>::CannotListNftOwnedByNft)?;
				}
				// Ensure sender is not the owner
				ensure!(sender == current_owner, Error::<T>::NoPermission);

				ListedNfts::<T>::mutate_exists(collection_id, nft_id, |list_price| *list_price = Some(amount));

				// TODO: royalty info

				Self::deposit_event(Event::TokenListed {
					owner: current_owner,
					collection_id,
					nft_id,
					price: amount,
					royalty: None
				});
			} else {
				Err(Error::<T>::TokenDoesNotExist)?;
			}

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
			// Check if NFT is still in ListedNfts storage
			ensure!(Self::is_nft_listed(collection_id, nft_id), Error::<T>::CannotUnlistToken);
			let owner = pallet_uniques::Pallet::<T>::owner(collection_id, nft_id);
			// Ensure owner of NFT is performing call to unlist
			if let Some(current_owner) = owner {
				ensure!(sender == current_owner, Error::<T>::NoPermission);
				// Remove from storage
				ListedNfts::<T>::remove(collection_id, nft_id);
				// Emit TokenUnlisted Event
				Self::deposit_event(Event::TokenUnlisted {
					owner: current_owner,
					collection_id,
					nft_id,
				});
			} else {
				Err(Error::<T>::TokenDoesNotExist)?;
			}

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
			expires: Option<T::BlockNumber>,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			// Ensure amount is above the minimum threshold
			ensure!(amount >= T::MinimumOfferAmount::get(), Error::<T>::OfferTooLow);
			// Ensure NFT exists & sender is not owner
			let owner = pallet_uniques::Pallet::<T>::
				owner(collection_id, nft_id).ok_or(Error::<T>::TokenDoesNotExist)?;

			ensure!(sender != owner, Error::<T>::CannotOfferOnOwnToken);
			// If offer has already been made, must withdraw_offer first before making a new offer
			ensure!(!Self::has_active_offer(collection_id, nft_id, sender.clone()), Error::<T>::AlreadyOffered);

			let token_id = (collection_id, nft_id);
			// Insert new offer into Offers storage
			Offers::<T>::insert(
				token_id,
				sender.clone(),
				Offer {
					maker: sender.clone(),
					amount,
					expires
				},
			);
			// Reserve currency from offerer account
			<T as pallet::Config>::Currency::reserve(&sender, amount)?;
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
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn withdraw_offer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			let token_id = (collection_id, nft_id);
			// Ensure that offer exists from sender that is withdrawing their offer
			Offers::<T>::try_mutate_exists(token_id, sender.clone(), |maybe_offer| -> DispatchResult {
				let offer = maybe_offer.take().ok_or(Error::<T>::UnknownOffer)?;
				// Ensure NFT exists & sender is not owner
				let owner = pallet_uniques::Pallet::<T>::
				owner(collection_id, nft_id).ok_or(Error::<T>::TokenDoesNotExist)?;
				// Cannot withdraw offer on own token
				ensure!(sender == owner || sender == offer.maker, Error::<T>::CannotWithdrawOffer);

				// Unreserve currency from offerer account
				<T as pallet::Config>::Currency::unreserve(&sender, offer.amount);
				// Emit OfferWithdrawn Event
				Self::deposit_event(Event::OfferWithdrawn {
					sender,
					collection_id,
					nft_id,
				});

				Ok(())
			})
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

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
{
	/// Buy the NFT helper funciton logic to handle both transactional calls of `buy` and
	/// `accept_offer`
	///
	/// Parameters:
	/// - `buyer`: The account that is buying the RMRK NFT
	/// - `collection_id`: The collection id of the RMRK NFT
	/// - `nft_id`: The id of the RMRK NFT
	/// - `is_offer`: Whether the call is from `accept_offer` or `buy`
	fn do_buy(
		buyer: T::AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		is_offer: bool,
	) -> DispatchResult {
		// Ensure buyer is not the root owner
		let (root_owner, _) = pallet_rmrk_core::Pallet::<T>::lookup_root_owner(collection_id, nft_id)?;
		ensure!(buyer != root_owner, Error::<T>::CannotBuyOwnToken);

		let root_owner_origin = T::Origin::from(RawOrigin::Signed(root_owner.clone()));
		let token_id = (collection_id, nft_id);

		ListedNfts::<T>::try_mutate(collection_id, nft_id, |list_price| -> DispatchResult {
			let mut list_price = if is_offer {
				Offers::<T>::get(token_id, buyer.clone())
					.map(|o| o.amount)
					.ok_or(Error::<T>::UnknownOffer)?
			} else {
				list_price.take().ok_or(Error::<T>::TokenNotForSale)?
			};

			// Check royalty set in NftInfo, NftInfo is populated at mint & if it was removed from
			// Nfts Storage, lookup_root_owner would have thrown an error.
			let nft_info = pallet_rmrk_core::Nfts::<T>::get(collection_id, nft_id);
			//let royalty = nft_info.royalty;
			//let recipient = nft_info.recipient;

			// TODO: Calculate royalty to be paid to the recipient
			// 1) Calculate price and substract from list_price
			// 2) Transfer currency to recipient
			// 3) Emit Event

			// Transfer currency then transfer the NFT
			<T as pallet::Config>::Currency::transfer(&buyer, &root_owner, list_price, ExistenceRequirement::KeepAlive)?;

			let new_owner = AccountIdOrCollectionNftTuple::AccountId(buyer.clone());
			pallet_rmrk_core::Pallet::<T>::send(root_owner_origin, collection_id, nft_id, new_owner)?;


			Self::deposit_event(Event::TokenSold {
				owner: root_owner,
				buyer,
				collection_id,
				nft_id,
				price: list_price,
				royalty: None,
				royalty_amount: None
			});

			Ok(())
		})
	}

	/// Helper function to check if a RMRK NFT is listed
	///
	/// Parameters:
	/// - collection_id: The collection id of the RMRK NFT
	/// - nft_id: The nft id of the RMRK NFT
	fn is_nft_listed(
		collection_id: CollectionId,
		nft_id: NftId,
	) -> bool {
		ListedNfts::<T>::contains_key(collection_id, nft_id)
	}

	/// Helper function to check if an account has already submitted an offer on a RMRK NFT
	///
	/// Parameters:
	/// - collection_id: The collection id of the RMRK NFT
	/// - nft_id: The nft id of the RMRK NFT
	/// - sender: The account that may or may not have already sent an offer
	fn has_active_offer(
		collection_id: CollectionId,
		nft_id: NftId,
		sender: T::AccountId,
	) -> bool {
		Offers::<T>::contains_key((collection_id, nft_id), sender)
	}

}
