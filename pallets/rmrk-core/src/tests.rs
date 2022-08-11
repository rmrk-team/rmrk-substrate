// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-core.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

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

/// Turns a string into a BoundedVec
fn stbd(s: &str) -> StringLimitOf<Test> {
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
fn basic_mint(id: u32) -> DispatchResult {
	RMRKCore::mint_nft(
		Origin::signed(ALICE),
		None, // if not specified defaults to minter
		id,
		COLLECTION_ID_0,
		Some(ALICE),
		Some(Permill::from_float(1.525)),
		bvec![0u8; 20],
		true,
		None,
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
		for id in 0..100 {
			assert_ok!(basic_mint(id));
		}
		// Last event should be the 100th NFT creation
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NftMinted {
			owner: AccountIdOrCollectionNftTuple::AccountId(ALICE),
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
		for id in 0..4 {
			assert_ok!(basic_mint(id));
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
		assert_noop!(basic_mint(5), Error::<Test>::CollectionFullOrLocked);
		// Burn an NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS));
		// Should now have only three NFTS in collection
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 3);
		// Still we should be unable to mint another NFT
		assert_ok!(basic_mint(5));
	});
}

/// Collection: Destroy collection tests (RMRK2.0 spec: doesn't exist)
#[test]
fn destroy_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection (has 5 max)
		assert_ok!(basic_collection());
		// Mint an NFT
		assert_ok!(basic_mint(0));
		// Non-empty collection should not be able to be destroyed
		assert_noop!(
			RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0),
			Error::<Test>::CollectionNotEmpty
		);
		// Burn the single NFT in collection
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS));
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
		// Must set ownership acceptance with BOB before transfer due to uniques dependency
		assert_ok!(Uniques::set_accept_ownership(Origin::signed(BOB), Some(0)));
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
			UNQ::Error::<Test>::Unaccepted
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
		assert_ok!(basic_mint(0));
		// Minting an NFT should trigger an NftMinted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NftMinted {
			owner: AccountIdOrCollectionNftTuple::AccountId(ALICE),
			collection_id: 0,
			nft_id: 0,
		}));
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			None,
			1,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			None,
		));
		// BOB shouldn't be able to mint in ALICE's collection
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(BOB),
				Some(BOB),
				2,
				COLLECTION_ID_0,
				Some(CHARLIE),
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20],
				true,
				None,
			),
			Error::<Test>::NoPermission
		);
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 2);
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(ALICE),
				Some(ALICE),
				NFT_ID_0,
				NOT_EXISTING_CLASS_ID,
				Some(CHARLIE),
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20],
				true,
				None,
			),
			Error::<Test>::CollectionUnknown
		);
		// Throw NftAlreadyExists when attempting to mint with the same id
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(ALICE),
				None,
				1,
				COLLECTION_ID_0,
				Some(ALICE),
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20],
				true,
				None,
			),
			Error::<Test>::NftAlreadyExists
		);
	});
}

/// NFT: Mint directly to NFT
#[test]
fn mint_directly_to_nft() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());

		// Mint directly to non-existent NFT fails
		assert_noop!(
			RMRKCore::mint_nft_directly_to_nft(
				Origin::signed(ALICE),
				(0, 0),
				NFT_ID_0,
				COLLECTION_ID_0,
				None,
				Some(Permill::from_float(20.525)),
				bvec![0u8; 20],
				true,
				None,
			),
			Error::<Test>::NoAvailableNftId
		);

		// ALICE mints an NFT for BOB
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(BOB),
			NFT_ID_0,
			COLLECTION_ID_0,
			None,
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			None,
		));

		// BOB owns NFT (0, 0)
		assert_eq!(
			RmrkCore::nfts(0, 0).unwrap().owner,
			AccountIdOrCollectionNftTuple::AccountId(BOB)
		);

		// ALICE mints NFT directly to BOB-owned NFT (0, 0)
		assert_ok!(RMRKCore::mint_nft_directly_to_nft(
			Origin::signed(ALICE),
			(0, 0),
			1,
			COLLECTION_ID_0,
			None,
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			None,
		));

		// Minted NFT (0, 1) exists
		assert!(RmrkCore::nfts(0, 1).is_some());

		// Minted NFT (0, 1) has owner NFT (0, 0)
		assert_eq!(
			RmrkCore::nfts(0, 1).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		);

		// Minted NFT (0, 1) is pending
		assert!(RmrkCore::nfts(0, 1).unwrap().pending);
	});
}

