use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use sp_runtime::Permill;

use super::*;
use mock::{Event as MockEvent, *};
use pallet_uniques as UNQ;

use sp_std::{convert::TryInto, vec::Vec};

type RMRKEquip = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, UniquesStringLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Base: Basic base tests
#[test]
fn create_base_works() {
	ExtBuilder::default().build().execute_with(|| {
		let fixed_part = FixedPart {
			id: stb("fixed_part_id"),
			z: 0,
			src: stb("fixed_part_src"),
		};
		let slot_part = SlotPart {
			id: stb("slot_part_id"),
			z: 0,
			src: stb("slot_part_src"),
			equippable: vec![
				0, // Collection 0
				1, // Collection 1
			]
		};

		RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			bvec![0u8; 20], // base_type
			bvec![0u8; 20], // symbol
			vec![
				FixedOrSlotPart::FixedPart(fixed_part),
				FixedOrSlotPart::SlotPart(slot_part),
				],
		);

		println!("{:?}", RmrkEquip::bases(0).unwrap());
	});
}
