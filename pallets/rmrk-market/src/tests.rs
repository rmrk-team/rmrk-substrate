// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-market.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event as MockEvent, *};

use sp_runtime::Permill;
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
		Some(ALICE),
		COLLECTION_ID_0,
		Some(ALICE),
		Some(Permill::from_float(1.525)),
		bvec![0u8; 20],
		true,
		None,
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
		assert_noop!(
			RmrkMarket::list(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, 10u128, None,),
			Error::<Test>::NoPermission
		);
		// ALICE cannot list a non-existing NFT
		assert_noop!(
			RmrkMarket::list(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NOT_EXISTING_NFT_ID,
				10u128,
				None,
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
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(
				COLLECTION_ID_0,
				NFT_ID_0,
			),
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_1,
			approval_required: false,
		}));
		// ALICE cannot list NFT [0,1] bc it is owned by NFT[0,0]
		assert_noop!(
			RmrkMarket::list(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_1, 10u128, None,),
			Error::<Test>::CannotListNftOwnedByNft
		);
		// ALICE lists the NFT successfully
		assert_ok!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
			None,
		));
		// Listed NFT should trigger TokenListed event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: 0,
			nft_id: 0,
			price: 10u128,
		}));
	});
}

#[test]
fn list_non_transferable_fail() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Mint non-transferable NFT
		assert_ok!(RmrkCore::mint_nft(
			Origin::signed(ALICE),
			Some(ALICE),
			COLLECTION_ID_0,
			Some(ALICE),
			Some(Permill::from_float(1.525)),
			bvec![0u8; 20],
			false, // non-transferable
			None,
		));
		assert_noop!(
			RmrkMarket::list(Origin::signed(ALICE), COLLECTION_ID_0, 0, 10u128, None,),
			pallet_rmrk_core::Error::<Test>::NonTransferable
		);
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
			None,
		));
		// Listed NFT should trigger TokenListed event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
		}));
		// Ensure that ALICE cannot buy the listed NFT
		assert_noop!(
			RmrkMarket::buy(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, Some(10u128),),
			Error::<Test>::CannotBuyOwnToken
		);
		// Ensure that ALICE cannot buy the listed NFT
		assert_noop!(
			RmrkMarket::buy(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, Some(9u128),),
			Error::<Test>::PriceDiffersFromExpected
		);
		// BOB buys the NFT and the NFT is transferred from ALICE to BOB
		assert_ok!(RmrkMarket::buy(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, Some(10u128),));
		// Bought NFT should trigger TokenSold event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenSold {
			owner: ALICE,
			buyer: BOB,
			collection_id: 0,
			nft_id: 0,
			price: 10u128,
		}));
		// Ensure BOB is the new owner of NFT (0,0)
		assert_eq!(Uniques::owner(COLLECTION_ID_0, NFT_ID_0), Some(BOB));
	});
}

#[test]
fn buy_wont_work_after_list_expires() {
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
			Some(1),
		));
		// Listed NFT should trigger TokenListed event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
		}));
		// Set block number to expired block
		System::set_block_number(2);
		// Ensure that BOB cannot buy the listed NFT as the listing expired
		assert_noop!(
			RmrkMarket::buy(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, None,),
			Error::<Test>::ListingHasExpired
		);
	});
}

#[test]
fn send_wont_work_if_sent_after_list() {
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
			None,
		));
		// Listed NFT should trigger TokenListed event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
		}));
		// Ensure that ALICE cannot buy the listed NFT
		assert_noop!(
			RmrkMarket::buy(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, None,),
			Error::<Test>::CannotBuyOwnToken
		);
		// NFT Lock Tests Ensure ALICE cannot sends CHARLIE NFT [0,0] bc it is now locked
		assert_noop!(
			RmrkCore::send(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				AccountIdOrCollectionNftTuple::AccountId(CHARLIE),
			),
			pallet_uniques::Error::<Test>::Locked
		);
		// BOB buys the NFT at whatever price is in storage and the NFT is transferred from ALICE to
		// BOB
		assert_ok!(RmrkMarket::buy(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, None,));

		// Bought NFT should trigger TokenSold event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenSold {
			owner: ALICE,
			buyer: BOB,
			collection_id: 0,
			nft_id: 0,
			price: 10u128,
		}));
		// Ensure BOB is the still new owner of NFT (0,0)
		assert_eq!(Uniques::owner(COLLECTION_ID_0, NFT_ID_0), Some(BOB));
	});
}

