use super::{
	part::{EquippableList, PartType},
	theme::Theme,
};
use crate::primitives::{BaseId, SlotId};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::vec::Vec;
use frame_support::pallet_prelude::MaxEncodedLen;


#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BaseInfo<AccountId, BoundedString, BoundedParts> {
	/// Original creator of the Base
	pub issuer: AccountId,
	/// Specifies how an NFT should be rendered, ie "svg"
	pub base_type: BoundedString,
	/// User provided symbol during Base creation
	pub symbol: BoundedString,
	/// Parts, full list of both Fixed and Slot parts
	pub parts: BoundedParts,
}

// Abstraction over a Base system.
pub trait Base<AccountId, CollectionId, NftId, BoundedString, BoundedParts, BoundedCollectionList> {
	fn base_create(
		issuer: AccountId,
		base_type: BoundedString,
		symbol: BoundedString,
		parts: BoundedParts,
	) -> Result<BaseId, DispatchError>;
	fn base_change_issuer(
		base_id: BaseId,
		new_issuer: AccountId,
	) -> Result<(AccountId, CollectionId), DispatchError>;
	fn do_equip(
		issuer: AccountId, // Maybe don't need?
		item: (CollectionId, NftId),
		equipper: (CollectionId, NftId),
		base_id: BaseId, // Maybe BaseId ?
		slot: SlotId,    // Maybe SlotId ?
	) -> Result<(CollectionId, NftId, BaseId, SlotId, bool), DispatchError>;
	fn do_equippable(
		issuer: AccountId,
		base_id: BaseId,
		slot: SlotId,
		equippables: EquippableList<BoundedCollectionList>,
	) -> Result<(BaseId, SlotId), DispatchError>;
	fn add_theme(
		issuer: AccountId,
		base_id: BaseId,
		theme: Theme<BoundedString>,
	) -> Result<(), DispatchError>;
}
