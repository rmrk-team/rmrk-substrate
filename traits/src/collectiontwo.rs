// use crate::Change;
use codec::FullCodec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32Bit, Bounded, MaybeSerializeDeserialize},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
	result,
};

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

/// Abstraction over a simple auction system.
pub trait Auction<AccountId, BlockNumber> {
	/// The id of an AuctionInfo
	type AuctionId: FullCodec
		+ Default
		+ Copy
		+ Eq
		+ PartialEq
		+ MaybeSerializeDeserialize
		+ Bounded
		+ Debug;
	/// The price to bid.
	type Balance: AtLeast32Bit + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

	/// The auction info of `id`
	fn auction_info(
		id: Self::AuctionId,
	) -> Option<AuctionInfo<AccountId, Self::Balance, BlockNumber>>;
	/// Update the auction info of `id` with `info`
	fn update_auction(
		id: Self::AuctionId,
		info: AuctionInfo<AccountId, Self::Balance, BlockNumber>,
	) -> DispatchResult;
	/// Create new auction with specific startblock and endblock, return the id
	/// of the auction
	fn new_auction(
		start: BlockNumber,
		end: Option<BlockNumber>,
	) -> result::Result<Self::AuctionId, DispatchError>;
	/// Remove auction by `id`
	fn remove_auction(id: Self::AuctionId);

	fn create_collection() -> sp_std::result::Result<Self::CollectionTwoId, DispatchError> {}
}
