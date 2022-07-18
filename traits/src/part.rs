// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use crate::{
	primitives::*,
	serialize,
};
use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::Serialize;

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(bound = "BoundedString: AsRef<[u8]>")
)]
pub struct FixedPart<BoundedString> {
	pub id: PartId,
	pub z: ZIndex,

	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub src: BoundedString,
}

#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(bound = "BoundedCollectionList: AsRef<[CollectionId]>")
)]
pub enum EquippableList<BoundedCollectionList> {
	All,
	Empty,
	Custom(
		#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
		BoundedCollectionList
	),
}

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(
		bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedCollectionList: AsRef<[CollectionId]>
		"#
	)
)]
pub struct SlotPart<BoundedString, BoundedCollectionList> {
	pub id: PartId,
	pub equippable: EquippableList<BoundedCollectionList>,
	#[cfg_attr(feature = "std", serde(with = "serialize::opt_vec"))]
	pub src: Option<BoundedString>,
	pub z: ZIndex,
}

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(
		bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedCollectionList: AsRef<[CollectionId]>
		"#
	)
)]
pub enum PartType<BoundedString, BoundedCollectionList> {
	FixedPart(FixedPart<BoundedString>),
	SlotPart(SlotPart<BoundedString, BoundedCollectionList>),
}
