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
		// First we'll build our parts
		// Fixed part body 1 is one option for body type
		let fixed_part_body_1 = FixedPart {
			id: 101,
			z: 0,
			src: stb("body-1"),
		};
		// Fixed part body 2 is second option for body type
		let fixed_part_body_2 = FixedPart {
			id: 102,
			z: 0,
			src: stb("body-2"),
		};
		// Slot part left hand can equip items from collections 0 or 1
		let slot_part_left_hand = SlotPart {
			id: 201,
			z: 0,
			src: stb("left-hand"),
			equippable: vec![
				0, // Collection 0
				1, // Collection 1
			]
		};
		// Slot part right hand can equip items from collections 2 or 3
		let slot_part_right_hand = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: vec![
				2, // Collection 2
				3, // Collection 3
			]
		};
		// Let's create a base with these 4 parts
		RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			stb("svg"), // base_type
			stb("KANPEOPLE"), // symbol
			vec![
				NewPartTypes::FixedPart(fixed_part_body_1),
				NewPartTypes::FixedPart(fixed_part_body_2),
				NewPartTypes::SlotPart(slot_part_left_hand),
				NewPartTypes::SlotPart(slot_part_right_hand),
				],
		);

		// Create collection 0
		RmrkCore::create_collection(
			Origin::signed(ALICE),
			stb("ipfs://col0-metadata"), // metadata
			Some(5), // max
			stb("COL1") // symbol
		);

		// Create collection 1
		RmrkCore::create_collection(
			Origin::signed(ALICE),
			stb("ipfs://col1-metadata"), // metadata
			Some(5), // max
			stb("COL2") // symbol
		);

		// Mint NFT 0 from collection 0
		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE, // owner
			0, // collection ID
			Some(ALICE), // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://warrior-metadata"), // metadata
		);

		// Mint NFT 0 from collection 1
		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE, // owner
			1, // collection ID
			Some(ALICE), // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://sword-metadata"), // metadata
		);

		// Sends NFT (0, 1) [sword] to NFT (0, 0) [warrior]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			1, // Collection ID
			0, // NFT ID
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0), // Recipient
		));

		// Add our body-1+left-hand resource to our soldier nft
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
						101, // ID of body-1 part
						201, // ID of left-hand slot
					],
					src: Some(stb("ipfs://backup-src")), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
				}
			),
		));

		// Add our sword resource to our sword NFT
		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			1, // collection id
			0, // nft id
			None, // base id
			ResourceType::Base(
				NoncomposableResource { 
					base: 0, // Base ID this resource can be equipped into
					slot_id: 201, // Slot ID this resource can be equipped into
					id: 777, // pub id: ResourceId,
					src: stb("ipfs://sword-metadata"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					theme_id: None, // pub themeId: Option<BoundedString>,
				}
			),
		));

		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			1, // Item CollectionId
			0, // Item NftId
			0, // Equipper CollectionId
			0, // Equipper NftId
			0, // BaseId
			201, // SlotId
		));
	});
}





		// for i in <pallet_rmrk_core::NewResources<Test>>::iter_prefix_values((0, 0, None::<BaseId>)) {
		// 	println!("iii: {:?}", i);
		// }

		// println!("aaa: {:?}", <pallet_rmrk_core::NewResources<Test>>::get((0, 0, None::<BaseId>, 1)));
		// println!("aaa: {:?}", <pallet_rmrk_core::NewResources<Test>>::get((0, 0, None::<BaseId>, 2)));

		// for i in <pallet_rmrk_core::NewResources<Test>>::iter_prefix_values((0, 0, Some(0))) {
		// 	println!("jjj: {:?}", i);
		// }

		// for i in Equippings::<Test>::iter_prefix_values(((0, 0), 0)) {
		// 	println!("kkk: {:?}", i);
		// }



		// let fixed_part = FixedPart {
		// 	id: 333,
		// 	z: 0,
		// 	src: stb("fixed_part_src"),
		// };

		// let fixed_part_2 = FixedPart {
		// 	id: 444,
		// 	z: 0,
		// 	src: stb("fixed_part_src"),
		// };

		// let slot_part = SlotPart {
		// 	id: 555,
		// 	z: 0,
		// 	src: stb("slot_part_src"),
		// 	equippable: vec![
		// 		0, // Collection 0
		// 		1, // Collection 1
		// 	]
		// };

		// RmrkEquip::create_base(
		// 	Origin::signed(ALICE), // origin
		// 	bvec![0u8; 20], // base_type
		// 	bvec![0u8; 20], // symbol
		// 	vec![
		// 		NewPartTypes::FixedPart(fixed_part),
		// 		NewPartTypes::FixedPart(fixed_part_2.clone()),
		// 		NewPartTypes::FixedPart(fixed_part_2),
		// 		NewPartTypes::SlotPart(slot_part),
		// 		],
		// );