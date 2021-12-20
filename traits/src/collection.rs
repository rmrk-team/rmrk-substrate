use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;

/// Collection info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CollectionInfo<BoundedString, AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,
	pub metadata: BoundedString,
	pub max: u32,
	pub symbol: BoundedString,
}

/// Abstraction over a Collection system.
#[allow(clippy::upper_case_acronyms)]
pub trait Collection<BoundedString, AccountId> {
	fn issuer(collection_id: CollectionId) -> Option<AccountId>;
	fn create_collection(
		issuer: AccountId,
		metadata: BoundedString,
		max: u32,
		symbol: BoundedString,
	) -> sp_std::result::Result<CollectionId, DispatchError>;
	fn burn_collection(issuer: AccountId, collection_id: CollectionId) -> DispatchResult;
	fn change_issuer(
		collection_id: CollectionId,
		new_issuer: AccountId,
	) -> sp_std::result::Result<(AccountId, CollectionId), DispatchError>;
	fn lock_collection(
		collection_id: CollectionId,
	) -> sp_std::result::Result<CollectionId, DispatchError>;
}