/// NFT: When minting directly to a non-owned NFT *with resources*, the resources should *not* be
/// pending
#[test]
fn mint_directly_to_nft_with_resources() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());

		// ALICE mints an NFT for BOB
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(BOB),
			NFT_ID_0,
			COLLECTION_ID_0,
			None,
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			None,
		));

		// Compose a resource to add to an NFT
		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Construct as a BoundedVec of resources which mint_nft will accept
		let resources_to_add =
			bvec![ResourceInfoMin { id: 0, resource: ResourceTypes::Basic(basic_resource) }];

		// ALICE mints NFT directly to BOB-owned NFT (0, 0), with the above resource
		assert_ok!(RMRKCore::mint_nft_directly_to_nft(
			Origin::signed(ALICE),
			(0, 0),
			1,
			COLLECTION_ID_0,
			None,
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			Some(resources_to_add),
		));

		// Created resource 0 on NFT (0, 1) should exist
		assert!(RmrkCore::resources((0, 1, 0)).is_some());

		println!("{:?}", RmrkCore::resources((0, 1, 0)).unwrap());
		// Created resource 0 on NFT (0, 1) should not be pending
		assert!(!RmrkCore::resources((0, 1, 0)).unwrap().pending);
	});
}

/// NFT: Mint tests with max (RMRK2.0 spec: MINT)
#[test]
fn mint_collection_max_logic_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint 5 NFTs (filling collection)
		for id in 0..5 {
			assert_ok!(basic_mint(id));
		}
		// Minting beyond collection max (5) should fail
		assert_noop!(basic_mint(5), Error::<Test>::CollectionFullOrLocked);
		// Burn one NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, 0, MAX_BURNS));
		// Minting is allowed
		assert_ok!(basic_mint(5));
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
			Some(ALICE),
			NFT_ID_0,
			COLLECTION_ID_0,
			None, // No royalty recipient
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			None,
		));
		// Royalty recipient should default to issuer (ALICE)
		assert_eq!(RmrkCore::nfts(0, 0).unwrap().royalty.unwrap().recipient, ALICE);
		// Mint another NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			1,
			COLLECTION_ID_0,
			Some(BOB), // Royalty recipient is BOB
			Some(Permill::from_float(20.525)),
			bvec![0u8; 20],
			true,
			None,
		));
		// Royalty recipient should be BOB
		assert_eq!(RmrkCore::nfts(0, 1).unwrap().royalty.unwrap().recipient, BOB);
		// Mint another NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			2,
			COLLECTION_ID_0,
			None, // No royalty recipient is BOB
			None, // No royalty amount
			bvec![0u8; 20],
			true,
			None,
		));
		// Royalty should not exist
		assert!(RmrkCore::nfts(0, 2).unwrap().royalty.is_none());
		// Mint another NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			3,
			COLLECTION_ID_0,
			Some(ALICE), // Royalty recipient is ALICE
			None,        // No royalty amount
			bvec![0u8; 20],
			true,
			None,
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
		for id in 0..3 {
			assert_ok!(basic_mint(id));
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

#[test]
fn send_non_transferable_fail() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint non-transferable NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			false, // non-transferable
			None,
		));
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				0,
				0,
				AccountIdOrCollectionNftTuple::AccountId(BOB)
			),
			Error::<Test>::NonTransferable
		);
	});
}

