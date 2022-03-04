use crate::primitives::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

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
	Custom(Vec<CollectionId>),
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
pub enum PartType<BoundedString> {
	FixedPart(FixedPart<BoundedString>),
	SlotPart(SlotPart<BoundedString>),
}
