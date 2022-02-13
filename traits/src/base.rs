use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError};
use sp_std::cmp::Eq;
use crate::primitives::*;
use sp_std::{vec::Vec};

// // #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
// #[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
// pub struct Equipping<BoundedString> {
// 	// base
// 	// pub base: BaseId,
// 	// slot
// 	pub slot: BoundedString,
// }

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
pub struct FixedPart<BoundedString> {
	pub id: PartId,
	pub z: ZIndex,
	pub src: BoundedString,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
pub enum EquippableList {
	All,
	Empty,
	Custom(Vec<CollectionId>)
}

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
pub struct SlotPart<BoundedString> {
	pub id: PartId,
	pub equippable: EquippableList,
	pub src: BoundedString,
	pub z: ZIndex,
}

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
pub enum NewPartTypes<BoundedString> {
	FixedPart(FixedPart<BoundedString>),
	SlotPart(SlotPart<BoundedString>), 
}

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct BaseInfo<AccountId, BoundedString> {
	/// Original creator of the Base
	pub issuer: AccountId,
	/// Specifies how an NFT should be rendered, ie "svg"
	pub base_type: BoundedString,
	/// User provided symbol during Base creation
	pub symbol: BoundedString,
	/// Parts, full list of both Fixed and Slot parts
	pub parts: Vec<NewPartTypes<BoundedString>>,
}

// Abstraction over a Base system.
pub trait Base<AccountId, CollectionId, NftId, BoundedString> {
	fn base_create(
		issuer: AccountId,
		base_type: BoundedString,
		symbol: BoundedString,
		parts: Vec<NewPartTypes<BoundedString>>
) -> Result<BaseId, DispatchError>;
	fn do_equip(
		issuer: AccountId, // Maybe don't need?
		equipping_item_collection_id: CollectionId,
		equipping_item_nft_id: NftId,
		equipper_collection_id: CollectionId,
		equipper_nft_id: NftId,
		base_id: u32, // Maybe BaseId ?
		slot: u32 // Maybe SlotId ?
)-> Result<(), DispatchError>;
	fn do_equippable(
		issuer: AccountId, 
		base_id: BaseId,
		slot: SlotId,
		equippables: EquippableList,
	)-> Result<(), DispatchError>;
}