#[test]
fn mint_non_transferrable_gem_on_to_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());

		// Mint NFT (transferrable, will be the parent of a later-minted non-transferrable NFT)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(BOB),
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true, // transferable
			None,
		));

		// Mint non-transferable NFT *on to* Bob-owned NFT (0, 0)
		assert_ok!(RMRKCore::mint_nft_directly_to_nft(
			Origin::signed(ALICE),
			(0, 0),
			1,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			false, // non-transferable
			None,
		));

		// NFT (0, 1) exists and is non-transferrable
		assert!(!RMRKCore::nfts(0, 1).unwrap().transferable);

		// NFT (0, 1) is owned by BOB-owned NFT (0, 0)
		assert_eq!(
			RMRKCore::nfts(0, 1).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		);

		// BOB *cannot send* non-transferrable NFT (0, 1) to CHARLIE
		assert_noop!(
			RMRKCore::send(
				Origin::signed(BOB),
				0,
				1,
				AccountIdOrCollectionNftTuple::AccountId(CHARLIE)
			),
			Error::<Test>::NonTransferable
		);

		// BOB *cannot send* non-transferrable NFT (0, 1) to BOB (his own root account)
		assert_noop!(
			RMRKCore::send(
				Origin::signed(BOB),
				0,
				1,
				AccountIdOrCollectionNftTuple::AccountId(BOB)
			),
			Error::<Test>::NonTransferable
		);

		// BOB *can* send NFT (0, 0) to CHARLIE
		assert_ok!(RMRKCore::send(
			Origin::signed(BOB),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(CHARLIE)
		));

		// CHARLIE now rootowns NFT (0, 1)
		assert_eq!(RMRKCore::lookup_root_owner(0, 1).unwrap().0, CHARLIE);
	});
}

/// NFT: Reject tests (RMRK2.0 spec: new)
#[test]
fn reject_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Cannot reject non-existent NFT
		assert_noop!(
			RMRKCore::reject_nft(Origin::signed(BOB), 0, 2,),
			Error::<Test>::NoAvailableNftId
		);
		// Mint NFTs (0, 0), (0, 1), (0, 2)
		for id in 0..3 {
			assert_ok!(basic_mint(id));
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
		// Bob rejects NFT (0,0) for Bob-owned NFT (0,0)
		assert_ok!(RMRKCore::reject_nft(Origin::signed(BOB), 0, 0,));
		// Rejected NFT gets burned
		assert!(RMRKCore::nfts(0, 0).is_none());
		// Child is burned if parent is rejected
		assert!(RMRKCore::nfts(0, 1).is_none());
	});
}

/// NFT: Reject test: Cannot reject non-pending NFT
#[test]
fn reject_cannot_reject_non_pending_nft() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// ALICE mints (0, 0) for ALICE
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			None,
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true,
			None
		));
		// NFT (0, 0) is not pending
		assert!(!RMRKCore::nfts(0, 0).unwrap().pending);
		// ALICE cannot reject NFT (0, 0) since it is not pending
		assert_noop!(
			RMRKCore::reject_nft(Origin::signed(ALICE), 0, 0),
			Error::<Test>::CannotRejectNonPendingNft
		);
		// NFT (0, 0) still exists after failed rejection
		assert!(RMRKCore::nfts(0, 0).is_some());
	});
}

/// NFT: Reject tests (RMRK2.0 spec: new)
#[test]
fn reject_nft_removes_self_from_parents_children() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Alice mints (0, 0) for herself
		assert_ok!(basic_mint(0));
		// Alice mints (0, 1) for Bob
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(BOB),
			1,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true,
			None
		));
		// BOB sends NFT (0, 1) to ALICE's NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// ALICE rejects NFT (0, 1)
		assert_ok!(RMRKCore::reject_nft(Origin::signed(ALICE), 0, 1));
		// Rejected NFT gets burned
		assert_eq!(RMRKCore::nfts(0, 1).is_none(), true);
		assert_eq!(RMRKCore::children((0, 0), (0, 1)).is_none(), true);
	});
}

