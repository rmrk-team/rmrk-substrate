// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use serde::Serialize;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::{cmp::Eq, result::Result};

use crate::{
	primitives::{BaseId, PartId, ResourceId, SlotId},
	serialize,
};
#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(feature = "std", serde(bound = "BoundedString: AsRef<[u8]>"))]
pub struct BasicResource<BoundedString> {
	/// Reference to IPFS location of metadata
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub metadata: BoundedString,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedParts: AsRef<[PartId]>
		"#)
)]
pub struct ComposableResource<BoundedString, BoundedParts> {
	/// If a resource is composed, it will have an array of parts that compose it
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub parts: BoundedParts,

	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: BaseId,

	/// Reference to IPFS location of metadata
	#[cfg_attr(feature = "std", serde(with = "serialize::opt_vec"))]
	pub metadata: Option<BoundedString>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	pub slot: Option<(BaseId, SlotId)>,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(feature = "std", serde(bound = "BoundedString: AsRef<[u8]>"))]
pub struct SlotResource<BoundedString> {
	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: BaseId,

	/// Reference to IPFS location of metadata
	#[cfg_attr(feature = "std", serde(with = "serialize::opt_vec"))]
	pub metadata: Option<BoundedString>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	/// The baseslot will be composed of two dot-delimited values, like so:
	/// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
	/// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird
	pub slot: SlotId,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedParts: AsRef<[PartId]>
		"#)
)]
pub enum ResourceTypes<BoundedString, BoundedParts> {
	Basic(BasicResource<BoundedString>),
	Composable(ComposableResource<BoundedString, BoundedParts>),
	Slot(SlotResource<BoundedString>),
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedParts: AsRef<[PartId]>
		"#)
)]
pub struct ResourceInfo<BoundedString, BoundedParts> {
	pub id: ResourceId,

	/// Resource
	pub resource: ResourceTypes<BoundedString, BoundedParts>,

	/// If resource is sent to non-rootowned NFT, pending will be false and need to be accepted
	pub pending: bool,

	/// If resource removal request is sent by non-rootowned NFT, pending will be true and need to
	/// be accepted
	pub pending_removal: bool,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedParts: AsRef<[PartId]>
		"#)
)]
pub struct ResourceInfoMin<BoundedString, BoundedParts> {
	pub id: ResourceId,
	pub resource: ResourceTypes<BoundedString, BoundedParts>,
}

/// Abstraction over a Resource system.
pub trait Resource<BoundedString, AccountId, BoundedPart, CollectionId, NftId> {
	fn resource_add(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource: ResourceTypes<BoundedString, BoundedPart>,
		pending: bool,
		resource_id: ResourceId,
	) -> Result<ResourceId, DispatchError>;
	fn accept(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> DispatchResult;
	fn resource_remove(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
		pending_resource: bool,
	) -> DispatchResult;
	fn resource_replace(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource: ResourceTypes<BoundedString, BoundedPart>,
		resource_id: ResourceId,
	) -> DispatchResult;
	fn accept_removal(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> DispatchResult;
}
