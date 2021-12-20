use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::{cmp::Eq, result};

use frame_support::pallet_prelude::*;
use sp_runtime::Permill;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId> {
	AccountId(AccountId),
	CollectionAndNftTuple(CollectionId, NftId),
}

/// Nft info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct NftInfo<AccountId, BoundedString, CollectionId, NftId> {
	/// The rootowner of the account, must be an account
	pub rootowner: AccountId,
	/// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
	pub owner: AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId>,
	/// The user account which receives the royalty
	pub recipient: AccountId,
	/// Royalty in per mille (1/1000)
	pub royalty: Permill,
	/// Arbitrary data about an instance, e.g. IPFS hash
	pub metadata: BoundedString,
}

/// Abstraction over a Nft system.
#[allow(clippy::upper_case_acronyms)]
pub trait Nft<AccountId, BoundedString> {
	type NftId: Default + Copy;
	type CollectionId: Default + Copy;
	type MaxRecursions: Get<u32>;

	fn mint_nft(
		sender: AccountId,
		owner: AccountId,
		collection_id: Self::CollectionId,
		recipient: Option<AccountId>,
		royalty: Option<Permill>,
		metadata: BoundedString,
	) -> sp_std::result::Result<(Self::CollectionId, Self::NftId), DispatchError>;
	fn burn_nft(
		collection_id: Self::CollectionId,
		nft_id: Self::NftId,
		max_recursions: u32,
	) -> sp_std::result::Result<(Self::CollectionId, Self::NftId), DispatchError>;
	fn send(
		sender: AccountId,
		collection_id: Self::CollectionId,
		nft_id: Self::NftId,
		new_owner: AccountIdOrCollectionNftTuple<AccountId, Self::CollectionId, Self::NftId>,
		max_recursions: u32,
	) -> sp_std::result::Result<(Self::CollectionId, Self::NftId), DispatchError>;
}