/// NFT: Send tests, siblings (RMRK2.0 spec: SEND)
#[test]
fn send_two_nfts_to_same_nft_creates_two_children() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2)
		for id in 0..3 {
			assert_ok!(basic_mint(id));
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
		assert!(RMRKCore::children((0, 0), (0, 1)).is_some());
		assert!(RMRKCore::children((0, 0), (0, 2)).is_some());
	});
}

/// NFT: Send tests, removing parent (RMRK2.0 spec: SEND)
#[test]
fn send_nft_removes_existing_parent() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2), (0, 3)
		for id in 0..4 {
			assert_ok!(basic_mint(id));
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
		for id in 0..3 {
			assert_ok!(basic_mint(id));
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
		assert_ok!(basic_mint(0));
		// Add two resources to NFT (to test if burning also burns the resources)

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			0,
			0,
			basic_resource.clone(),
			0
		));

		assert_ok!(RMRKCore::add_basic_resource(Origin::signed(ALICE), 0, 0, basic_resource, 1));

		// Ensure resources are there
		assert_eq!(Resources::<Test>::iter_prefix((COLLECTION_ID_0, NFT_ID_0)).count(), 2);

		// BOB should not be able to burn ALICE's NFT
		assert_noop!(
			RMRKCore::burn_nft(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS),
			Error::<Test>::NoPermission
		);
		// ALICE burns her NFT
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS));
		// Successful burn creates NFTBurned event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::NFTBurned {
			owner: ALICE,
			nft_id: 0,
		}));
		// NFT count of collection is now 0
		assert_eq!(RMRKCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// ALICE can't burn an NFT twice
		assert_noop!(
			RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS),
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
		for id in 0..4 {
			assert_ok!(basic_mint(id));
		}

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Add two resources to the great-grandchild (0, 3)
		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			3,
			basic_resource.clone(),
			0
		));

		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			3,
			basic_resource,
			1
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
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS));
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
		for id in 0..5 {
			assert_ok!(basic_mint(id));
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
		assert_noop!(
			RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, MAX_BURNS),
			Error::<Test>::TooManyRecursions
		);
		// All NFTs still exist
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 0).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 1).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 2).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 4).is_some(), true);
	});
}

/// NFT: Burn child removes NFT from owner-NFT's Children list
#[test]
fn burn_child_nft_removes_parents_children() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFTs (0, 0), (0, 1), (0, 2), (0, 3)
		for id in 0..2 {
			assert_ok!(basic_mint(id));
		}
		// ALICE sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// All NFTs exist
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 0).is_some(), true);
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 1).is_some(), true);
		// NFT (0, 0) should have 1 Children storage member
		assert_eq!(Children::<Test>::iter_prefix((0, 0)).count(), 1);
		// Burn NFT (0, 1)
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), 0, 1, MAX_BURNS),);
		// NFT (0, 0) should have 0 Children storage members
		assert_eq!(Children::<Test>::iter_prefix((0, 0)).count(), 0);
	});
}

/// Resource: Basic resource addition (RMRK2.0 spec: RESADD)
#[test]
fn create_resource_works() {
	ExtBuilder::default().build().execute_with(|| {
		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Adding a resource to non-existent NFT should fail
		assert_noop!(
			RMRKCore::add_basic_resource(
				Origin::signed(ALICE),
				0, // collection_id
				0, // nft_id
				basic_resource,
				0,
			),
			Error::<Test>::CollectionUnknown
		);
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint(0));

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Add resource to NFT
		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			basic_resource,
			0,
		));
		// Successful resource addition should trigger ResourceAdded event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAdded {
			nft_id: 0,
			resource_id: 0, // resource_id
		}));
		// Since ALICE rootowns NFT, pending status of resource should be false
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, false);

		// Create Composable resource
		let composable_resource = ComposableResource {
			parts: vec![0, 1].try_into().unwrap(), // BoundedVec of Parts
			base: 0,                               // BaseID
			metadata: None,
			slot: None,
		};

		// Composable resource addition works
		assert_ok!(RMRKCore::add_composable_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			composable_resource,
			1
		));

		// Create Slot resource
		let slot_resource = SlotResource {
			base: 0, // BaseID
			metadata: None,
			slot: 0, // SlotID
		};

		// Slot resource addition works
		assert_ok!(RMRKCore::add_slot_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			slot_resource,
			2
		));
	});
}

