// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

#![cfg_attr(not(feature = "std"), no_std)]

pub mod base;
pub mod collection;
pub mod nft;
pub mod part;
pub mod priority;
pub mod property;
pub mod resource;
pub mod theme;
pub mod phantom_type;
mod serialize;

pub use base::{Base, BaseInfo};
pub use part::{EquippableList, FixedPart, PartType, SlotPart};
pub use theme::{Theme, ThemeProperty};
// pub use part::{PartInfo};
pub use collection::{Collection, CollectionInfo};
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftInfo, RoyaltyInfo, NftChild};
pub use priority::Priority;
pub use property::{Property, PropertyInfo};
pub use resource::{
	BasicResource, ComposableResource, Resource, ResourceInfo, ResourceTypes, SlotResource,
};
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
