use frame_support::{assert_noop, assert_ok};
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

/// Collection: Creating collection with None max doesn't prevent NFTs from being minted
#[test]
fn create_collection_no_max_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a collection with max of None
		assert_ok!(RMRKCore::create_collection(
			Origin::signed(ALICE),
			bvec![0u8; 20],
			None,
			bvec![0u8; 15]
		));
		// Creating collection should trigger CollectionCreated event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::CollectionCreated {
			issuer: ALICE,
			collection_id: 0,
		}));
		// Mint 100 NFTs
		for _ in 0..100 {
			assert_ok!(basic_mint());
		}
		// Last event should be the 100th NFT creation
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NftMinted {
			owner: ALICE,
			collection_id: 0,
			nft_id: 99,
		}));
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
		// Lock collection won't work with BOB
		assert_noop!(
			RMRKCore::lock_collection(Origin::signed(BOB), 0),
			Error::<Test>::NoPermission
		);
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
fn change_collection_issuer_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// BOB can't change issuer because he is not the current issuer
		assert_noop!(
			RMRKCore::change_collection_issuer(Origin::signed(BOB), 0, BOB),
			Error::<Test>::NoPermission
		);
		// Change issuer from ALICE to BOB
		assert_ok!(RMRKCore::change_collection_issuer(Origin::signed(ALICE), 0, BOB));
		// Changing issuer should trigger IssuerChanged event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::IssuerChanged {
			old_issuer: ALICE,
			new_issuer: BOB,
			collection_id: 0,
		}));
		// New issuer should be Bob
		assert_eq!(RMRKCore::collections(0).unwrap().issuer, BOB);
		// BOB can't change issuer if calls transfer_ownership in uniques
		assert_noop!(
			UNQ::Pallet::<Test>::transfer_ownership(Origin::signed(ALICE), 0, ALICE),
			UNQ::Error::<Test>::NoPermission
		);
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

/// NFT: Royalty defaults to self when amount provided but no recipient
#[test]
fn royalty_recipient_default_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint an NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			None, // No royalty recipient
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20]
		));
		// Royalty recipient should default to issuer (ALICE)
		assert_eq!(RmrkCore::nfts(0, 0).unwrap().royalty.unwrap().recipient, ALICE);
		// Mint another NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(BOB), // Royalty recipient is BOB
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20]
		));
		// Royalty recipient should be BOB
		assert_eq!(RmrkCore::nfts(0, 1).unwrap().royalty.unwrap().recipient, BOB);
		// Mint another NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			None, // No royalty recipient is BOB
			None, // No royalty amount
			bvec![0u8; 20]
		));
		// Royalty should not exist
		assert!(RmrkCore::nfts(0, 2).unwrap().royalty.is_none());
		// Mint another NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE), // Royalty recipient is ALICE
			None, // No royalty amount
			bvec![0u8; 20]
		));
		// Royalty should not exist
		assert!(RmrkCore::nfts(0, 3).unwrap().royalty.is_none());
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
		assert!(RMRKCore::children((0, 0), (0, 1)).is_some());
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
		// ALICE sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// ALICE sends NFT (0, 2) to BOB 
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		));
		// ALICE sends NFT (0, 0) to Bob-owned NFT (0,2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));
		// Bob rejects NFT (0,2) for Bob-owned NFT (0,0)
		assert_ok!(RMRKCore::reject_nft(Origin::signed(BOB), 0, 2,));
		// Rejected NFT gets burned
		assert_eq!(RMRKCore::nfts(0, 0).is_none(), true);
		// Child is burned if parent is rejected
		assert_eq!(RMRKCore::nfts(0, 1).is_none(), true);
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
		assert!(RMRKCore::children((0, 0), (0, 1)).is_some());
		// ALICE sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// NFT (0,0) has NFT (0,1) & (0,2) in Children StorageMap
		assert!(RMRKCore::children((0, 0), (0,1)).is_some());
		assert!(RMRKCore::children((0, 0), (0,2)).is_some());
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
		assert!(RMRKCore::children((0, 0), (0, 1)).is_some());
		assert!(RMRKCore::children((0, 0), (0, 2)).is_some());
		// ALICE sends NFT (0, 1) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));
		// NFT (0, 0) is no longer parent of NFT (0, 1)
		assert!(RMRKCore::children((0, 0), (0, 1)).is_none());
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
		// Add two resources to NFT (to test if burning also burns the resources)

		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			0,
			0,
			stbr("res-1"),
			Some(0),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			None,
			None,
			None,
			None
		));

		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			0,
			0,
			stbr("res-2"),
			Some(0),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			None,
			None,
			None,
			None
		));

		// Ensure resources are there
		assert_eq!(Resources::<Test>::iter_prefix((COLLECTION_ID_0, NFT_ID_0)).count(), 2);

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
		// Resources associated with the NFT should no longer exist
		assert_eq!(Resources::<Test>::iter_prefix((COLLECTION_ID_0, NFT_ID_0)).count(), 0);
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
		// Add two resources to the great-grandchild (0, 3)
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			3,
			stbr("res-1"),
			Some(0),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			None,
			None,
			None,
			None
		));

		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			3,
			stbr("res-2"),
			Some(0),
			Some(bvec![0u8; 20]),
			Some(bvec![0u8; 20]),
			None,
			None,
			None,
			None
		));

		// Ensure resources are there
		assert_eq!(Resources::<Test>::iter_prefix((COLLECTION_ID_0, 3)).count(), 2);

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
		assert!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_none());
		// Great-grandchild resources are gone
		assert_eq!(Resources::<Test>::iter_prefix((COLLECTION_ID_0, 3)).count(), 0);
	});
}

