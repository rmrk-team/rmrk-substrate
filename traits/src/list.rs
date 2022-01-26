use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ListInfo<ListId, CollectionId, NftId, Balance> {
    /// List id of the listed RMRK NFT
    pub list_id: ListId,
    /// Collection id of the listed RMRK NFT
    pub collection_id: CollectionId,
    /// NFT id of the listed RMRK NFT
    pub nft_id: NftId,
    /// Price of the listed RMRK NFT
    pub price: Balance,
}

/// Abstraction over a listed NFT
pub trait List<ListId, CollectionId, NftId, Balance> {
    /// List id of the listed RMRK NFT
    fn list_id(&self) -> ListId;
    /// Collection id of the listed RMRK NFT
    fn collection_id(&self) -> CollectionId;
    /// NFT id of the listed RMRK NFT
    fn nft_id(&self) -> NftId;
    /// Price of the listed RMRK NFT
    fn price(&self) -> Balance;
}