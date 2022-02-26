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

#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct Theme<BoundedString> {
	/// Name of the theme
	pub name: BoundedString,
	/// Theme properties
	pub properties: Vec<ThemeProperty<BoundedString>>,
}

#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct ThemeProperty<BoundedString> {
	/// Key of the property
	pub key: BoundedString,
	/// Value of the property
	pub value: BoundedString,
	/// Inheritability
	pub inherit: Option<bool>,
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
		item: (CollectionId, NftId),
		equipper: (CollectionId, NftId),
		base_id: BaseId, // Maybe BaseId ?
		slot: SlotId // Maybe SlotId ?
)-> Result<(CollectionId, NftId, BaseId, SlotId, bool), DispatchError>;
	fn do_equippable(
		issuer: AccountId, 
		base_id: BaseId,
		slot: SlotId,
		equippables: EquippableList,
	)-> Result<(BaseId, SlotId), DispatchError>;
	fn add_theme(
		issuer: AccountId,
		base_id: BaseId,
		theme: Theme<BoundedString>,
	) -> Result<(), DispatchError>;
}
