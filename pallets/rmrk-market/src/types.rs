// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-market.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use frame_support::pallet_prelude::*;
use sp_runtime::Permill;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ListInfo<AccountId, Balance, BlockNumber> {
	/// Owner who listed the NFT at the time
	pub(super) listed_by: AccountId,
	/// Listed amount
	pub(super) amount: Balance,
	/// After this block the listing can't be bought
	pub(super) expires: Option<BlockNumber>,
}

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Offer<AccountId, Balance, BlockNumber> {
	/// User who made the offer
	pub(super) maker: AccountId,
	/// Offered amount
	pub(super) amount: Balance,
	/// After this block the offer can't be accepted
	pub(super) expires: Option<BlockNumber>,
}

/// Trait to calculate Marketplace hooks that can be implemented downstream to enforce standard
/// Marketplace fees and royalties.
pub trait MarketplaceHooks<Balance, CollectionId, NftId> {
	/// Market Fee.
	type MarketFee;
	/// Standard Marketplace fee set downstream. The default return value will be None.
	fn calculate_market_fee(amount: Balance) -> Option<Balance>;
	/// For Marketplaces that enforce royalties, a royalty fee is paid after a successful `buy()`.
	/// Default return value is None.
	fn calculate_royalty_fee(amount: Balance, royalty_fee: Permill) -> Option<Balance>;
	/// Check to ensure the NFT can be listed or bought in the Marketplace. Default is true.
	fn can_list_or_buy_in_marketplace(collection_id: &CollectionId, nft_id: &NftId) -> bool;
}

impl<Balance, CollectionId, NftId> MarketplaceHooks<Balance, CollectionId, NftId> for () {
	type MarketFee = Permill;
	fn calculate_market_fee(_amount: Balance) -> Option<Balance> {
		None
	}

	fn calculate_royalty_fee(_amount: Balance, _royalty_fee: Permill) -> Option<Balance> {
		None
	}

	fn can_list_or_buy_in_marketplace(_collection_id: &CollectionId, _nft_id: &NftId) -> bool {
		true
	}
}
