use frame_support::{
	assert_noop,
	assert_ok,
	// error::BadOrigin
};
// use sp_runtime::AccountId32;
use sp_runtime::Permill;
// use crate::types::ClassType;

use super::*;
use mock::{Event as MockEvent, *};
use pallet_uniques as UNQ;
use sp_std::{convert::TryInto, vec::Vec};

type RMRKCore = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, ValueLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedResource
fn stbr(s: &str) -> BoundedResource<ResourceSymbolLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedVec
fn stbk(s: &str) -> BoundedVec<u8, KeyLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a Vec
fn stv(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Shortcut for a test collection creation (Alice is issue, max NFTs is 5)
fn basic_collection() -> DispatchResult {
	RMRKCore::create_collection(Origin::signed(ALICE), bvec![0u8; 20], Some(5), bvec![0u8; 15])
}

/// Shortcut for a basic mint (Alice owner, Collection ID 0, Royalty 1.525)
fn basic_mint() -> DispatchResult {
	RMRKCore::mint_nft(
		Origin::signed(ALICE),
		ALICE,
		COLLECTION_ID_0,
		Some(ALICE),
		Some(Permill::from_float(1.525)),
		bvec![0u8; 20],
	)
}

// Tests ordered as follows:
// Collection: create, lock, destroy, changeissuer
// NFT: mint, send, burn
// Resource: create, add, accept
// Property: set
// Priority: set

/// Collection: Basic collection tests (RMRK2.0 spec: CREATE)
#[test]
fn create_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Creating collection should trigger CollectionCreated event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionCreated {
			issuer: ALICE,
			collection_id: 0,
		}));
		// Reassign CollectionIndex to max value
		CollectionIndex::<Test>::mutate(|id| *id = CollectionId::max_value());
		// Creating collection above max_value of CollectionId (4294967295) should fail
		assert_noop!(
			RMRKCore::create_collection(
				Origin::signed(ALICE),
				bvec![0u8; 20],
				None,
				bvec![0u8; 15],
			),
			Error::<Test>::NoAvailableCollectionId
		);
	});
}

/// Collection: Locking collection tests (RMRK2.0 spec: LOCK)
#[test]
fn lock_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection (has 5 max)
		assert_ok!(basic_collection());
		// Mint 4 NFTs
		for _ in 0..4 {
			assert_ok!(basic_mint());
		}
		// Lock collection
		assert_ok!(RMRKCore::lock_collection(Origin::signed(ALICE), 0));
		// Locking collection should trigger CollectionLocked event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionLocked {
			issuer: ALICE,
			collection_id: 0,
		}));
		// Attempt to mint in a locked collection should fail
		assert_noop!(basic_mint(), Error::<Test>::CollectionFullOrLocked);
		// Burn an NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Should now have only three NFTS in collection
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 3);
		// Still we should be unable to mint another NFT
		assert_noop!(basic_mint(), Error::<Test>::CollectionFullOrLocked);
	});
}

/// Collection: Destroy collection tests (RMRK2.0 spec: doesn't exist)
#[test]
fn destroy_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection (has 5 max)
		assert_ok!(basic_collection());
		// Mint an NFT
		assert_ok!(basic_mint());
		// Non-empty collection should not be able to be destroyed
		assert_noop!(
			RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0),
			Error::<Test>::CollectionNotEmpty
		);
		// Burn the single NFT in collection
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Empty collection can be destroyed
		assert_ok!(RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0));
		// Destroy event is triggered by successful destroy_collection
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionDestroyed {
			issuer: ALICE,
			collection_id: COLLECTION_ID_0,
		}));
	});
}

/// Collection: Change issuer tests (RMRK2.0 spec: CHANGEISSUER)=
#[test]
fn change_issuer_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Change issuer from ALICE to BOB
		assert_ok!(RMRKCore::change_issuer(Origin::signed(ALICE), 0, BOB));
		// Changing issuer should trigger IssuerChanged event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::IssuerChanged {
			old_issuer: ALICE,
			new_issuer: BOB,
			collection_id: 0,
		}));
		// New issuer should be Bob
		assert_eq!(RMRKCore::collections(0).unwrap().issuer, BOB);
	});
}

