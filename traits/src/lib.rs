// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![cfg_attr(not(feature = "std"), no_std)]

pub mod base;
pub mod budget;
pub mod collection;
pub mod misc;
pub mod nft;
pub mod part;
pub mod phantom_type;
pub mod priority;
pub mod property;
pub mod resource;
mod serialize;
pub mod theme;

pub use base::{Base, BaseInfo};
pub use collection::{Collection, CollectionInfo};
pub use misc::TransferHooks;
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftChild, NftInfo, RoyaltyInfo};
pub use part::{EquippableList, FixedPart, PartType, SlotPart};
pub use priority::Priority;
pub use property::{Property, PropertyInfo};
pub use resource::{
	BasicResource, ComposableResource, Resource, ResourceInfo, ResourceInfoMin, ResourceTypes,
	SlotResource,
};
pub use theme::{Theme, ThemeProperty};
pub mod primitives {
	pub type CollectionId = u32;
	pub type ResourceId = u32;
	pub type NftId = u32;
	pub type BaseId = u32;
	pub type SlotId = u32;
	pub type PartId = u32;
	pub type ZIndex = u32;
}
pub use phantom_type::PhantomType;
