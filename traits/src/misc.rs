// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

/// Trait for pre-checks and post-checks for transfers that can be implemented downstream to extend
/// the logic of RMRK's current funcitonality.
pub trait TransferHooks<AccountId, CollectionId, NftId> {
	/// Check if the NFT's pre-checks and post-checks for the transfer function based on the sender,
	/// `collection_id` and `nft_id` parameters.
	fn pre_check(
		sender: &AccountId,
		recipient: &AccountId,
		collection_id: &CollectionId,
		nft_id: &NftId,
	) -> bool;
	fn post_transfer(
		sender: &AccountId,
		recipient: &AccountId,
		collection_id: &CollectionId,
		nft_id: &NftId,
	) -> bool;
}

impl<AccountId, CollectionId, NftId> TransferHooks<AccountId, CollectionId, NftId> for () {
	fn pre_check(
		_sender: &AccountId,
		_recipient: &AccountId,
		_collection_id: &CollectionId,
		_nft_id: &NftId,
	) -> bool {
		true
	}

	fn post_transfer(
		_sender: &AccountId,
		_recipient: &AccountId,
		_collection_id: &CollectionId,
		_nft_id: &NftId,
	) -> bool {
		true
	}
}
