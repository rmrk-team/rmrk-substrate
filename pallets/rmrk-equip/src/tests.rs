use super::*;

use rmrk_traits::{FixedPart, SlotPart, ThemeProperty};

use frame_support::{assert_noop, assert_ok};
use mock::{Event as MockEvent, *};
use sp_runtime::Permill;
use sp_std::convert::TryInto;
type RMRKEquip = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, UniquesStringLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedResource
fn stbr(s: &str) -> BoundedVec<u8, ResourceSymbolLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Attempt to convert a &str to a BoundedVec of some sort
macro_rules! sbvec {
	($( $x:tt )*) => {
		$( $x )*.as_bytes().to_vec().try_into().unwrap()
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
			equippable: EquippableList::Custom(bvec![
				0, // Collection 0
				1, // Collection 1
			]),
		};

		assert_ok!(RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			bvec![0u8; 20],        // base_type
			bvec![0u8; 20],        // symbol
			bvec![PartType::FixedPart(fixed_part), PartType::SlotPart(slot_part),],
		));
	});
}

/// Base: Change issuer tests (RMRK2.0 spec: CHANGEISSUER)=
#[test]
fn change_base_issuer_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a base
		assert_ok!(RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			bvec![0u8; 20],        // base_type
			bvec![0u8; 20],        // symbol
			bvec![],               // parts
		));
		// Issuer should be Alice
		assert_eq!(RmrkEquip::bases(0).unwrap().issuer, ALICE);
		// Bob can't change issuer (no permission)
		assert_noop!(
			RmrkEquip::change_base_issuer(Origin::signed(BOB), 0, BOB),
			Error::<Test>::PermissionError
		);
		// Changing Base Issuer should be Alice
		assert_ok!(RmrkEquip::change_base_issuer(Origin::signed(ALICE), 0, BOB));
		// Issuer should be Bob
		assert_eq!(RmrkEquip::bases(0).unwrap().issuer, BOB);
		// Last event should be BaseIssuerChanged
		System::assert_last_event(MockEvent::RmrkEquip(crate::Event::BaseIssuerChanged {
			old_issuer: ALICE,
			new_issuer: BOB,
			base_id: 0,
		}));
	});
}

/// Base: Attempting to create a base with more the max parts fails
#[test]
#[should_panic]
fn exceeding_parts_bound_panics() {
	// PartsLimit bound is 50 per mock.rs, 60 should panic on unwrap
	let parts_bounded_vec: BoundedVec<PartId, PartsLimit> = bvec![
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9,
		10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8,
		9, 10,
	];
}