/// NFT: Basic Mint tests (RMRK2.0 spec: MINT)
#[test]
fn mint_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Minting an NFT should trigger an NftMinted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NftMinted {
			owner: ALICE,
			collection_id: 0,
			nft_id: 0,
		}));
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20]
		));
		// BOB shouldn't be able to mint in ALICE's collection
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(BOB),
				BOB,
				COLLECTION_ID_0,
				Some(CHARLIE),
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20]
			),
			Error::<Test>::NoPermission
		);
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 2);
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(ALICE),
				ALICE,
				NOT_EXISTING_CLASS_ID,
				Some(CHARLIE),
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20]
			),
			Error::<Test>::CollectionUnknown
		);
	});
}

/// NFT: Mint tests with max (RMRK2.0 spec: MINT)
#[test]
fn mint_collection_max_logic_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint 5 NFTs (filling collection)
		for _ in 0..5 {
			assert_ok!(basic_mint());
		}
		// Minting beyond collection max (5) should fail
		assert_noop!(basic_mint(), Error::<Test>::CollectionFullOrLocked);
		// Burn an NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, 0));
		// Minting should still fail, as burning should not affect "fullness" of collection
		assert_noop!(basic_mint(), Error::<Test>::CollectionFullOrLocked);
	});
}

/// NFT: Send tests (RMRK2.0 spec: SEND)
#[test]
fn send_nft_to_minted_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2)
		for _ in 0..3 {
			assert_ok!(basic_mint());
		}
		// ALICE sends NFT (0, 0) [parent] to Bob
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		));
		// Successful send triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::AccountId(BOB),
			collection_id: 0,
			nft_id: 0,
			approval_required: false,
		}));
		// ALICE sends NFT (0, 1) [child] to BOB-owned NFT (0, 0) [parent]
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Successful send to NFT triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
			collection_id: 0,
			nft_id: 1,
			approval_required: true,
		}));
		// Bob accepts NFT (0,1) for Bob-owned NFT (0,0)
		assert_ok!(RMRKCore::accept_nft(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Successful send triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTAccepted {
			sender: BOB,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
			collection_id: 0,
			nft_id: 1,
		}));
		// ALICE sends NFT (0, 2) [child] to BOB-owned NFT (0, 0) [parent]
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));
		// Successful send to NFT triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
			collection_id: 0,
			nft_id: 2,
			approval_required: true,
		}));
		// Bob accepts NFT (0,2) for Bob-owned NFT (0,1)
		assert_ok!(RMRKCore::accept_nft(
			Origin::signed(BOB),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));
		// Successful send triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTAccepted {
			sender: BOB,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
			collection_id: 0,
			nft_id: 2,
		}));
		// Bob-rootowned NFT (0,1) [child] is owned by Bob-rootowned NFT (0,0) [parent]
		assert_eq!(UNQ::Pallet::<Test>::owner(0, 1), Some(RMRKCore::nft_to_account_id(0, 0)),);
		// NFT (0,0) has NFT (0,1) in Children StorageMap
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 1)]);
		// Attempt to send NFT to self should fail
		assert_noop!(
			RMRKCore::send(
				Origin::signed(BOB),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
			),
			Error::<Test>::CannotSendToDescendentOrSelf
		);
		// Attempt to send NFT to its own descendent should fail
		assert_noop!(
			RMRKCore::send(
				Origin::signed(BOB),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2)
			),
			Error::<Test>::CannotSendToDescendentOrSelf
		);
		// BOB now root-owns NFT (0, 1) [child] (originally was ALICE)
		if let Ok((root_owner, _)) = RMRKCore::lookup_root_owner(0, 1) {
			assert_eq!(root_owner, BOB);
		}
		// Sending NFT that is not root-owned should fail
		assert_noop!(
			RMRKCore::send(
				Origin::signed(CHARLIE),
				0,
				0,
				AccountIdOrCollectionNftTuple::AccountId(BOB)
			),
			Error::<Test>::NoPermission
		);
		// Sending non-existent NFT should fail
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				666,
				666,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
			),
			Error::<Test>::NoAvailableNftId
		);
		// Root-owner (Bob) can send child NFT to another account
		assert_ok!(RMRKCore::send(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		));
		// Sending to non-existent NFT should fail
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				0,
				1,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(666, 666)
			),
			Error::<Test>::NoAvailableNftId
		);
	});
}

