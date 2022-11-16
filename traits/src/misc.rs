// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

/// Trait to handle transfer function checks that can be implemented downstream to extend the logic
/// of RMRK's current functionality.
pub trait CheckAllowTransferFn<AccountId, CollectionId, NftId> {
	/// Check if the NFT can be transferred based on the sender, collection_id and nft_id
	/// parameters.
	fn check(sender: &AccountId, collection_id: &CollectionId, nft_id: &NftId) -> bool;
}

impl<AccountId, CollectionId, NftId> CheckAllowTransferFn<AccountId, CollectionId, NftId> for () {
	fn check(_sender: &AccountId, _collection_id: &CollectionId, _nft_id: &NftId) -> bool {
		true
	}
}
