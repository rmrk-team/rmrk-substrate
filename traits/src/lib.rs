#![cfg_attr(not(feature = "std"), no_std)]

pub mod collection;
pub mod nft;
pub mod priority;
pub mod property;
pub mod resource;
pub mod list;

pub use collection::{Collection, CollectionInfo};
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftInfo};
pub use priority::Priority;
pub use property::Property;
pub use resource::{Resource, ResourceInfo};
pub use list::{List, ListInfo};
pub mod primitives {
	pub type CollectionId = u32;
	pub type ResourceId = u32;
	pub type NftId = u32;
	pub type ListId = u128;
}