/// Minting with resources works
#[test]
fn add_resource_on_mint_works() {
	ExtBuilder::default().build().execute_with(|| {
		let basic_resource: BasicResource<BoundedVec<u8, UniquesStringLimit>> =
			BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Create a basic collection
		assert_ok!(basic_collection());

		// Resources to add
		let resources_to_add = bvec![
			ResourceInfoMin { id: 0, resource: ResourceTypes::Basic(basic_resource.clone()) },
			ResourceInfoMin { id: 1, resource: ResourceTypes::Basic(basic_resource) },
		];

		// Mint NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true,
			Some(resources_to_add),
		));

		assert_eq!(RMRKCore::resources((0, 0, 0)).is_some(), true);
		assert_eq!(RMRKCore::resources((0, 0, 1)).is_some(), true);
	});
}

/// Minting with more than max resources (set to 3 in mock) should panic
#[should_panic]
#[test]
fn add_resource_on_mint_beyond_max_fails() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Resources to add
		let resources_to_add = bvec![
			{ ResourceInfoMin { resource: ResourceTypes::Basic(basic_resource.clone()), id: 0 } },
			{ ResourceInfoMin { resource: ResourceTypes::Basic(basic_resource.clone()), id: 1 } },
			{ ResourceInfoMin { resource: ResourceTypes::Basic(basic_resource.clone()), id: 2 } },
			{ ResourceInfoMin { resource: ResourceTypes::Basic(basic_resource), id: 3 } },
		];

		// Mint NFT
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true,
			Some(resources_to_add),
		));
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
			Some(BOB),
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(BOB),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true,
			None
		));

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Since BOB isn't collection issuer, he can't add resources
		assert_noop!(
			RMRKCore::add_basic_resource(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				basic_resource.clone(),
				0,
			),
			Error::<Test>::NoPermission
		);

		// Collection issuer can add resource
		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			basic_resource,
			0,
		));

		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, true);
		// ALICE doesn't own BOB's NFT, so accept should fail
		assert_noop!(
			RMRKCore::accept_resource(Origin::signed(ALICE), 0, 0, 0),
			Error::<Test>::NoPermission
		);
		// BOB can accept his own NFT's pending resource
		assert_ok!(RMRKCore::accept_resource(Origin::signed(BOB), 0, 0, 0));
		// Valid resource acceptance should trigger a ResourceAccepted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceAccepted {
			nft_id: 0,
			resource_id: 0, // resource_id
		}));
		// Resource should now have false pending status
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending, false);
		// Accepting resource again should fail with ResourceNotPending
		assert_noop!(
			RMRKCore::accept_resource(Origin::signed(BOB), 0, 0, 0),
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
		assert_ok!(basic_mint(0));

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Add resource to NFT
		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			basic_resource,
			0
		));
		// Resource res-1 doesn't exist
		assert_noop!(
			RMRKCore::remove_resource(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				1, // resource_id
			),
			Error::<Test>::ResourceDoesntExist
		);
		// Only collection issuer can request resource removal
		assert_noop!(
			RMRKCore::remove_resource(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				0, // resource_id
			),
			Error::<Test>::NoPermission
		);
		// Remove resource
		assert_ok!(RMRKCore::remove_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			0, // resource_id
		));
		// Successful resource removal should trigger ResourceRemoval event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceRemoval {
			nft_id: 0,
			resource_id: 0, // resource_id
		}));
		// Since ALICE rootowns NFT, resource should be removed
		assert_eq!(RMRKCore::resources((0, 0, 0)), None);
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
			Some(BOB),
			NFT_ID_0,
			COLLECTION_ID_0,
			Some(BOB),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			true,
			None
		));

		let basic_resource = BasicResource { metadata: stbd("bafybeiakahlc6") };

		// Add resource to NFT
		assert_ok!(RMRKCore::add_basic_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			basic_resource,
			0
		));

		assert_ok!(RMRKCore::accept_resource(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, 0,));
		// Accepting a resource removal that is not pending should fail
		assert_noop!(
			RMRKCore::accept_resource_removal(Origin::signed(BOB), 0, 0, 0),
			Error::<Test>::ResourceNotPending
		);
		// Only collection's issuer can request resource removal
		assert_noop!(
			RMRKCore::remove_resource(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				0, // resource_id
			),
			Error::<Test>::NoPermission
		);
		// Resource removal requested by the collection issuer
		assert_ok!(RMRKCore::remove_resource(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			0, // resource_id
		));
		// Since ALICE doesn't root-own NFT, resource's removal is waiting for acceptance
		assert_eq!(RMRKCore::resources((0, 0, 0)).unwrap().pending_removal, true);
		// ALICE doesn't own BOB's NFT, so accept should fail
		assert_noop!(
			RMRKCore::accept_resource_removal(Origin::signed(ALICE), 0, 0, 0),
			Error::<Test>::NoPermission
		);
		// Resource res-1 doesn't exist
		assert_noop!(
			RMRKCore::accept_resource_removal(Origin::signed(BOB), 0, 0, 1),
			Error::<Test>::ResourceDoesntExist
		);
		// BOB can accept his own NFT's pending resource removal
		assert_ok!(RMRKCore::accept_resource_removal(Origin::signed(BOB), 0, 0, 0));
		// Successful resource removal acceptance should trigger ResourceRemovalAccepted event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::ResourceRemovalAccepted {
			nft_id: 0,
			resource_id: 0, // resource_id
		}));
		// Resource removed
		assert_eq!(RMRKCore::resources((0, 0, 0)), None);
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
		// set_property fails without a collection (CollectionUnknown)
		assert_noop!(
			RMRKCore::set_property(Origin::signed(ALICE), 0, Some(0), key.clone(), value.clone()),
			Error::<Test>::CollectionUnknown
		);
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint(0));
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

