#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult, ensure, traits::tokens::nonfungibles::*, transactional, BoundedVec,
};
use frame_system::ensure_signed;
use sp_std::cmp::Eq;
use sp_runtime::{DispatchError, RuntimeDebug};
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use rmrk_traits::{primitives::*};


pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct AuctionInfo<AccountId, BoundedString> {
	pub issuer: AccountId,
	pub name: BoundedString
}

pub type AuctionInfoOf<T> = AuctionInfo<
	<T as frame_system::Config>::AccountId,
	BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, StorageValue};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Auctions<T: Config> = StorageMap<_, Twox64Concat, AuctionId, AuctionInfoOf<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AuctionCreated { issuer: T::AccountId, auction_id: AuctionId },
		AuctionEnded { auction_id: AuctionId },
		AuctionDeleted { auction_id: AuctionId }
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_auction(origin: OriginFor<T>, auction_id: AuctionId) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let issuer = ensure_signed(origin)?;

			// Update storage.
			// <Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::AuctionCreated { issuer, auction_id });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


	}
}