/// Base: Basic equip tests
#[test]
fn equip_works() {
	ExtBuilder::default().build().execute_with(|| {
		// First we'll build our parts
		// Fixed part body 1 is one option for body type
		let fixed_part_body_1 = FixedPart { id: 101, z: 0, src: stb("body-1") };
		// Fixed part body 2 is second option for body type
		let fixed_part_body_2 = FixedPart { id: 102, z: 0, src: stb("body-2") };
		// Slot part left hand can equip items from collections 0 or 1
		let slot_part_left_hand = SlotPart {
			id: 201,
			z: 0,
			src: stb("left-hand"),
			equippable: EquippableList::Custom(bvec![
				0, // Collection 0
				1, // Collection 1
			]),
		};
		// Slot part right hand can equip items from collections 2 or 3
		let slot_part_right_hand = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: EquippableList::Custom(bvec![
				0, // Collection 2
				1, // Collection 3
			]),
		};
		// Let's create a base with these 4 parts
		assert_ok!(RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			stb("svg"),            // base_type
			stb("KANPEOPLE"),      // symbol
			bvec![
				PartType::FixedPart(fixed_part_body_1),
				PartType::FixedPart(fixed_part_body_2),
				PartType::SlotPart(slot_part_left_hand),
				PartType::SlotPart(slot_part_right_hand),
			],
		));

		// Create collection 0
		assert_ok!(RmrkCore::create_collection(
			Origin::signed(ALICE),
			stb("ipfs://col0-metadata"), // metadata
			Some(5),                     // max
			sbvec!["COL0"]               // symbol
		));

		// Create collection 1
		assert_ok!(RmrkCore::create_collection(
			Origin::signed(ALICE),
			stb("ipfs://col1-metadata"), // metadata
			Some(5),                     // max
			sbvec!["COL1"]               // symbol
		));

		// Mint NFT 0 from collection 0 (character-0)
		assert_ok!(RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,                              // owner
			0,                                  // collection ID
			Some(ALICE),                        // recipient
			Some(Permill::from_float(1.525)),   // royalties
			stb("ipfs://character-0-metadata"), // metadata
		));

		// Mint NFT 1 from collection 0 (character-1)
		assert_ok!(RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,                              // owner
			0,                                  // collection ID
			Some(ALICE),                        // recipient
			Some(Permill::from_float(1.525)),   // royalties
			stb("ipfs://character-1-metadata"), // metadata
		));

		// Mint NFT 0 from collection 1 (sword)
		assert_ok!(RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,                            // owner
			1,                                // collection ID
			Some(ALICE),                      // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://sword-metadata"),     // metadata
		));

		// Mint NFT 1 from collection 1 (flashlight)
		assert_ok!(RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,                             // owner
			1,                                 // collection ID
			Some(ALICE),                       // recipient
			Some(Permill::from_float(1.525)),  // royalties
			stb("ipfs://flashlight-metadata"), // metadata
		));

		// Attempt to equip sword should fail as character-0 doesn't own sword
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				(1, 0),                // item
				(0, 0),                // equipper
				0,                     // BaseId
				201,                   // SlotId
			),
			Error::<Test>::MustBeDirectParent
		);

		// Sends NFT (0, 1) [sword] to NFT (0, 0) [character-0]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			1,                                                          // Collection ID
			0,                                                          // NFT ID
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0), // Recipient
		));

		// Attempt to equip sword should fail as character-0 doesn't have a resource that is
		// associated with this base
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				(1, 0),                // item
				(0, 0),                // equipper
				0,                     // BaseId
				201,                   // SlotId
			),
			Error::<Test>::NoResourceForThisBaseFoundOnNft
		);

		// Add a Base 0 resource (body-1 and left-hand slot) to our character-0 nft
		assert_ok!(RmrkCore::add_resource(
			Origin::signed(ALICE),
			0,                              // collection_id
			0,                              // nft id
			stbr("res-1"),                  // resource_id
			Some(0),                        // pub base: BaseId,
			Some(stb("ipfs://backup-src")), // pub src: BoundedString,
			None,                           // metadata
			None,                           // slot
			None,                           // license
			None,                           // thumb
			Some(bvec![
				// parts
				101, // ID of body-1 part
				201, // ID of left-hand slot
			]),
		));

		// Attempt to equip sword should fail as the sword doesn't have a resource that is
		// equippable into that slot
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				(1, 0),                // item
				(0, 0),                // equipper
				0,                     // BaseId
				201,                   // SlotId
			),
			Error::<Test>::ItemHasNoResourceToEquipThere
		);

		// Add our sword left-hand resource to our sword NFT
		assert_ok!(RmrkCore::add_resource(
			Origin::signed(ALICE),
			1,                                       // collection id
			0,                                       // nft id
			stbr("res-777"),                         // resource_id
			Some(0),                                 // pub base: BaseId,
			Some(stb("ipfs://sword-metadata-left")), // pub src: BoundedString,
			None,                                    // metadata
			Some(201),                               // slot
			None,                                    // license
			None,                                    // thumb
			None,                                    // parts
		));

		// Equipping should now work
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			(1, 0),                // item
			(0, 0),                // equipper
			0,                     // BaseId
			201,                   // SlotId
		));

		System::assert_last_event(MockEvent::RmrkEquip(crate::Event::SlotEquipped {
			item_collection: 1,
			item_nft: 0,
			base_id: 0,
			slot_id: 201,
		}));

		// Equipped resource ID Some(777) should now be associated with equippings for character-0
		// on base 0, slot 201
		let equipped = RmrkEquip::equippings(((0, 0), 0, 201));
		assert_eq!(equipped.clone().unwrap(), stbr("res-777"),);

		// Resource for equipped item should exist
		assert!(RmrkCore::resources((1, 0, equipped.unwrap())).is_some());

		// Add our sword left-hand resource to our sword NFT
		assert_ok!(RmrkCore::add_resource(
			Origin::signed(ALICE),
			1,                                       // collection id
			0,                                       // nft id
			stbr("res-130"),                         // resource_id
			Some(0),                                 // pub base: BaseId,
			Some(stb("ipfs://sword-metadata-left")), // pub src: BoundedString,
			None,                                    // metadata
			Some(201),                               // slot
			None,                                    // license
			None,                                    // thumb
			None,                                    // parts
		));

		// Add our sword right-hand resource to our sword NFT
		assert_ok!(RmrkCore::add_resource(
			Origin::signed(ALICE),
			1,                                        // collection id
			0,                                        // nft id
			stbr("res-130"),                          // resource_id
			Some(0),                                  // pub base: BaseId,
			Some(stb("ipfs://sword-metadata-right")), // pub src: BoundedString,
			None,                                     // metadata
			Some(202),                                // slot
			None,                                     // license
			None,                                     // thumb
			None,                                     // parts
		));

		// Equipping to right-hand should fail (already equipped in left hand)
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				(1, 0),                // item
				(0, 0),                // equipper
				0,                     // BaseId
				202,                   // SlotId
			),
			Error::<Test>::AlreadyEquipped
		);

		// Unequipping from left-hand should work
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			(1, 0),                // item
			(0, 0),                // equipper
			0,                     // BaseId
			201,                   // SlotId
		));

		System::assert_last_event(MockEvent::RmrkEquip(crate::Event::SlotUnequipped {
			item_collection: 1,
			item_nft: 0,
			base_id: 0,
			slot_id: 201,
		}));

		// Re-equipping to left-hand should work
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			(1, 0),                // item
			(0, 0),                // equipper
			0,                     // BaseId
			201,                   // SlotId
		));

		// Unequipping from left-hand should work
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			(1, 0),                // item
			(0, 0),                // equipper
			0,                     // BaseId
			201,                   // SlotId
		));

		// Equipping to right-hand should work
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			(1, 0),                // item
			(0, 0),                // equipper
			0,                     // BaseId
			202,                   // SlotId
		));
	});
}
/// Base: Basic equip tests
#[test]
fn equippable_works() {
	ExtBuilder::default().build().execute_with(|| {
		// First we'll build our parts
		// Fixed part body 1 is one option for body type
		let fixed_part_body_1 = FixedPart { id: 101, z: 0, src: stb("body-1") };
		// Fixed part body 2 is second option for body type
		let fixed_part_body_2 = FixedPart { id: 102, z: 0, src: stb("body-2") };
		// Slot part left hand can equip items from collections 0 or 1
		let slot_part_left_hand = SlotPart {
			id: 201,
			z: 0,
			src: stb("left-hand"),
			equippable: EquippableList::Custom(bvec![
				0, // Collection 0
				1, // Collection 1
			]),
		};
		// Slot part right hand can equip items from collections 2 or 3
		let slot_part_right_hand = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: EquippableList::Custom(bvec![
				2, // Collection 2
				3, // Collection 3
			]),
		};
		// Let's create a base with these 4 parts
		assert_ok!(RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			stb("svg"),            // base_type
			stb("KANPEOPLE"),      // symbol
			bvec![
				PartType::FixedPart(fixed_part_body_1),
				PartType::FixedPart(fixed_part_body_2),
				PartType::SlotPart(slot_part_left_hand),
				PartType::SlotPart(slot_part_right_hand),
			],
		));

		// equippable extrinsic should work
		assert_ok!(RmrkEquip::equippable(
			Origin::signed(ALICE),
			0,                                      // base ID
			202,                                    // slot ID
			EquippableList::Custom(bvec![5, 6, 7]), // equippable collections
		));

		// Last event should be EquippablesUpdated
		System::assert_last_event(MockEvent::RmrkEquip(crate::Event::EquippablesUpdated {
			base_id: 0,
			slot_id: 202,
		}));

		// Parts storage should be updated
		let should_be = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: EquippableList::Custom(bvec![5, 6, 7]),
		};
		assert_eq!(RmrkEquip::parts(0, 202).unwrap(), PartType::SlotPart(should_be));

		// Should not be able to change equippable on non-existent base
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(ALICE),
				666,                                    // base ID
				202,                                    // slot ID
				EquippableList::Custom(bvec![5, 6, 7]), // equippable collections
			),
			Error::<Test>::BaseDoesntExist
		);

		// Should not be able to change equippable on non-existent part
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(ALICE),
				0,                                      // base ID
				200,                                    // slot ID
				EquippableList::Custom(bvec![5, 6, 7]), // equippable collections
			),
			Error::<Test>::PartDoesntExist
		);

		// Should not be able to change equippable on FixedPart part
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(ALICE),
				0,                                            // base ID
				101,                                          // slot ID
				EquippableList::Custom(bvec![5, 6, 7, 8, 9]), // equippable collections
			),
			Error::<Test>::NoEquippableOnFixedPart
		);

		// Should not be able to change equippable on non-issued base
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(BOB),
				0,                                      // base ID
				201,                                    // slot ID
				EquippableList::Custom(bvec![3, 4, 5]), // equippable collections
			),
			Error::<Test>::PermissionError
		);

		// Blanking out equippable (setting to []) works
		assert_ok!(RmrkEquip::equippable(
			Origin::signed(ALICE),
			0,                     // base ID
			202,                   // slot ID
			EquippableList::Empty, // equippable collections
		));

		// Check storage

		// Setting equippable to * works
		assert_ok!(RmrkEquip::equippable(
			Origin::signed(ALICE),
			0,                   // base ID
			202,                 // slot ID
			EquippableList::All, // equippable collections
		));

		// Question: Should be check existence of collections being equipped?
	});
}