#[test]
fn send_to_nft_wont_work_after_list() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// Mint an two NFTs
		assert_ok!(basic_mint());
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 2);
		// ALICE lists the NFT successfully
		assert_ok!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
			None,
		));
		// Listed NFT should trigger TokenListed event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
		}));
		// Ensure that ALICE cannot buy the listed NFT
		assert_noop!(
			RmrkMarket::buy(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, Some(10u128),),
			Error::<Test>::CannotBuyOwnToken
		);
		// NFT Lock Tests ALICE sends NFT [0,0] to NFT [0,1] won't work
		assert_noop!(
			RmrkCore::send(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(COLLECTION_ID_0, NFT_ID_1),
			),
			pallet_uniques::Error::<Test>::Locked
		);
		// NFT Lock test directly calling the Uniques:do_transfer should fail
		assert_noop!(
			Uniques::do_transfer(
				COLLECTION_ID_0,
				NFT_ID_0,
				CHARLIE,
				|_class_details, _details| Ok(()),
			),
			pallet_uniques::Error::<Test>::Locked
		);
		// BOB buys the NFT and the NFT is transferred from ALICE to BOB
		assert_ok!(RmrkMarket::buy(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, Some(10u128),));
		// Bought NFT should trigger TokenSold event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenSold {
			owner: ALICE,
			buyer: BOB,
			collection_id: 0,
			nft_id: 0,
			price: 10u128,
		}));
		// Ensure BOB is the still new owner of NFT [0,0]
		assert_eq!(Uniques::owner(COLLECTION_ID_0, NFT_ID_0), Some(BOB));
		// BOB can now send NFT [0,0] to NFT [0,1] since NFT is not locked
		assert_ok!(RmrkCore::send(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(COLLECTION_ID_0, NFT_ID_1),
		));
		// Successful send triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(pallet_rmrk_core::Event::NFTSent {
			sender: BOB,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(
				COLLECTION_ID_0,
				NFT_ID_1,
			),
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			approval_required: true,
		}));
	});
}

#[test]
fn accept_offer_wont_work_if_traded_to_nft_after_list() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// Mint an two NFTs
		assert_ok!(basic_mint());
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 2);
		// BOB successfully places offer
		assert_ok!(RmrkMarket::make_offer(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			MIN_OFFER_ON_NFT,
			None,
		));
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferPlaced {
			offerer: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: MIN_OFFER_ON_NFT,
		}));
		// ALICE sends NFT [0,0] to NFT [0.1]
		assert_ok!(RmrkCore::send(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(COLLECTION_ID_0, NFT_ID_1),
		));
		// Successful send to NFT triggers NFTSent event
		System::assert_last_event(MockEvent::RmrkCore(pallet_rmrk_core::Event::NFTSent {
			sender: ALICE,
			recipient: AccountIdOrCollectionNftTuple::CollectionAndNftTuple(
				COLLECTION_ID_0,
				NFT_ID_1,
			),
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			approval_required: false,
		}));
		// ALICE cannot accept offer anymore
		assert_noop!(
			RmrkMarket::accept_offer(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, BOB,),
			Error::<Test>::NoPermission
		);
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
		assert_noop!(
			RmrkMarket::list(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0, 10u128, None,),
			Error::<Test>::NoPermission
		);
		// ALICE cannot list a non-existing NFT
		assert_noop!(
			RmrkMarket::list(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NOT_EXISTING_NFT_ID,
				10u128,
				None,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// ALICE cannot unlist a NFT if not listed
		assert_noop!(
			RmrkMarket::unlist(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0,),
			Error::<Test>::CannotUnlistToken
		);
		// ALICE lists the NFT successfully
		assert_ok!(RmrkMarket::list(
			Origin::signed(ALICE),
			COLLECTION_ID_0,
			NFT_ID_0,
			10u128,
			None,
		));
		// Listed NFT should trigger TokenListed event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenListed {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: 10u128,
		}));
		// BOB cannot unlist a NFT if not owned by BOB
		assert_noop!(
			RmrkMarket::unlist(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0,),
			Error::<Test>::NoPermission
		);
		// ALICE unlists the NFT successfully
		assert_ok!(RmrkMarket::unlist(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0,));
		// Unisted NFT should trigger TokenUnlisted event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::TokenUnlisted {
			owner: ALICE,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
		}));
	});
}

#[test]
fn offer_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// ALICE cannot offer on a non-existing NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		// ALICE cannot offer on own NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::CannotOfferOnOwnToken
		);
		// BOB cannot offer below the MinimumOfferAmount threshold
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT - 1,
				None,
			),
			Error::<Test>::OfferTooLow
		);
		// BOB successfully places offer
		assert_ok!(RmrkMarket::make_offer(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			MIN_OFFER_ON_NFT,
			None,
		));
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferPlaced {
			offerer: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: MIN_OFFER_ON_NFT,
		}));
		// BOB cannot offer again on a NFT with an active offer
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT + 50,
				None,
			),
			Error::<Test>::AlreadyOffered
		);
	});
}

