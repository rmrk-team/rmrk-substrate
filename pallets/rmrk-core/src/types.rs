use frame_support::pallet_prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId> {
	AccountId(AccountId),
	CollectionAndNftTuple(CollectionId, NftId),
}

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassInfo<BoundedString, AccountId> {
	/// Arbitrary data about a class, e.g. IPFS hash
	pub issuer: AccountId,
	pub metadata: BoundedString,
}

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct InstanceInfo<AccountId, BoundedString, CollectionId, NftId> {
	/// The rootowner of the account, must be an account
	pub rootowner: AccountId,
	/// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
	pub owner: AccountIdOrCollectionNftTuple<AccountId, CollectionId, NftId>,
	/// The user account which receives the royalty
	pub author: AccountId,
	/// Royalty in percent in range 0-99
	pub royalty: u8,
	/// Arbitrary data about an instance, e.g. IPFS hash
	pub metadata: BoundedString,
}
