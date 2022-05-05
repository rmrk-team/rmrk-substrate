use crate::primitives::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use frame_support::pallet_prelude::MaxEncodedLen;

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
pub struct FixedPart<BoundedString> {
	pub id: PartId,
	pub z: ZIndex,
	pub src: BoundedString,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
pub enum EquippableList<BoundedCollectionList> {
	All,
	Empty,
	Custom(BoundedCollectionList),
	// Custom(Vec<CollectionId>),
}

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
pub struct SlotPart<BoundedString, BoundedCollectionList> {
	pub id: PartId,
	pub equippable: EquippableList<BoundedCollectionList>,
	pub src: BoundedString,
	pub z: ZIndex,
}

// #[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen)]
pub enum PartType<BoundedString, BoundedCollectionList> {
	FixedPart(FixedPart<BoundedString>),
	SlotPart(SlotPart<BoundedString, BoundedCollectionList>),
}
