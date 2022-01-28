use super::*;
use crate::mock::*;
use mock::{Event as MockEvent, *};
use frame_support::{assert_noop, assert_ok, traits::Currency};

use pallet_balances::Error as BalancesError;
use sp_std::prelude::*;
use sp_std::{convert::TryInto, vec::Vec};

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

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Shortcut for a test collection creation (Alice is issue, max NFTs is 5)
fn basic_collection() -> DispatchResult {
	RmrkCore::create_collection(Origin::signed(ALICE), bvec![0u8; 20], Some(5), bvec![0u8; 15])
}

/// Shortcut for a basic mint (Alice owner, Collection ID 0, Royalty 1.525)
fn basic_mint() -> DispatchResult {
	RmrkCore::mint_nft(
		Origin::signed(ALICE),
		ALICE,
		COLLECTION_ID_0,
		Some(ALICE),
		Some(Permill::from_float(1.525)),
		bvec![0u8; 20],
	)
}

#[test]
fn list_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Mint another NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 2);
		// BOB shouldn't be able to list ALICE's NFT
		assert_noop!(RmrkMarket::list(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
			),
			Error::<Test>::NoPermission
		);
		// ALICE cannot list a non-existing NFT
		assert_noop!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NOT_EXISTING_NFT_ID,
			10u128,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// ALICE sends NFT [0,1] to NFT [0,0]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(COLLECTION_ID_0, NFT_ID_0),
		));
		// Successful send to NFT triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(pallet_rmrk_core::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(COLLECTION_ID_0, NFT_ID_0),
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_1,
		}));
		// ALICE cannot list NFT [0,1] bc it is owned by NFT[0,0]
		assert_noop!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_1,
			10u128,
			),
			Error::<Test>::CannotListNftOwnedByNft
		);
		// ALICE lists the NFT successfully
		assert_ok!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
		));
		// Listed NFT should trigger TokenListed event TODO: royalty
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: 0,
			nft_id: 0,
			price: 10u128,
			royalty: None,
		}));
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		// ALICE lists the NFT successfully
		assert_ok!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
		));
		// Listed NFT should trigger TokenListed event TODO: royalty
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
			royalty: None,
		}));
		// Ensure that ALICE cannot buy the listed NFT
		assert_noop!(RmrkMarket::buy(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			),
			Error::<Test>::CannotBuyOwnToken
		);
		// BOB buys the NFT and the NFT is transferred from ALICE to BOB
		assert_ok!(RmrkMarket::buy(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
		));
		// Bought NFT should trigger TokenSold event TODO: royalty
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenSold {
			owner: ALICE,
			buyer: BOB,
			collection_id: 0,
			nft_id: 0,
			price: 10u128,
			royalty: None,
			royalty_amount: None,
		}));
		// Ensure BOB is the new owner of NFT (0,0)
		assert_eq!(Uniques::owner(COLLECTION_ID_0, NFT_ID_0), Some(BOB));
	});
}

#[test]
fn unlist_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Mint another NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 2);
		// BOB shouldn't be able to list ALICE's NFT
		assert_noop!(RmrkMarket::list(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
			),
			Error::<Test>::NoPermission
		);
		// ALICE cannot list a non-existing NFT
		assert_noop!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NOT_EXISTING_NFT_ID,
			10u128,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// ALICE cannot unlist a NFT if not listed
		assert_noop!(RmrkMarket::unlist(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			),
			Error::<Test>::CannotUnlistToken
		);
		// ALICE lists the NFT successfully
		assert_ok!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
		));
		// Listed NFT should trigger TokenListed event TODO: royalty
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
			royalty: None,
		}));
		// BOB cannot unlist a NFT if not owned by BOB
		assert_noop!(RmrkMarket::unlist(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			),
			Error::<Test>::NoPermission
		);
		// ALICE unlists the NFT successfully
		assert_ok!(RmrkMarket::unlist(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
		));
		// Unisted NFT should trigger TokenUnlisted event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenUnlisted {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
		}));
	});
}

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// TODO: test cases
		assert_eq!(1, 1);
	});
}
