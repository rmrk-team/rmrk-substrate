use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, traits::Currency};
use pallet_balances::Error as BalancesError;
use sp_std::prelude::*;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// TODO: test cases
		assert_eq!(1, 1);
	});
}
