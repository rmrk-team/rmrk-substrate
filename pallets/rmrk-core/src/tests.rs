use frame_support::{assert_noop, assert_ok, error::BadOrigin};
// use sp_runtime::AccountId32;

// use crate::types::ClassType;

use super::*;
use mock::*;
use pallet_uniques as UNQ;

type RMRKCore = Pallet<Test>;

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, ValueLimit> {
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
			Some(0),
			Some(b"metadata".to_vec())
		));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			COLLECTION_ID_0,
			Some(ALICE),
			Some(20),
			Some(b"metadata".to_vec())
		));
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(BOB),
			BOB,
			COLLECTION_ID_0,
			Some(CHARLIE),
			Some(20),
			Some(b"metadata".to_vec())
		));
		assert_noop!(
			RMRKCore::mint_nft(
				Origin::signed(ALICE),
				ALICE,
				NOT_EXISTING_CLASS_ID,
				Some(CHARLIE),
				Some(20),
				Some(b"metadata".to_vec())
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
			Some(0),
			Some(nft_metadata.clone())
		));
		// Alice mints NFT (0, 1) [will be the child]
		assert_ok!(RMRKCore::mint_nft(
			Origin::signed(ALICE),
			ALICE,
			0,
			Some(ALICE),
			Some(0),
			Some(nft_metadata)
		));
		// Alice sends NFT (0, 0) [parent] to Bob
		let x = RMRKCore::send(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB),
		);
		// Alice sends NFT (0, 1) [child] to NFT (0, 0) [parent]
		let x = RMRKCore::send(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		);

		// Check that NFT (0,1) [child] is owned by NFT (0,0) [parent]
		assert_eq!(
			RMRKCore::nfts(0, 1).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
		);

		// Check that Bob now root-owns NFT (0, 1) [child] since he wasn't originally rootowner
		assert_eq!(RMRKCore::nfts(0, 1).unwrap().rootowner, BOB)
	});
}