#[test]
fn offer_withdrawn_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// ALICE cannot offer on a non-existing NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		// ALICE cannot offer on own NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::CannotOfferOnOwnToken
		);
		// BOB cannot offer below the MinimumOfferAmount threshold
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT - 1,
				None,
			),
			Error::<Test>::OfferTooLow
		);
		// BOB cannot withdraw an offer that hasn't been made
		assert_noop!(
			RmrkMarket::withdraw_offer(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0,),
			Error::<Test>::UnknownOffer
		);
		// BOB successfully places offer
		assert_ok!(RmrkMarket::make_offer(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			MIN_OFFER_ON_NFT,
			None,
		));
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferPlaced {
			offerer: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: MIN_OFFER_ON_NFT,
		}));
		// ALICE cannot withdraw offer on own NFT
		assert_noop!(
			RmrkMarket::withdraw_offer(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0,),
			Error::<Test>::UnknownOffer
		);
		// BOB successfully places withdraws offer
		assert_ok!(RmrkMarket::withdraw_offer(Origin::signed(BOB), COLLECTION_ID_0, NFT_ID_0,));
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferWithdrawn {
			sender: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
		}));
		// ALICE cannot accept offer anymore
		assert_noop!(
			RmrkMarket::accept_offer(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, BOB,),
			Error::<Test>::UnknownOffer
		);
	});
}

#[test]
fn accept_offer_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// ALICE cannot offer on a non-existing NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		// ALICE cannot offer on own NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::CannotOfferOnOwnToken
		);
		// BOB cannot offer below the MinimumOfferAmount threshold
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT - 1,
				None,
			),
			Error::<Test>::OfferTooLow
		);
		// BOB successfully places offer
		assert_ok!(RmrkMarket::make_offer(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			MIN_OFFER_ON_NFT,
			None,
		));
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferPlaced {
			offerer: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: MIN_OFFER_ON_NFT,
		}));
		// ALICE accepts BOB's offer
		assert_ok!(
			RmrkMarket::accept_offer(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, BOB,)
		);
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferAccepted {
			owner: ALICE,
			buyer: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
		}));
	});
}

#[test]
fn accept_expired_offer_wont_works() {
	new_test_ext().execute_with(|| {
		// Create a basic collection
		assert_ok!(basic_collection());
		// Collection nfts_count should be 0 prior to minting
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 0);
		// ALICE cannot offer on a non-existing NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::TokenDoesNotExist
		);
		// Mint an NFT
		assert_ok!(basic_mint());
		// Minting an NFT should cause nfts_count to increase to 1
		assert_eq!(RmrkCore::collections(COLLECTION_ID_0).unwrap().nfts_count, 1);
		// ALICE cannot offer on own NFT
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(ALICE),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT,
				None,
			),
			Error::<Test>::CannotOfferOnOwnToken
		);
		// BOB cannot offer below the MinimumOfferAmount threshold
		assert_noop!(
			RmrkMarket::make_offer(
				Origin::signed(BOB),
				COLLECTION_ID_0,
				NFT_ID_0,
				MIN_OFFER_ON_NFT - 1,
				None,
			),
			Error::<Test>::OfferTooLow
		);
		// BOB successfully places offer
		assert_ok!(RmrkMarket::make_offer(
			Origin::signed(BOB),
			COLLECTION_ID_0,
			NFT_ID_0,
			MIN_OFFER_ON_NFT,
			Some(1),
		));
		// Offer from BOB on ALICE's NFT should trigger OfferPlaced event
		System::assert_last_event(MockEvent::RmrkMarket(crate::Event::OfferPlaced {
			offerer: BOB,
			collection_id: COLLECTION_ID_0,
			nft_id: NFT_ID_0,
			price: MIN_OFFER_ON_NFT,
		}));
		// CHARLIE cannot accepts BOB's offer
		assert_noop!(
			RmrkMarket::accept_offer(Origin::signed(CHARLIE), COLLECTION_ID_0, NFT_ID_0, BOB,),
			Error::<Test>::NoPermission
		);
		// Set block number to expired block
		System::set_block_number(2);
		// ALICE accepts BOB's offer
		assert_noop!(
			RmrkMarket::accept_offer(Origin::signed(ALICE), COLLECTION_ID_0, NFT_ID_0, BOB,),
			Error::<Test>::OfferHasExpired
		);
	});
}
