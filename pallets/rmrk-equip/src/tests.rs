use frame_support::{
	assert_noop, 
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
	// println!("len: {:?}", s.as_bytes().to_vec().len());
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
			equippable: EquippableList::Custom(vec![
				0, // Collection 0
				1, // Collection 1
			]),
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
			equippable: EquippableList::Custom(vec![
				0, // Collection 0
				1, // Collection 1
			]),
		};
		// Slot part right hand can equip items from collections 2 or 3
		let slot_part_right_hand = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: EquippableList::Custom(vec![
				0, // Collection 2
				1, // Collection 3
			]),
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

		// Mint NFT 0 from collection 0 (character-0)
		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE, // owner
			0, // collection ID
			Some(ALICE), // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://character-0-metadata"), // metadata
		);

		// Mint NFT 1 from collection 0 (character-1)
		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE, // owner
			0, // collection ID
			Some(ALICE), // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://character-1-metadata"), // metadata
		);

		// Mint NFT 0 from collection 1 (sword)
		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE, // owner
			1, // collection ID
			Some(ALICE), // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://sword-metadata"), // metadata
		);

		// Mint NFT 1 from collection 1 (flashlight)
		RmrkCore::mint_nft(
			Origin::signed(ALICE),
			ALICE, // owner
			1, // collection ID
			Some(ALICE), // recipient
			Some(Permill::from_float(1.525)), // royalties
			stb("ipfs://flashlight-metadata"), // metadata
		);

		// Attempt to equip sword should fail as character-0 doesn't own sword
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				201, // SlotId
			),
			Error::<Test>::MustBeDirectParent
		);

		// Sends NFT (0, 1) [sword] to NFT (0, 0) [character-0]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			1, // Collection ID
			0, // NFT ID
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0), // Recipient
		));

		// Attempt to equip sword should fail as character-0 doesn't have a resource that is associated with this base
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				201, // SlotId
			),
			Error::<Test>::NoResourceForThisBaseFoundOnNft
		);

		// Add a Base 0 resource (body-1 and left-hand slot) to our character-0 nft
		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			0, // collection_id
			0, // nft id
			// Some(0), // base id
			ResourceType::Slot(
				ComposableResource { //<BaseId, SlotId, ResourceId, BoundedString> {
					base: 0, // pub base: BaseId,
					id: 1000, // pub id: ResourceId,
					parts: vec![
						101, // ID of body-1 part
						201, // ID of left-hand slot
					],
					src: Some(stb("ipfs://backup-src")), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
				}
			),
		));

		// Attempt to equip sword should fail as the sword doesn't have a resource that is equippable into that slot
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				201, // SlotId
			),
			Error::<Test>::ItemHasNoResourceToEquipThere
		);

		// Add our sword left-hand resource to our sword NFT
		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			1, // collection id
			0, // nft id
			// None, // base id
			ResourceType::Base(
				NoncomposableResource { 
					base: 0, // Base ID this resource can be equipped into
					slot_id: 201, // Slot ID this resource can be equipped into (left hand)
					id: 777, // pub id: ResourceId,
					src: stb("ipfs://sword-metadata-left"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					theme_id: None, // pub themeId: Option<BoundedString>,
				}
			),
		));

		// Equipping should now work
		assert_ok!(RmrkEquip::equip(
			Origin::signed(ALICE), // Signer
			1, // Item CollectionId
			0, // Item NftId
			0, // Equipper CollectionId
			0, // Equipper NftId
			0, // BaseId
			201, // SlotId
		));

		System::assert_last_event(MockEvent::RmrkEquip(crate::Event::SlotEquipped {
			item_collection: 1,
			item_nft: 0,
			base_id: 0,
			slot_id: 201,
		}));

		// Equipped resource ID Some(777) should now be associated with equippings for character-0 on base 0, slot 201
		let equipped = RmrkEquip::equippings(((0, 0), 0, 201));
		assert_eq!(
			equipped,
			Some(777)
		);

		// Resource for equipped item should exist
		assert!(RmrkCore::new_resources((1, 0, equipped.unwrap())).is_some());

		// Add our sword right-hand resource to our sword NFT
		assert_ok!(RmrkCore::new_add_resource(
			Origin::signed(ALICE),
			1, // collection id
			0, // nft id
			// None, // base id
			ResourceType::Base(
				NoncomposableResource { 
					base: 0, // Base ID this resource can be equipped into
					slot_id: 202, // Slot ID this resource can be equipped into (right hand)
					id: 778, // pub id: ResourceId,
					src: stb("ipfs://sword-metadata-right"), // pub src: BoundedString,
					thumb: None, // pub thumb: Option<BoundedString>,
					theme_id: None, // pub themeId: Option<BoundedString>,
				}
			),
		));
		
		// Equipping to right-hand should fail (already equipped in left hand)
		assert_noop!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				202, // SlotId
			),
			Error::<Test>::AlreadyEquipped
		);

		// Unequipping from left-hand should work
		assert_ok!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				201, // SlotId
			)
		);

		System::assert_last_event(MockEvent::RmrkEquip(crate::Event::SlotUnequipped {
			item_collection: 1,
			item_nft: 0,
			base_id: 0,
			slot_id: 201,
		}));

		// Re-equipping to left-hand should work
		assert_ok!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				201, // SlotId
			)

		);

		// Unequipping from left-hand should work
		assert_ok!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				201, // SlotId
			)
		);

		// Equipping to right-hand should work
		assert_ok!(
			RmrkEquip::equip(
				Origin::signed(ALICE), // Signer
				1, // Item CollectionId
				0, // Item NftId
				0, // Equipper CollectionId
				0, // Equipper NftId
				0, // BaseId
				202, // SlotId
			)
		);



	});
}
/// Base: Basic equip tests
#[test]
fn equippable_works() {
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
			equippable: EquippableList::Custom(vec![
				0, // Collection 0
				1, // Collection 1
			])
		};
		// Slot part right hand can equip items from collections 2 or 3
		let slot_part_right_hand = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: EquippableList::Custom(vec![
				2, // Collection 2
				3, // Collection 3
			])
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

		// equippable extrinsic should work
		assert_ok!(RmrkEquip::equippable(
			Origin::signed(ALICE),
			0, // base ID
			202, // slot ID
			EquippableList::Custom(vec![5, 6, 7]), // equippable collections
		));

		// Parts storage should be updated
		let should_be = SlotPart {
			id: 202,
			z: 0,
			src: stb("right-hand"),
			equippable: EquippableList::Custom(vec![5, 6, 7]),
		};
		assert_eq!(
			RmrkEquip::parts(0, 202).unwrap(),
			NewPartTypes::SlotPart(should_be)
		);

		// Should not be able to change equippable on non-existent base
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(ALICE),
				666, // base ID
				202, // slot ID
				EquippableList::Custom(vec![5, 6, 7]), // equippable collections
			),
			Error::<Test>::BaseDoesntExist
		);

		// Should not be able to change equippable on non-existent part
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(ALICE),
				0, // base ID
				200, // slot ID
				EquippableList::Custom(vec![5, 6, 7]), // equippable collections
			),
			Error::<Test>::PartDoesntExist
		);

		// Should not be able to change equippable on FixedPart part
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(ALICE),
				0, // base ID
				101, // slot ID
				EquippableList::Custom(vec![5, 6, 7, 8, 9]), // equippable collections
			),
			Error::<Test>::NoEquippableOnFixedPart
		);

		// Should not be able to change equippable on non-issued base
		assert_noop!(
			RmrkEquip::equippable(
				Origin::signed(BOB),
				0, // base ID
				201, // slot ID
				EquippableList::Custom(vec![3, 4, 5]), // equippable collections
			),
			Error::<Test>::PermissionError
		);

		// Blanking out equippable (setting to []) works
		assert_ok!(RmrkEquip::equippable(
			Origin::signed(ALICE),
			0, // base ID
			202, // slot ID
			EquippableList::Empty, // equippable collections
		));

		// Check storage

		// Setting equippable to * works
		assert_ok!(RmrkEquip::equippable(
			Origin::signed(ALICE),
			0, // base ID
			202, // slot ID
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
				ThemeProperty {
					key: stb("sound"),
					value: stb("woof"),
					inherit: Some(true),
				},
				ThemeProperty {
					key: stb("secondary_color"),
					value: stb("blue"),
					inherit: None,
				}
			]
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
			bvec![0u8; 20], // base_type
			bvec![0u8; 20], // symbol
			vec![],
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
				ThemeProperty {
					key: stb("primary_color"),
					value: stb("red"),
					inherit: None,
				},
				ThemeProperty {
					key: stb("secondary_color"),
					value: stb("blue"),
					inherit: None,
				}
			]
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