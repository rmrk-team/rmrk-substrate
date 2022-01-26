#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::BoundedVec;
use frame_support::dispatch::DispatchError;

pub use pallet::*;

use rmrk_traits::{primitives::*, BaseInfo, Base};

mod functions;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub type StringLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn bases)]
	/// Stores bases info
	pub type Bases<T: Config> =
		StorageMap<_, Twox64Concat, BaseId, BaseInfo<T::AccountId, StringLimitOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn next_base_id)]
	pub type NextBaseId<T: Config> = StorageValue<_, BaseId, ValueQuery>;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BaseCreated { issuer: T::AccountId, base_id: BaseId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableBaseId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// TODO: equip a child NFT into a parent's slot, or unequip
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn equip(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Self::deposit_event(Event::SomethingStored(something, sender));
			Ok(())
		}

		/// TODO: changes the list of equippable collections on a base's part
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn equippable(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Self::deposit_event(Event::SomethingStored(something, sender));
			Ok(())
		}

		/// TODO: add a new theme to a base
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn theme_add(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Self::deposit_event(Event::SomethingStored(something, sender));
			Ok(())
		}

		/// create a base. catalogue of parts. It is not an NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_base(
			origin: OriginFor<T>,
			base_type: BoundedVec<u8, T::StringLimit>,
			symbol: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let base_id = Self::base_create(sender.clone(), base_type, symbol)?;

			Self::deposit_event(Event::BaseCreated { issuer: sender, base_id });
			Ok(())
		}
	}
}
