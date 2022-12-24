#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-equip pallet

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

use crate::Pallet as RmrkEquip;

pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

const SEED: u32 = 0;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn funded_account<T: Config>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

/// Creates a base
fn create_base<T: Config>(creator: T::AccountId) {
	let _ = RmrkEquip::<T>::create_base(
		RawOrigin::Signed(creator).into(), // origin
		bvec![0u8; 20],                    // base_type
		bvec![0u8; 20],                    // symbol
		bvec![],                           // parts
	);
}

benchmarks! {
	change_base_issuer {
		let caller: T::AccountId = whitelisted_caller();
		let new_issuer = funded_account::<T>("new_issuer", 0);
		let new_issuer_lookup = T::Lookup::unlookup(new_issuer.clone());
		create_base::<T>(caller.clone());
	}: _(RawOrigin::Signed(caller.clone()), 0u32, new_issuer_lookup)
	verify {
		assert_last_event::<T>(Event::BaseIssuerChanged {
			old_issuer: caller,
			new_issuer: new_issuer,
			base_id: 0u32,
		}.into());
	}

	impl_benchmark_test_suite!(RmrkEquip, crate::benchmarking::tests::new_test_ext(), crate::mock::Test);
}

#[cfg(test)]
mod tests {
	use crate::mock;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		mock::ExtBuilder::default().build()
	}
}