/// Base: Basic theme_add tests
#[test]
fn theme_add_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Define a non-default theme
		let non_default_theme = Theme {
			name: stb("doglover"),
			properties: vec![
				ThemeProperty { key: stb("sound"), value: stb("woof") },
				ThemeProperty { key: stb("secondary_color"), value: stb("blue") },
			],
			inherit: false,
		};

		// Attempt to add theme (should fail: Base must exist)
		assert_noop!(
			RmrkEquip::theme_add(
				Origin::signed(ALICE),
				0, // BaseID
				non_default_theme.clone()
			),
			Error::<Test>::BaseDoesntExist
		);

		// Build a base
		assert_ok!(RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			bvec![0u8; 20],        // base_type
			bvec![0u8; 20],        // symbol
			bvec![],
		));

		// Add non-default theme to base (should fail w/o default)
		assert_noop!(
			RmrkEquip::theme_add(
				Origin::signed(ALICE),
				0, // BaseID
				non_default_theme.clone()
			),
			Error::<Test>::NeedsDefaultThemeFirst
		);

		// Define a default theme
		let default_theme = Theme {
			name: stb("default"),
			properties: vec![
				ThemeProperty { key: stb("primary_color"), value: stb("red") },
				ThemeProperty { key: stb("secondary_color"), value: stb("blue") },
			],
			inherit: false,
		};

		// Attempt to add default theme (should fail: Signer must be issuer of base)
		assert_noop!(
			RmrkEquip::theme_add(
				Origin::signed(BOB),
				0, // BaseID
				default_theme.clone()
			),
			Error::<Test>::PermissionError
		);

		// Add default theme to base
		assert_ok!(RmrkEquip::theme_add(
			Origin::signed(ALICE),
			0, // BaseID
			default_theme
		));

		// Add non-default theme to base (should succeed)
		assert_ok!(RmrkEquip::theme_add(
			Origin::signed(ALICE),
			0, // BaseID
			non_default_theme
		));

		assert_eq!(
			RmrkEquip::themes((0, stb("default"), stb("primary_color"))).unwrap(),
			stb("red")
		);

		assert_eq!(
			RmrkEquip::themes((0, stb("default"), stb("secondary_color"))).unwrap(),
			stb("blue")
		);

		// Base must exist
		// Caller must be issuer of base
		// "default" must exist first

		// Question: do we need a cap on number of properties?
		// - Pretty sure

		// Do we want to automatically override a theme, or error when already exists?
		// - If error, we want some mechanism to remove a theme
	});
}

