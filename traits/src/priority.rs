use sp_runtime::DispatchResult;

use crate::primitives::*;
use sp_std::vec::Vec;

/// Abstraction over a Priority system.
#[allow(clippy::upper_case_acronyms)]
pub trait Priority<BoundedString, AccountId, BoundedPriorities> {
	fn priority_set(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		priorities: BoundedPriorities,
	) -> DispatchResult;
}