/// NFT: Reject tests (RMRK2.0 spec: new)
#[test]
fn reject_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2)
		for _ in 0..3 {
			assert_ok!(basic_mint());
		}
		// ALICE sends NFT (0, 0) [parent] to Bob
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		));
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		));
		// ALICE sends NFT (0, 2) to Bob-owned NFT (0,0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Bob rejects NFT (0,2) for Bob-owned NFT (0,0)
		assert_ok!(RMRKCore::reject_nft(
			Origin::signed(BOB),
			0,
			2,
		));
	});
}

/// NFT: Send tests, siblings (RMRK2.0 spec: SEND)
#[test]
fn send_two_nfts_to_same_nft_creates_two_children() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2)
		for _ in 0..3 {
			assert_ok!(basic_mint());
		}
		// ALICE sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// NFT (0,0) has NFT (0,1) in Children StorageMap
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 1)]);
		// ALICE sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// NFT (0,0) has NFT (0,1) & (0,2) in Children StorageMap
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 1), (0, 2)]);
	});
}

/// NFT: Send tests, removing parent (RMRK2.0 spec: SEND)
#[test]
fn send_nft_removes_existing_parent() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2), (0, 3)
		for _ in 0..4 {
			assert_ok!(basic_mint());
		}
		// ALICE sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// ALICE sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// NFT (0, 0) is parent of NFT (0, 1)
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 1), (0, 2)]);
		// ALICE sends NFT (0, 1) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));
		// NFT (0, 0) is no longer parent of NFT (0, 1)
		assert_eq!(RMRKCore::children((0, 0)), vec![(0, 2)]);
	});
}

/// NFT: Send tests, multi-generational circular testing (RMRK2.0 spec: SEND)
#[test]
fn send_to_grandchild_fails() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2)
		for _ in 0..3 {
			assert_ok!(basic_mint());
		}
		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Alice sends NFT (0, 2) to NFT (0, 1)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));
		// Sending NFT to its own grandchild should fail
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
			),
			Error::<Test>::CannotSendToDescendentOrSelf
		);
	});
}

/// NFT: Burn simple tests (RMRK2.0 spec: BURN)
#[test]
fn burn_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint an NFT
		assert_ok!(basic_mint());
		// BOB should not be able to burn ALICE's NFT
		assert_noop!(
			RMRKCore::burn_nft(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0),
			Error::<Test>::NoPermission
		);
		// ALICE burns her NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Successful burn creates NFTBurned event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTBurned {
			owner: ALICE,
			nft_id: 0,
		}));
		// NFT count of collection is now 0
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// ALICE can't burn an NFT twice
		assert_noop!(
			RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0),
			Error::<Test>::NoAvailableNftId
		);
		// Burned NFT no longer exists
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, NFT_ID_0).is_none(), true);
	});
}

/// NFT: Burn complex multi-generational tests (RMRK2.0 spec: BURN)
#[test]
fn burn_nft_with_great_grandchildren_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2), (0, 3)
		for _ in 0..4 {
			assert_ok!(basic_mint());
		}
		// ALICE sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// ALICE sends NFT (0, 2) to NFT (0, 1)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1),
		));
		// ALICE sends NFT (0, 3) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			3,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));
		// Great-grandchild NFT (0, 3) exists
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_some(), true);
		// Burn great-grandparent NFT (0, 0)
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Great-grandchild NFT (0, 3) is dead :'-(
		assert!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_none())
	});
}

