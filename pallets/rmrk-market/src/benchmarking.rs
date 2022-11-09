#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-market pallet

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use pallet_rmrk_core::Pallet as RmrkCore;
use sp_runtime::{traits::Bounded, Permill};

use crate::Pallet as RmrkMarket;

pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

const SEED: u32 = 0;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn funded_account<T: Config>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	<T as pallet_uniques::Config>::Currency::make_free_balance_be(
		&caller,
		BalanceOf::<T>::max_value(),
	);
	caller
}

/// Creates a collection
fn create_test_collection<T: Config>(
	caller: T::AccountId,
	collection_index: u32,
) -> T::CollectionId {
	let collection_id = T::Helper::collection(collection_index);
	let metadata = bvec![0u8; 20];
	let max = None;
	let symbol = bvec![0u8; 15];
	<T as pallet_uniques::Config>::Currency::make_free_balance_be(
		&caller,
		BalanceOf::<T>::max_value(),
	);
	let _ = RmrkCore::<T>::create_collection(
		(RawOrigin::Signed(caller.clone())).into(),
		collection_id.clone(),
		metadata,
		max,
		symbol,
	);

	collection_id
}

/// Mint a token
fn mint_test_nft<T: Config>(
	owner: T::AccountId,
	mint_for: Option<T::AccountId>,
	collection_id: T::CollectionId,
	nft_index: u32,
) -> T::ItemId {
	let nft_id = T::Helper::item(nft_index);
	let royalty_recipient = owner.clone();
	let royalty = Permill::from_percent(1);
	let nft_metadata = bvec![0u8; 20];
	let resource = None;
	let _ = RmrkCore::<T>::mint_nft(
		RawOrigin::Signed(owner.clone()).into(),
		mint_for,
		nft_id,
		collection_id,
		Some(royalty_recipient),
		Some(royalty),
		nft_metadata,
		true,
		resource,
	);
	nft_id
}

benchmarks! {
	buy {
		let bob = funded_account::<T>("bob", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(bob.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(bob.clone(), None, collection_id, 42);
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), collection_id, nft_id, None)
	verify {
		//assert_last_event::<T>(Event::CollectionCreated { issuer: caller, collection_id }.into());
	}

	impl_benchmark_test_suite!(RmrkMarket, crate::benchmarking::tests::new_test_ext(), crate::mock::Test);
}

#[cfg(test)]
mod tests {
	use crate::mock;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		mock::new_test_ext()
	}
}