/// Theme add fails when too many properties
#[test]
fn theme_add_too_many_properties_fails() {
	ExtBuilder::default().build().execute_with(|| {
		// Build a base
		assert_ok!(RmrkEquip::create_base(
			Origin::signed(ALICE), // origin
			bvec![0u8; 20],        // base_type
			bvec![0u8; 20],        // symbol
			bvec![],
		));

		// Define a default theme with too many properties (10)
		let default_theme = Theme {
			name: stb("default"),
			properties: vec![
				ThemeProperty { key: stb("1"), value: stb("red") },
				ThemeProperty { key: stb("2"), value: stb("blue") },
				ThemeProperty { key: stb("3"), value: stb("red") },
				ThemeProperty { key: stb("4"), value: stb("blue") },
				ThemeProperty { key: stb("5"), value: stb("red") },
				ThemeProperty { key: stb("6"), value: stb("blue") },
				ThemeProperty { key: stb("7"), value: stb("red") },
				ThemeProperty { key: stb("8"), value: stb("blue") },
				ThemeProperty { key: stb("9"), value: stb("red") },
				ThemeProperty { key: stb("10"), value: stb("blue") },
			],
			inherit: false,
		};

		// Add default theme to base should fail (too many properties)
		assert_noop!(
			RmrkEquip::theme_add(
				Origin::signed(ALICE),
				0, // BaseID
				default_theme
			),
			Error::<Test>::TooManyProperties
		);
	});
}
