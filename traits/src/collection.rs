// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};

#[cfg(feature = "std")]
use serde::Serialize;

use crate::{primitives::*, serialize};
use sp_std::result::Result;

/// Collection info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq, Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			AccountId: Serialize,
			BoundedString: AsRef<[u8]>,
			BoundedSymbol: AsRef<[u8]>
		"#)
)]
pub struct CollectionInfo<BoundedString, BoundedSymbol, AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,

	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub metadata: BoundedString,
	pub max: Option<u32>,

	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
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
