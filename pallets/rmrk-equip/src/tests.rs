use frame_support::{
	// assert_noop, 
	assert_ok, 
	// error::BadOrigin
};
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
			// id: stb("fixed_part_id"),
			id: 100,
			z: 0,
			src: stb("fixed_part_src"),
		};
		let slot_part = SlotPart {
			// id: stb("slot_part_id"),
			id: 102,
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
				NewPartTypes::FixedPart(fixed_part),
				NewPartTypes::SlotPart(slot_part),
				],
		);

		// println!("{:?}", RmrkEquip::bases(0).unwrap());
	});
}

/// Base: Basic equip tests
#[test]
fn equip_works() {
	ExtBuilder::default().build().execute_with(|| {
		let fixed_part = FixedPart {
			id: 111,
			z: 0,
			src: stb("fixed_part_src"),
		};
		let slot_part = SlotPart {
			id: 222,
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
				NewPartTypes::FixedPart(fixed_part),
				NewPartTypes::SlotPart(slot_part),
				],
		);


		

		// When you SEND an item to an NFT, it is not equipped into a slot by default.

		// First, an NFT exists.  There are no inherent requirements of this NFT.  The requirements become associated with the
		// *resources* added to the NFT.  Which will come later.

		RmrkCore::create_collection(Origin::signed(ALICE), bvec![0u8; 20], Some(5), bvec![0u8; 15]);
		RmrkCore::create_collection(Origin::signed(ALICE), bvec![0u8; 20], Some(5), bvec![0u8; 15]);

		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
		);

		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			1,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
		);

		// ALICE sends NFT (0, 1) [child] to ALICE-owned NFT (0, 0) [parent]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			1,
			0,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

		let fixed_part = FixedPart {
			id: 333,
			z: 0,
			src: stb("fixed_part_src"),
		};

		let fixed_part_2 = FixedPart {
			id: 444,
			z: 0,
			src: stb("fixed_part_src"),
		};

		let slot_part = SlotPart {
			id: 555,
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
				NewPartTypes::FixedPart(fixed_part),
				NewPartTypes::FixedPart(fixed_part_2.clone()),
				NewPartTypes::FixedPart(fixed_part_2),
				NewPartTypes::SlotPart(slot_part),
				],
		);

		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			0, // collection_id
			0, // nft id
			Some(0), // base id
			ResourceType::Slot(
				ComposableResource { //<BaseId, SlotId, ResourceId, BoundedString> {
					base: 0, // pub base: BaseId,
					id: 0, // pub id: ResourceId,
					parts: vec![
						2,
						3
					],
					src: Some(stb("src")), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
				}
			), // slot
		));

		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			0, // collection id
			0, // nft id
			None, // base id
			ResourceType::Base(
				NoncomposableResource { //<BaseId, SlotId, ResourceId, BoundedString> {
					base: 0, // pub base: BaseId,
					slot_id: 0, // pub slot_id: SlotId,
					id: 0, // pub id: ResourceId,
					src: stb("src"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					theme_id: None, // pub themeId: Option<BoundedString>,
				}
			), // slot
		));

		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			0, // collection id
			0, // nft id
			None, // base id
			ResourceType::Base(
				NoncomposableResource { //<BaseId, SlotId, ResourceId, BoundedString> {
					base: 0, // pub base: BaseId,
					slot_id: 0, // pub slot_id: SlotId,
					id: 0, // pub id: ResourceId,
					src: stb("src"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					theme_id: None, // pub themeId: Option<BoundedString>,
				}
			), // slot
		));

		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			1, // collection id
			0, // nft id
			None, // base id
			ResourceType::Base(
				NoncomposableResource { //<BaseId, SlotId, ResourceId, BoundedString> {
					base: 0, // pub base: BaseId,
					slot_id: 1, // pub slot_id: SlotId,
					id: 100, // pub id: ResourceId,
					src: stb("src"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					theme_id: None, // pub themeId: Option<BoundedString>,
				}
			), // slot
		));

		// println!("New res: {:?}", RmrkCore::new_resources((0,0,1)).unwrap());

		// (src_col, src_nft), (dest_col, dest_nft), base_id, part_id
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			1, // Item CollectionId
			0, // Item NftId
			0, // Equipper CollectionId
			0, // Equipper NftId
			0, // BaseId
			1, // SlotId
		));

		for i in <pallet_rmrk_core::NewResources<Test>>::iter_prefix_values((0, 0, None::<BaseId>)) {
			println!("iii: {:?}", i);
		}

		println!("aaa: {:?}", <pallet_rmrk_core::NewResources<Test>>::get((0, 0, None::<BaseId>, 1)));
		println!("aaa: {:?}", <pallet_rmrk_core::NewResources<Test>>::get((0, 0, None::<BaseId>, 2)));

		for i in <pallet_rmrk_core::NewResources<Test>>::iter_prefix_values((0, 0, Some(0))) {
			println!("jjj: {:?}", i);
		}

		for i in Equippings::<Test>::iter_prefix_values(((0, 0), 0)) {
			println!("kkk: {:?}", i);
		}



	});
}