/// NFT: Burn beyond max_recursions fails gracefully
#[test]
fn burn_nft_beyond_max_recursions_fails_gracefully() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2), (0, 3)
		for _ in 0..5 {
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
		// ALICE sends NFT (0, 4) to NFT (0, 3)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			4,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 3),
		));
		// All NFTs exist
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 0).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 1).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 2).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 4).is_some(), true);
		// Burn great-grandparent NFT (0, 0)
		assert_noop!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0), Error::<Test>::TooManyRecursions);
		// All NFTs still exist
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 0).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 1).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 2).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 4).is_some(), true);
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
				0,             // collection_id
				0,             // nft_id
				stbr("res-1"), // resource_id
				Some(0),       // base_id
				None,          // src
				None,          // metadata
				None,          // slot
				None,          // license
				None,          // thumb
				None,          // parts
			),
			Error::<Test>::CollectionUnknown
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
				None,          // base_id
				None,          // src
				None,          // metadata
				None,          // slot
				None,          // license
				None,          // thumb
				None,          // parts
			),
			Error::<Test>::EmptyResource
		);
		// Add resource to NFT
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-3"), // resource_id
			Some(0),       // base_id
			None,          // src
			None,          // metadata
			None,          // slot
			None,          // license
			None,          // thumb
			None,          // parts
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
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			BOB,
			COLLECTION_ID_0,
			Some(BOB),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
		));
		// Since BOB isn't collection issuer, he can't add resources
		assert_noop!(
			RMRKCore::add_resource(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				stbr("res-4"), // resource_id
				Some(0),       // base_id
				None,          // src
				None,          // metadata
				None,          // slot
				None,          // license
				None,          // thumb
				None,          // parts
			),
			Error::<Test>::NoPermission
		);
		// Collection issuer can add resource
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-4"), // resource_id
			Some(0),       // base_id
			None,          // src
			None,          // metadata
			None,          // slot
			None,          // license
			None,          // thumb
			None,          // parts
		));
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-4"))).unwrap().pending, true);
		// ALICE doesn't own BOB's NFT, so accept should fail
		assert_noop!(
			RMRKCore::accept_resource(Origin::signed(ALICE), 0, 0, stbr("res-4")),
			Error::<Test>::NoPermission
		);
		// BOB can accept his own NFT's pending resource
		assert_ok!(RMRKCore::accept_resource(Origin::signed(BOB), 0, 0, stbr("res-4")));
		// Valid resource acceptance should trigger a ResourceAccepted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAccepted {
			nft_id: 0,
			resource_id: stbr("res-4"), // resource_id
		}));
		// Resource should now have false pending status
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-4"))).unwrap().pending, false);
		// Accepting resource again should fail with ResourceNotPending
		assert_noop!(
			RMRKCore::accept_resource(Origin::signed(BOB), 0, 0, stbr("res-4")),
			Error::<Test>::ResourceNotPending
		);
	});
}

