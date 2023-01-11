#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-market pallet

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Get;
use pallet_rmrk_core::Pallet as RmrkCore;
use sp_runtime::{traits::Bounded, Permill, SaturatedConversion};

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

fn u32_to_balance<T: Config>(
	val: u32,
) -> <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance
{
	<<T as pallet::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance::from(val)
}

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
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
	let collection_id = <T as pallet::Config>::Helper::collection(collection_index);
	let metadata = bvec![0u8; 20];
	let max = None;
	let symbol = bvec![0u8; 15];
	<T as pallet_uniques::Config>::Currency::make_free_balance_be(
		&caller,
		50_000_000u64.saturated_into(),
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
	let nft_id = <T as pallet::Config>::Helper::item(nft_index);
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

/// Lists an Nft
fn list_test_nft<T: Config>(
	owner: T::AccountId,
	collection_id: T::CollectionId,
	nft_id: T::ItemId,
	price: u32,
) -> <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance
{
	let amount = u32_to_balance::<T>(price);
	let _ = RmrkMarket::<T>::list(
		RawOrigin::Signed(owner.clone()).into(),
		collection_id,
		nft_id,
		amount,
		None,
	);
	amount.into()
}

benchmarks! {
	buy {
		let owner = funded_account::<T>("owner", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 42);

		let price = list_test_nft::<T>(owner.clone(), collection_id, nft_id, 100);
		let caller: T::AccountId = whitelisted_caller();
		<T as pallet_uniques::Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	}: _(RawOrigin::Signed(caller.clone()), collection_id, nft_id, None)
	verify {
		assert_last_event::<T>(Event::TokenSold { owner, buyer: caller, collection_id, nft_id, price }.into());
	}

	list {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 1;

		let collection_id = create_test_collection::<T>(caller.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(caller.clone(), None, collection_id, 42);
		let price = u32_to_balance::<T>(100);
	}: _(RawOrigin::Signed(caller.clone()), collection_id, nft_id, price, None)
	verify {
		assert_last_event::<T>(Event::TokenListed { owner: caller, collection_id, nft_id, price }.into());
	}

	unlist {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 1;

		let collection_id = create_test_collection::<T>(caller.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(caller.clone(), None, collection_id, 42);

		let _ = list_test_nft::<T>(caller.clone(), collection_id, nft_id, 100);
	}: _(RawOrigin::Signed(caller.clone()), collection_id, nft_id)
	verify {
		assert_last_event::<T>(Event::TokenUnlisted { owner: caller, collection_id, nft_id }.into());
	}

	make_offer {
		let owner = funded_account::<T>("owner", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 42);

		let caller: T::AccountId = whitelisted_caller();
		let amount =  T::MinimumOfferAmount::get();
		<T as pallet_uniques::Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	}: _(RawOrigin::Signed(caller.clone()), collection_id, nft_id, amount, None)
	verify {
		assert_last_event::<T>(Event::OfferPlaced { offerer: caller, collection_id, nft_id, price: amount }.into());
	}

	withdraw_offer {
		let owner = funded_account::<T>("owner", 0);
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 42);

		let caller: T::AccountId = whitelisted_caller();
		let amount =  T::MinimumOfferAmount::get();
		<T as pallet_uniques::Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		let _ = RmrkMarket::<T>::make_offer(RawOrigin::Signed(caller.clone()).into(), collection_id, nft_id, amount, None);
	}: _(RawOrigin::Signed(caller.clone()), collection_id, nft_id)
	verify {
		assert_last_event::<T>(Event::OfferWithdrawn { sender: caller, collection_id, nft_id }.into());
	}

	accept_offer {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 1;

		let collection_id = create_test_collection::<T>(caller.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(caller.clone(), None, collection_id, 42);

		let offerer = funded_account::<T>("offerer", 0);
		let amount =  T::MinimumOfferAmount::get();
		let _ = RmrkMarket::<T>::make_offer(RawOrigin::Signed(offerer.clone()).into(), collection_id, nft_id, amount, None);
	}: _(RawOrigin::Signed(caller.clone()), collection_id, nft_id, offerer.clone())
	verify {
		assert_last_event::<T>(Event::OfferAccepted { owner: caller, buyer: offerer, collection_id, nft_id }.into());
	}

	impl_benchmark_test_suite!(RmrkMarket, crate::mock::new_test_ext(), crate::mock::Test);
}