#[test]
fn set_property_with_internal_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Define property key
		let key = stbk("test-key");
		// Define property value
		let value = stb("test-value");
		// set_property fails without a collection (CollectionUnknown)
		assert_noop!(
			RMRKCore::do_set_property(0, Some(0), key.clone(), value.clone()),
			Error::<Test>::CollectionUnknown
		);
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint(0));
		// Root sets property on NFT
		assert_ok!(RMRKCore::do_set_property(0, Some(0), key.clone(), value.clone()));
		// Successful property setting should trigger a PropertySet event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PropertySet {
			collection_id: 0,
			maybe_nft_id: Some(0),
			key: key.clone(),
			value: value.clone(),
		}));
		// Property value now exists
		assert_eq!(RMRKCore::properties((0, Some(0), key)).unwrap(), value);
	});
}

#[test]
fn remove_property_with_internal_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Define property key
		let key = stbk("test-key");
		// Define property value
		let value = stb("test-value");
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint(0));
		// Root sets property on NFT
		assert_ok!(RMRKCore::do_set_property(0, Some(0), key.clone(), value.clone()));
		// Successful property setting should trigger a PropertySet event
		System::assert_last_event(MockEvent::RmrkCore(crate::Event::PropertySet {
			collection_id: 0,
			maybe_nft_id: Some(0),
			key: key.clone(),
			value: value.clone(),
		}));
		// Property value now exists
		assert_eq!(RMRKCore::properties((0, Some(0), key.clone())).unwrap(), value);
		// Origin::root() removes property
		assert_ok!(RMRKCore::do_remove_property(0, Some(0), key.clone()));
		assert_eq!(RMRKCore::properties((0, Some(0), key)), None);
	});
}

/// Priority: Setting priority tests (RMRK2.0 spec: SETPRIORITY)
#[test]
fn set_priority_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint NFT
		assert_ok!(basic_mint(0));
		// BOB cannot set priority on NFT

		assert_noop!(
			RMRKCore::set_priority(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, bvec![100, 500]),
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
