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

/// Base: Basic equip tests
#[test]
fn equip_works() {
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


		

		// When you SEND an item to an NFT, it is not equipped into a slot by default.

		// First, an NFT exists.  There are no inherent requirements of this NFT.  The requirements become associated with the
		// *resources* added to the NFT.  Which will come later.

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
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
		);

		// ALICE sends NFT (0, 1) [child] to ALICE-owned NFT (0, 0) [parent]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

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

		// Add resource to NFT
		assert_ok!(RmrkCore::add_resource(
			Origin::signed(ALICE),
			0,
			0,
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(stb("slot_part_src")), // slot
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
		));


		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			0,
			0,
			ResourceType::Slot(
				SlotResourceInfo { //<BaseId, SlotId, ResourceId, BoundedString> {
					base: 0, // pub base: BaseId,
					slot_id: 0, // pub slot_id: SlotId,
					id: 0, // pub id: ResourceId,
					src: stb("src"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					themeId: None, // pub themeId: Option<BoundedString>,
				}
			), // slot
		));

		println!("CORE:\n {:?}", RmrkCore::new_resources((0,0,1)));





		// Separately, a BASE may be defined of a wide array of parts of two categories, fixed and slot.
		// - Fixed parts are *defined* pieces.  For example, hairstyle-bald, hairstyle-curly, with specific data defining what these
		// pieces are.
		// - Slot parts are *placeholder* pieces.  They don't define what will take its spot, rather they define only which
		// collection IDs qualify for filling this slot.

		// Once a BASE is defined, the issuer of a collection may RESADD a resource to an NFT in that collection.  This would define
		// the combination of fixed and slot parts that compose this resource.

		// Once the resource has been ACCEPTed by the NFTs owner, equipping operations may begin.

		// If an NFT owns an NFT that is a member of a collection that is EQUIPPABLE into a base's slot, it may be EQUIPped

		// Should not be able to equip onto non-existing NFT
		// Should not be able to equip onto non-owned NFT
		// Should not be able to equip a non-existent base

		// RmrkEquip::equip();

		/*

		Here's a RESADD for a kanaria:
		resadd: ["RMRK", 
		"RESADD", 
		"2.0.0",
		"8949171-e0b9bdcc456a36497a-KANBIRD-KANL-00009479", 
		"%7B%22base%22%3A%22base-8788686-KANBASE%22%2C%22id%22%3A%22ugYtHT0b%22%2C%22parts%22%3A%5B%22var4_beak%22%2C%22
		var2_body%22%2C%221f32b_eyes%22%2C%22var2_footLeft%22%2C%22var2_footRight%22%2C%22var2_handLeft%22%2C%222694-fe0f_handright%22
		%2C%22var2_head%22%2C%22var2_tail%22%2C%22var2_wingLeft%22%2C%222694-fe0f_wingright%22%2C%22card_le%22%2C%22background%22%2C
		%22foreground%22%2C%22headwear%22%2C%22backpack%22%2C%22objectleft%22%2C%22objectright%22%2C%22necklace%22%2C%22gem_empty2%22
		%2C%22gem_empty3%22%2C%22gem_empty4%22%5D%2C%22thumb%22%3A%22ipfs%3A%2F%2Fipfs%2FQmR3rK1P4n24PPqvfjGYNXWixPJpyBKTV6rYzAS2TYHLpT
		%22%2C%22themeId%22%3A%22mattel%22%7D"]

		Decoded:
		{
			"base":"base-8788686-KANBASE",
			"id":"6BrBxg_X",
			"parts":["1F60F_beak","var4_body","1F60F_eyes","var4_footLeft","var4_footRight",
				"var4_handLeft","1f48e_handRight","1f48e_head","var4_tail","var4_wingLeft","1f48e_wingRight","card_le","background",
				"foreground", "headwear","backpack","objectleft","objectright","necklace","gem_empty2","gem_empty3","gem_empty4"],
			"thumb":"ipfs://ipfs/QmR3rK1P4n24PPqvfjGYNXWixPJpyBKTV6rYzAS2TYHLpT",
			"themeId":"eggplant"
		}

		RESADD for new years eve sparkler:
		"RMRK", 
		"RESADD",
		"2.0.0",
		"10758408-e0b9bdcc456a36497a-EVNTS-NYE22-00000193",
		"%7B%22slot%22%3A%22base-8788686-KANBASE.objectright%22%2C%22
		id%22%3A%22fG-kSh3M%22%2C%22src%22%3A%22ipfs%3A%2F%2Fipfs%2FQmPAdcr87vu2mENZbJNg14yoRRjtUZ6vAYWN5oZQczyCvo%2F
		sparklers22_objectright.svg%22%2C%22thumb%22%3A%22ipfs%3A%2F%2Fipfs%2FQmPAdcr87vu2mENZbJNg14yoRRjtUZ6vAYWN5oZQczyCvo%2F
		sparklers22_object_thumb.png%22%7D"

		Decoded:
		{
			"slot":"base-8788686-KANBASE.objectright",
			"id":"fG-kSh3M",
			"src":"ipfs://ipfs/QmPAdcr87vu2mENZbJNg14yoRRjtUZ6vAYWN5oZQczyCvo/sparklers22_objectright.svg",
			"thumb":"ipfs://ipfs/QmPAdcr87vu2mENZbJNg14yoRRjtUZ6vAYWN5oZQczyCvo/sparklers22_object_thumb.png"
		}

		Create TWO resadd operations.  (1) BaseResAdd and (2) SlotResAdd.  


		*/

		println!("{:?}", RmrkEquip::bases(0).unwrap());
	});
}
