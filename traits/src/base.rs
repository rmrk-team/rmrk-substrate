// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use super::{part::EquippableList, theme::Theme};
use crate::{
	primitives::{BaseId, ResourceId, SlotId},
	serialize,
};
use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

#[cfg(feature = "std")]
use serde::Serialize;

#[cfg_attr(feature = "std", derive(PartialEq, Eq, Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			AccountId: Serialize,
			BoundedString: AsRef<[u8]>
		"#)
)]
pub struct BaseInfo<AccountId, BoundedString> {
	/// Original creator of the Base
	pub issuer: AccountId,

	/// Specifies how an NFT should be rendered, ie "svg"
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub base_type: BoundedString,

	/// User provided symbol during Base creation
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub symbol: BoundedString,
}

pub enum EquippableOperation<CollectionId, BoundedCollectionList> {
	/// Adds a new collection that is allowed to be equipped.
	Add(CollectionId),
	/// Removes a collection from the list of equippables.
	Remove(CollectionId),
	/// Overrides all of the equippables.
	Override(EquippableList<BoundedCollectionList>),
}

// Abstraction over a Base system.
pub trait Base<
	AccountId,
	CollectionId,
	NftId,
	BoundedString,
	BoundedParts,
	BoundedCollectionList,
	BoundedThemeProperties,
>
{
	fn base_create(
		issuer: AccountId,
		base_type: BoundedString,
		symbol: BoundedString,
		parts: BoundedParts,
	) -> Result<BaseId, DispatchError>;
	fn base_change_issuer(
		base_id: BaseId,
		new_issuer: AccountId,
	) -> Result<(AccountId, BaseId), DispatchError>;
	fn do_equip(
		issuer: AccountId, // Maybe don't need?
		item: (CollectionId, NftId),
		equipper: (CollectionId, NftId),
		resource_id: ResourceId,
		base_id: BaseId, // Maybe BaseId ?
		slot: SlotId,    // Maybe SlotId ?
	) -> Result<(CollectionId, NftId, BaseId, SlotId), DispatchError>;
	fn do_unequip(
		issuer: AccountId, // Maybe don't need?
		item: (CollectionId, NftId),
		equipper: (CollectionId, NftId),
		base_id: BaseId, // Maybe BaseId ?
		slot: SlotId,    // Maybe SlotId ?
	) -> Result<(CollectionId, NftId, BaseId, SlotId), DispatchError>;
	fn do_equippable(
		issuer: AccountId,
		base_id: BaseId,
		slot: SlotId,
		operation: EquippableOperation<CollectionId, BoundedCollectionList>,
	) -> Result<(BaseId, SlotId), DispatchError>;
	fn add_theme(
		issuer: AccountId,
		base_id: BaseId,
		theme: Theme<BoundedString, BoundedThemeProperties>,
	) -> Result<(), DispatchError>;
}
