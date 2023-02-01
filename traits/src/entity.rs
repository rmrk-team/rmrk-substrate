// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen, Clone, PartialEq)]
pub enum Entity<CollectionId, ItemId, BaseId, PartId> {
	/// The entity is a collection
	Collection(CollectionId),
	/// The entity is a collection nft
	Nft(CollectionId, ItemId),
	/// The entity is a base
	Base(BaseId),
	/// The entity is a Part
	Part(PartId),
}
