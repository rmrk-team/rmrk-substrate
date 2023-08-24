// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md
#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::cmp::Eq;

use frame_support::pallet_prelude::*;
use sp_runtime::Permill;

use crate::{
	budget::Budget,
	primitives::{ResourceId, SlotId},
	serialize,
};
use sp_std::result::Result;

#[cfg(feature = "std")]
use serde::Serialize;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize))]
pub enum AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId> {
	AccountId(AccountId),
	CollectionAndNftTuple(CollectionId, NftId),
}

/// Royalty information (recipient and amount)
#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct RoyaltyInfo<AccountId, RoyaltyAmount> {
	/// Recipient (AccountId) of the royalty
	pub recipient: AccountId,
	/// Amount (Permill) of the royalty
	pub amount: RoyaltyAmount,
}

/// Nft info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq, Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			AccountId: Serialize,
			RoyaltyAmount: Serialize,
			BoundedString: AsRef<[u8]>,
			NftId: Serialize,
			CollectionId: Serialize,
		"#)
)]
pub struct NftInfo<AccountId, RoyaltyAmount, BoundedString, CollectionId, NftId> {
	/// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
	pub owner: AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId>,
	/// Royalty (optional)
	pub royalty: Option<RoyaltyInfo<AccountId, RoyaltyAmount>>,

	/// Arbitrary data about an instance, e.g. IPFS hash
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub metadata: BoundedString,

	/// Contains an optional `ResourceId` and the `SlotId` for the equipped nft.
	pub equipped: Option<(ResourceId, SlotId)>,
	/// Pending state (if sent to NFT)
	pub pending: bool,
	/// transferability ( non-transferable is "souldbound" )
	pub transferable: bool,
}

#[cfg_attr(feature = "std", derive(PartialEq, Eq, Serialize))]
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct NftChild<CollectionId, NftId> {
	pub collection_id: CollectionId,
	pub nft_id: NftId,
}

/// Abstraction over a Nft system.
#[allow(clippy::upper_case_acronyms)]
pub trait Nft<AccountId, BoundedString, BoundedResourceVec, CollectionId, NftId> {
	fn nft_mint(
		sender: AccountId,
		owner: AccountId,
		nft_id: NftId,
		collection_id: CollectionId,
		royalty_recipient: Option<AccountId>,
		royalty_amount: Option<Permill>,
		metadata: BoundedString,
		transferable: bool,
		resources: Option<BoundedResourceVec>,
	) -> Result<(CollectionId, NftId), DispatchError>;
	fn nft_mint_directly_to_nft(
		sender: AccountId,
		owner: (CollectionId, NftId),
		nft_id: NftId,
		collection_id: CollectionId,
		royalty_recipient: Option<AccountId>,
		royalty_amount: Option<Permill>,
		metadata: BoundedString,
		transferable: bool,
		resources: Option<BoundedResourceVec>,
	) -> Result<(CollectionId, NftId), DispatchError>;
	fn nft_burn(
		owner: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		budget: &dyn Budget,
	) -> DispatchResultWithPostInfo;
	fn nft_send(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId>,
	) -> Result<(AccountId, bool), DispatchError>;
	fn nft_accept(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId>,
	) -> Result<(AccountId, CollectionId, NftId), DispatchError>;
	fn nft_reject(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
	) -> DispatchResultWithPostInfo;
}
