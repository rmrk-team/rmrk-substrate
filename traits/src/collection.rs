use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::{cmp::Eq, result};

/// Collection info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CollectionInfo<BoundedString, AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,
	pub metadata: BoundedString,
	pub max: u32,
	pub symbol: BoundedString,
	pub id: BoundedString,
}

/// Abstraction over a Collection system.
#[allow(clippy::upper_case_acronyms)]
pub trait Collection<BoundedString, AccountId> {
	type CollectionId: Default + Copy;
	// fn collection_two_info(
	// 	id: Self::CollectionTwoId,
	// ) -> Option<CollectionTwoInfo<AccountId, BoundedString>>;
	fn issuer(collection_id: Self::CollectionId) -> Option<AccountId>;
	fn create_collection(
		issuer: AccountId,
		metadata: BoundedString,
		max: u32,
		symbol: BoundedString,
		id: BoundedString,
	) -> sp_std::result::Result<Self::CollectionId, DispatchError>;
	fn burn_collection(issuer: AccountId, collection_id: Self::CollectionId) -> DispatchResult;
}

// #[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// pub struct ClassInfo<BoundedString, AccountId> {
// 	/// Arbitrary data about a class, e.g. IPFS hash
// 	pub issuer: AccountId,
// }

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