/// Resource: Basic resource removal
#[test]
fn resource_removal_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint());
		// Add resource to NFT
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-0"), // resource_id
			Some(0),       // base_id
			None,          // src
			None,          // metadata
			None,          // slot
			None,          // license
			None,          // thumb
			None,          // parts
		));
		// Resource res-1 doesn't exist
		assert_noop!(
			RMRKCore::remove_resource(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				stbr("res-1"), // resource_id
			),
			Error::<Test>::ResourceDoesntExist
		);
		// Only collection issuer can request resource removal
		assert_noop!(
			RMRKCore::remove_resource(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				stbr("res-0"), // resource_id
			),
			Error::<Test>::NoPermission
		);
		// Remove resource
		assert_ok!(RMRKCore::remove_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-0"), // resource_id
		));
		// Successful resource removal should trigger ResourceRemoval event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceRemoval {
			nft_id: 0,
			resource_id: stbr("res-0"), // resource_id
		}));
		// Since ALICE rootowns NFT, resource should be removed
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-0"))), None);
	});
}

/// Resource: Resource removal with pending and accept
#[test]
fn resource_removal_pending_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			BOB,
			COLLECTION_ID_0,
			Some(BOB),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
		));
		// Add resource to NFT
		assert_ok!(RMRKCore::add_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-0"), // resource_id
			Some(0),       // base_id
			None,          // src
			None,          // metadata
			None,          // slot
			None,          // license
			None,          // thumb
			None,          // parts
		));
		assert_ok!(RMRKCore::accept_resource(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-0"),
		));
		// Accepting a resource removal that is not pending should fail
		assert_noop!(
			RMRKCore::accept_resource_removal(Origin::signed(BOB), 0, 0, stbr("res-0")),
			Error::<Test>::ResourceNotPending
		);
		// Only collection's issuer can request resource removal
		assert_noop!(
			RMRKCore::remove_resource(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				stbr("res-0"), // resource_id
			),
			Error::<Test>::NoPermission
		);
		// Resource removal requested by the collection issuer
		assert_ok!(RMRKCore::remove_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			stbr("res-0"), // resource_id
		));
		// Since ALICE doesn't root-own NFT, resource's removal is waiting for acceptance
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-0"))).unwrap().pending_removal, true);
		// ALICE doesn't own BOB's NFT, so accept should fail
		assert_noop!(
			RMRKCore::accept_resource_removal(Origin::signed(ALICE), 0, 0, stbr("res-0")),
			Error::<Test>::NoPermission
		);
		// Resource res-1 doesn't exist
		assert_noop!(
			RMRKCore::accept_resource_removal(Origin::signed(BOB), 0, 0, stbr("res-1")),
			Error::<Test>::ResourceDoesntExist
		);
		// BOB can accept his own NFT's pending resource removal
		assert_ok!(RMRKCore::accept_resource_removal(Origin::signed(BOB), 0, 0, stbr("res-0")));
		// Successful resource removal acceptance should trigger ResourceRemovalAccepted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceRemovalAccepted {
			nft_id: 0,
			resource_id: stbr("res-0"), // resource_id
		}));
		// Resource removed
		assert_eq!(RMRKCore::resources((0, 0, stbr("res-0"))), None);
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
		// BOB cannot set priority on NFT

		assert_noop!(
			RMRKCore::set_priority(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				bvec![100, 500]
			),
			Error::<Test>::NoPermission
		);
		// ALICE sets priority on NFT
		assert_ok!(RMRKCore::set_priority(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			bvec![100, 500] // BoundedVec Resource 0, 1
		));
		// Successful priority set should trigger PrioritySet event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PrioritySet {
			collection_id: 0,
			nft_id: 0,
		}));
		// Resource 100 should have priority 0
		assert_eq!(RMRKCore::priorities((COLLECTION_ID_0, NFT_ID_0, 100)).unwrap(), 0);
		// Resource 500 should have priority 1
		assert_eq!(RMRKCore::priorities((COLLECTION_ID_0, NFT_ID_0, 500)).unwrap(), 1);
		// Setting priority again drains and resets priorities
		assert_ok!(RMRKCore::set_priority(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			bvec![1000, 100] // BoundedVec Resources 2, 0
		));
		// Priorities reset, resource 100 should have priority one
		assert_eq!(RMRKCore::priorities((COLLECTION_ID_0, NFT_ID_0, 100)).unwrap(), 1);
		// Resource 1000 should have priority zero
		assert_eq!(RMRKCore::priorities((COLLECTION_ID_0, NFT_ID_0, 1000)).unwrap(), 0);
		// Resource 500 should no longer have a priority
		assert!(RMRKCore::priorities((COLLECTION_ID_0, NFT_ID_0, 500)).is_none(),);
	});
}

