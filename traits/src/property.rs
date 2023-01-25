// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchResult;

#[cfg(feature = "std")]
use serde::Serialize;

use crate::serialize;

#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			BoundedKey: AsRef<[u8]>,
			BoundedValue: AsRef<[u8]>
		"#)
)]
pub struct PropertyInfo<BoundedKey, BoundedValue> {
	/// Key of the property
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub key: BoundedKey,

	/// Value of the property
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub value: BoundedValue,
}

/// Abstraction over a Property system.
#[allow(clippy::upper_case_acronyms)]
pub trait Property<KeyLimit, ValueLimit, AccountId, CollectionId, NftId> {
	fn property_set(
		sender: AccountId,
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimit,
		value: ValueLimit,
	) -> DispatchResult;

	/// Internal function to set a property that can be called from `Origin::root()` downstream.
	fn do_set_property(
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimit,
		value: ValueLimit,
	) -> DispatchResult;

	/// Internal function to remove a property that can be called from `Origin::root()` downstream.
	fn do_remove_property(
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimit,
	) -> DispatchResult;

	// Internal function to remove all of the properties that can be called from `Origin::root()`
	// downstream.
	fn do_remove_properties(
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		limit: u32,
	) -> DispatchResult;
}
