// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::{cmp::Eq, result::Result, vec::Vec};

use crate::primitives::*;

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BasicResource<BoundedString> {
	/// If the resource is Media, the base property is absent. Media src should be a URI like an
	/// IPFS hash.
	pub src: Option<BoundedString>,

	/// Reference to IPFS location of metadata
	pub metadata: Option<BoundedString>,

	/// Optional location or identier of license
	pub license: Option<BoundedString>,

	/// If the resource has the thumb property, this will be a URI to a thumbnail of the given
	/// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
	/// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
	/// another bird, showing the full render of one bird inside the other's inventory might be a
	/// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
	/// image that is lighter and faster to load but representative of this resource.
	pub thumb: Option<BoundedString>,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ComposableResource<BoundedString, BoundedParts> {
	/// If a resource is composed, it will have an array of parts that compose it
	pub parts: BoundedParts,

	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: BaseId,

	/// If the resource is Media, the base property is absent. Media src should be a URI like an
	/// IPFS hash.
	pub src: Option<BoundedString>,

	/// Reference to IPFS location of metadata
	pub metadata: Option<BoundedString>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	pub slot: Option<(BaseId, SlotId)>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	/// The baseslot will be composed of two dot-delimited values, like so:
	/// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
	/// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird

	/// Optional location or identier of license
	pub license: Option<BoundedString>,

	/// If the resource has the thumb property, this will be a URI to a thumbnail of the given
	/// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
	/// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
	/// another bird, showing the full render of one bird inside the other's inventory might be a
	/// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
	/// image that is lighter and faster to load but representative of this resource.
	pub thumb: Option<BoundedString>,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SlotResource<BoundedString> {
	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: BaseId,

	/// If the resource is Media, the base property is absent. Media src should be a URI like an
	/// IPFS hash.
	pub src: Option<BoundedString>,

	/// Reference to IPFS location of metadata
	pub metadata: Option<BoundedString>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	/// The baseslot will be composed of two dot-delimited values, like so:
	/// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
	/// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird
	pub slot: SlotId,

	/// The license field, if present, should contain a link to a license (IPFS or static HTTP
	/// url), or an identifier, like RMRK_nocopy or ipfs://ipfs/someHashOfLicense.
	pub license: Option<BoundedString>,

	/// If the resource has the thumb property, this will be a URI to a thumbnail of the given
	/// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
	/// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
	/// another bird, showing the full render of one bird inside the other's inventory might be a
	/// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
	/// image that is lighter and faster to load but representative of this resource.
	pub thumb: Option<BoundedString>,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ResourceTypes<BoundedString, BoundedParts> {
	Basic(BasicResource<BoundedString>),
	Composable(ComposableResource<BoundedString, BoundedParts>),
	Slot(SlotResource<BoundedString>),
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceInfo<BoundedString, BoundedParts> {
	/// id is a 5-character string of reasonable uniqueness.
	/// The combination of base ID and resource id should be unique across the entire RMRK
	/// ecosystem which
	pub id: ResourceId,

	/// Resource
	pub resource: ResourceTypes<BoundedString, BoundedParts>,

	/// If resource is sent to non-rootowned NFT, pending will be false and need to be accepted
	pub pending: bool,

	/// If resource removal request is sent by non-rootowned NFT, pending will be true and need to
	/// be accepted
	pub pending_removal: bool,
}

/// Abstraction over a Resource system.
pub trait Resource<BoundedString, AccountId, BoundedPart> {
	fn resource_add(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource: ResourceTypes<BoundedString, BoundedPart>,
		adding_on_mint: bool,
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
	) -> DispatchResult;
	fn accept_removal(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> DispatchResult;
}
