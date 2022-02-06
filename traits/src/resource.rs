#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::cmp::Eq;
use sp_std::{vec::Vec};

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceInfo<ResourceId, BoundedString> {
	/// id is a 5-character string of reasonable uniqueness.
	/// The combination of base ID and resource id should be unique across the entire RMRK
	/// ecosystem which
	pub id: ResourceId,

	/// If resource is sent to non-rootowned NFT, pending will be false and need to be accepted
	pub pending: bool,

	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: Option<BoundedString>,
	/// If the resource is Media, the base property is absent. Media src should be a URI like an
	/// IPFS hash.
	pub src: Option<BoundedString>,
	pub metadata: Option<BoundedString>,
	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	/// The baseslot will be composed of two dot-delimited values, like so:
	/// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
	/// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird
	pub slot: Option<BoundedString>,
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




/*
		Decoded:
		{
			"base":"base-8788686-KANBASE",
			"id":"6BrBxg_X",
			"parts":["1F60F_beak","var4_body","1F60F_eyes","var4_footLeft","var4_footRight",
				"var4_handLeft","1f48e_handRight","1f48e_head","var4_tail","var4_wingLeft","1f48e_wingRight","card_le","background",
				"foreground", "headwear","backpack","objectleft","objectright","necklace","gem_empty2","gem_empty3","gem_empty4"],
			"thumb":"ipfs://ipfs/QmR3rK1P4n24PPqvfjGYNXWixPJpyBKTV6rYzAS2TYHLpT",
			"themeId":"eggplant"
		}

		Decoded:
		{
			"slot":"base-8788686-KANBASE.objectright", ->
				"base"
				"slot"
			"id":"fG-kSh3M",
			"src":"ipfs://ipfs/QmPAdcr87vu2mENZbJNg14yoRRjtUZ6vAYWN5oZQczyCvo/sparklers22_objectright.svg",
			"thumb":"ipfs://ipfs/QmPAdcr87vu2mENZbJNg14yoRRjtUZ6vAYWN5oZQczyCvo/sparklers22_object_thumb.png"
		}

*/

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ResourceType<BaseId, SlotId, ResourceId, BoundedString> {
	Base(NoncomposableResource<BaseId, ResourceId, BoundedString>),
	Slot(ComposableResource<BaseId, SlotId, ResourceId, BoundedString>),
}


#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NoncomposableResource<BaseId, ResourceId, BoundedString> {
	pub base: BaseId,
	pub id: ResourceId,
	pub parts: Vec<BoundedString>, // maybe switch to Vec<Part> ?
	pub src: Option<BoundedString>,
	pub thumb: Option<BoundedString>,
}


#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ComposableResource<BaseId, SlotId, ResourceId, BoundedString> {
	pub base: BaseId,
	pub slot_id: SlotId,
	pub id: ResourceId,
	pub src: BoundedString,
	pub thumb: Option<BoundedString>,
	pub themeId: Option<BoundedString>,
}


/// Abstraction over a Resource system.
pub trait NewResource<AccountId, CollectionId, NftId, BaseId, SlotId, ResourceId, BoundedString> {
	fn new_resource_add(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource: ResourceType<BaseId, SlotId, ResourceId, BoundedString>,
	) -> Result<ResourceId, DispatchError>;
	fn accept(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> DispatchResult;
}

/// Abstraction over a Resource system.
pub trait Resource<BoundedString, AccountId> {
	fn resource_add(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		base: Option<BoundedString>,
		src: Option<BoundedString>,
		metadata: Option<BoundedString>,
		slot: Option<BoundedString>,
		license: Option<BoundedString>,
		thumb: Option<BoundedString>,
	) -> Result<ResourceId, DispatchError>;
	fn accept(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> DispatchResult;
}
