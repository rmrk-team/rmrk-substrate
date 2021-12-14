use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::{cmp::Eq, result};

/// Collection info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CollectionTwoInfo<AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,
}

/// Abstraction over a Collection system.
#[allow(clippy::upper_case_acronyms)]
pub trait CollectionTwo<AccountId> {
	type CollectionTwoId: Default + Copy;
	fn collection_two_info(id: Self::CollectionTwoId) -> Option<CollectionTwoInfo<AccountId>>;
	fn issuer(collection_id: Self::CollectionTwoId) -> Option<AccountId>;
	fn create_collection(issuer: AccountId) -> DispatchResult;
}

/// Auction info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct AuctionInfo<AccountId, Balance, BlockNumber> {
	/// Current bidder and bid price.
	pub bid: Option<(AccountId, Balance)>,
	/// Define which block this auction will be started.
	pub start: BlockNumber,
	/// Define which block this auction will be ended.
	pub end: Option<BlockNumber>,
}
