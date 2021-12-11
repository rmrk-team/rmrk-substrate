use frame_support::{assert_noop, assert_ok, error::BadOrigin};
// use sp_runtime::AccountId32;
use sp_runtime::Permill;
// use crate::types::ClassType;

use super::*;
use mock::*;
use pallet_uniques as UNQ;
use sp_std::{convert::TryInto, vec::Vec};

type RMRKCore = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, ValueLimit> {
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

#[test]
fn create_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()));
		assert_noop!(
			RMRKCore::create_collection(
				Origin::signed(ALICE),
				vec![0; <Test as UNQ::Config>::StringLimit::get() as usize + 1]
			),
			Error::<Test>::TooLong
		);
		NextCollectionId::<Test>::mutate(|id| *id = <Test as UNQ::Config>::ClassId::max_value());
		assert_noop!(
			RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()),
			Error::<Test>::NoAvailableCollectionId
		);
	});
}

#[test]
fn mint_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), b"metadata".to_vec()));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(20.525)),
			b"metadata".to_vec()
		));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(20.525)),
			b"metadata".to_vec()
		));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(BOB),
			BOB,
			COLLECTION_ID_0,
			Some(CHARLIE),
			Some(Permill::from_float(20.525)),
			b"metadata".to_vec()
		));
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(ALICE),
				ALICE,
				NOT_EXISTING_CLASS_ID,
				Some(CHARLIE),
				Some(Permill::from_float(20.525)),
				b"metadata".to_vec()
			),
			Error::<Test>::CollectionUnknown
		);
	});
}
#[test]
fn send_nft_to_minted_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		let collection_metadata = stv("testing");
		let nft_metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), collection_metadata));
		// Alice mints NFT (0, 0) [will be the parent]
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			nft_metadata.clone()
		));
		// Alice mints NFT (0, 1) [will be the child]
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			nft_metadata
		));
		// Alice sends NFT (0, 0) [parent] to Bob
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		));
		// Alice sends NFT (0, 1) [child] to NFT (0, 0) [parent]
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

		// Check that NFT (0,1) [child] is owned by NFT (0,0) [parent]
		assert_eq!(
			RMRKCore::nfts(0, 1).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		);

		// Check that Bob now root-owns NFT (0, 1) [child] since he wasn't originally rootowner
		assert_eq!(RMRKCore::nfts(0, 1).unwrap().rootowner, BOB);

		// Error if sender doesn't root-own sending NFT
		assert_noop!(
			RMRKCore::send(
				Origin::signed(CHARLIE),
				0,
				0,
				AccountIdOrCollectionNftTuple::AccountId(BOB)
			),
			Error::<Test>::NoPermission
		);

		// Error if sending NFT doesn't exist
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				666,
				666,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
			),
			Error::<Test>::NoAvailableNftId
		);

		// BOB can send back child NFT to ALICE
		assert_ok!(RMRKCore::send(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		));

		// Error if recipient is NFT and that NFT doesn't exist
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
fn send_two_nfts_to_same_nft_creates_two_children() {
	ExtBuilder::default().build().execute_with(|| {
		let collection_metadata = stv("testing");
		let nft_metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), collection_metadata));
		// Alice mints NFT (0, 0)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata.clone()
		));
		// Alice mints NFT (0, 1)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata.clone()
		));
		// Alice mints NFT (0, 2)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata
		));

		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Alice sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Children for NFT (0, 0) contains (0, 1) and (0, 2)
		assert_eq!(RMRKCore::children(0, 0).unwrap(), vec![(0, 1), (0, 2)]);
	});
}

#[test]
fn send_nft_removes_existing_parent() {
	ExtBuilder::default().build().execute_with(|| {
		let collection_metadata = stv("testing");
		let nft_metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), collection_metadata));
		// Alice mints NFT (0, 0)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata.clone()
		));
		// Alice mints NFT (0, 1)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata.clone()
		));
		// Alice mints NFT (0, 2)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata.clone()
		));
		// Alice mints NFT (0, 3)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			nft_metadata
		));

		// Alice sends NFT (0, 1) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));
		// Alice sends NFT (0, 2) to NFT (0, 0)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		));

		// NFT (0, 0) is parent of NFT (0, 1)
		assert_eq!(RMRKCore::children(0, 0).unwrap(), vec![(0, 1), (0, 2)]);

		// Alice sends NFT (0, 1) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));

		// NFT (0, 0) is not parent of NFT (0, 1)
		assert_eq!(RMRKCore::children(0, 0).unwrap(), vec![(0, 2)]);
	});
}

// #[test]
// TODO fn cannot send to its own descendent?  this should be easy enough to check
// TODO fn cannot send to its own grandparent?  this seems difficult to check without implementing a new Parent storage struct

#[test]
fn change_issuer_works() {
	ExtBuilder::default().build().execute_with(|| {
		let collection_metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), collection_metadata));
		assert_ok!(RMRKCore::change_issuer(Origin::signed(ALICE), 0, BOB));
		assert_eq!(RMRKCore::collections(0).unwrap().issuer, BOB);
	});
}

#[test]
fn burn_nft_works() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			metadata.clone()
		));
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, NFT_ID_0), None);
	});
}

#[test]
fn burn_nft_with_great_grandchildren_works() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()));
		// Alice mints (0, 0)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
		// Alice mints (0, 1)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
		// Alice mints (0, 2)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
		// Alice mints (0, 3)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
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
		// Alice sends NFT (0, 3) to NFT (0, 2)
		assert_ok!(RMRKCore::send(
			Origin::signed(ALICE),
			0,
			3,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
		));
		// Child is alive
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3).is_some(), true);
		// Burn great-grandfather
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		// Child is dead
		assert_eq!(RMRKCore::nfts(COLLECTION_ID_0, 3), None);
	});
}

#[test]
fn send_to_grandchild_fails() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()));
		// Alice mints (0, 0)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
		// Alice mints (0, 1)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
		// Alice mints (0, 2)
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(0.0)),
			metadata.clone()
		));
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

		// Alice sends (0, 0) to (0, 2)
		assert_noop!(
			RMRKCore::send(
				Origin::signed(ALICE),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 2),
			),
			Error::<Test>::CannotSendToDescendent
		);
	});
}

#[test]
fn destroy_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			metadata.clone()
		));
		assert_noop!(
			RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0),
			Error::<Test>::CollectionNotEmpty
		);
		assert_ok!(RMRKCore::burn_nft(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0));
		assert_ok!(RMRKCore::destroy_collection(Origin::signed(ALICE), COLLECTION_ID_0));
	});
}

#[test]
fn set_property_works() {
	ExtBuilder::default().build().execute_with(|| {
		let metadata = stv("testing");
		let key = stbk("test-key");
		let value = stb("test-value");
		assert_ok!(RMRKCore::create_collection(Origin::signed(ALICE), metadata.clone()));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(0),
			Some(metadata.clone())
		));
		assert_ok!(RMRKCore::set_property(
			Origin::signed(ALICE),
			0,
			Some(0),
			key.clone(),
			value.clone()
		));
		assert_eq!(RMRKCore::properties((0, Some(0), key)).unwrap(), value);
	});
}
