use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use frame_support::pallet_prelude::MaxEncodedLen;

use crate::primitives::*;
use sp_std::result::Result;

/// Collection info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectionInfo<BoundedString, BoundedSymbol, AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,
	pub metadata: BoundedString,
	pub max: Option<u32>,
	pub symbol: BoundedSymbol,
	pub nfts_count: u32,
}

/// Abstraction over a Collection system.
#[allow(clippy::upper_case_acronyms)]
pub trait Collection<BoundedString, BoundedSymbol, AccountId> {
	fn issuer(collection_id: CollectionId) -> Option<AccountId>;
	fn collection_create(
		issuer: AccountId,
		metadata: BoundedString,
		max: Option<u32>,
		symbol: BoundedSymbol,
	) -> Result<CollectionId, DispatchError>;
	fn collection_burn(issuer: AccountId, collection_id: CollectionId) -> DispatchResult;
	fn collection_change_issuer(
		collection_id: CollectionId,
		new_issuer: AccountId,
	) -> Result<(AccountId, CollectionId), DispatchError>;
	fn collection_lock(
		sender: AccountId,
		collection_id: CollectionId,
	) -> Result<CollectionId, DispatchError>;
}
