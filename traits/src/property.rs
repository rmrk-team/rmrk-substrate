use sp_runtime::DispatchResult;

use crate::primitives::*;

/// Abstraction over a Property system.
#[allow(clippy::upper_case_acronyms)]
pub trait Property<KeyLimit, ValueLimit, AccountId> {
	fn property_set(
		sender: AccountId,
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimit,
		value: ValueLimit,
	) -> DispatchResult;
}
