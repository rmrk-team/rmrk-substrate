use frame_support::pallet_prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ListInfo<AccountId, Balance, BlockNumber> {
    /// Owner who listed the NFT at the time
    pub(super) listed_by: AccountId,
    /// Listed amount
    pub(super) amount: Balance,
    /// After this block the listing can't be bought
    pub(super) expires: Option<BlockNumber>,
}

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Offer<AccountId, Balance, BlockNumber> {
    /// User who made the offer
    pub(super) maker: AccountId,
    /// Offered amount
    pub(super) amount: Balance,
    /// After this block the offer can't be accepted
    pub(super) expires: Option<BlockNumber>,
}