/// Resource: Basic resource addition (RMRK2.0 spec: RESADD)
#[test]
fn create_resource_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Adding a resource to non-existent NFT should fail
		assert_noop!(
			RMRKCore::add_resource(
				Origin::signed(ALICE),
				0, // collection_id
				0, // nft_id
				stbr("res-1"), // resource_id
				Some(0), // base_id
				None, // src
				None, // metadata
				None, // slot
				None, // license
				None, // thumb
				None, // parts
			),
			Error::<Test>::NoAvailableNftId
		);
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint());
		// Adding an empty resource should fail
		assert_noop!(
			RMRKCore::add_resource(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				stbr("res-2"), // resource_id
				None, // base_id
				None, // src
				None, // metadata
				None, // slot
				None, // license
				None, // thumb
				None, // parts
			),
			Error::<Test>::EmptyResource
		);
		// Add resource to NFT
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-3"), // resource_id
			Some(0), // base_id
			None, // src
			None, // metadata
			None, // slot
			None, // license
			None, // thumb
			None, // parts
		));
		// Successful resource addition should trigger ResourceAdded event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAdded {
			nft_id: 0,
			resource_id: stbr("res-3"), // resource_id
		}));
		// Since ALICE rootowns NFT, pending status of resource should be false
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-3"))).unwrap().pending, false);
	});
}


/// Resource: Resource addition with pending and accept (RMRK2.0 spec: ACCEPT)
#[test]
fn add_resource_pending_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint());
		// BOB adds a resource to ALICE's NFT
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-4"), // resource_id
			Some(0), // base_id
			None, // src
			None, // metadata
			None, // slot
			None, // license
			None, // thumb
			None, // parts
		));
		// Since BOB doesn't root-own NFT, resource's pending status should be true
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-4"))).unwrap().pending, true);
		// BOB doesn't own ALICES's NFT, so accept should fail
		assert_noop!(
			RMRKCore::accept(Origin::signed(BOB), 0, 0, stbr("res-4")),
			Error::<Test>::NoPermission);
		// ALICE can accept her own NFT's pending resource
		assert_ok!(RMRKCore::accept(Origin::signed(ALICE), 0, 0, stbr("res-4")));
		// Valid resource acceptance should trigger a ResourceAccepted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAccepted {
			nft_id: 0,
			resource_id: stbr("res-4"), // resource_id
		}));
		// Resource should now have false pending status
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-4"))).unwrap().pending, false);
	});
}

/// Property: Setting property tests (RMRK2.0 spec: SETPROPERTY)
#[test]
fn set_property_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Define property key
		let key = stbk("test-key");
		// Define property value
		let value = stb("test-value");
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint());
		// ALICE sets property on NFT
		assert_ok!(RMRKCore::set_property(
			Origin::signed(ALICE),
			0,
			Some(0),
			key.clone(),
			value.clone()
		));
		// Successful property setting should trigger a PropertySet event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PropertySet {
			collection_id: 0,
			maybe_nft_id: Some(0),
			key: key.clone(),
			value: value.clone(),
		}));
		// Property value now exists
		assert_eq!(RMRKCore::properties((0, Some(0), key.clone())).unwrap(), value.clone());
		// BOB does not own NFT so attempt to set property should fail
		assert_noop!(
			RMRKCore::set_property(Origin::signed(BOB), 0, Some(0), key.clone(), value.clone()),
			Error::<Test>::NoPermission
		);
	});
}

/// Priority: Setting priority tests (RMRK2.0 spec: SETPRIORITY)
#[test]
fn set_priority_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint());
		// ALICE sets priority on NFT
		assert_ok!(RMRKCore::set_priority(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			vec![stv("hello"), stv("world")]
		));
		// Successful priority set should trigger PrioritySet event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PrioritySet {
			collection_id: 0,
			nft_id: 0,
		}));
		// Priorities exist
		assert_eq!(
			RMRKCore::priorities(COLLECTION_ID_0, NFT_ID_0).unwrap(),
			vec![stv("hello"), stv("world")]
		);
	});
}
