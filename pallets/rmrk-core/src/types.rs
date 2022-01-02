use frame_support::pallet_prelude::*;
use sp_runtime::Permill;

use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use rmrk_traits::{primitives::*, AccountIdOrCollectionNftTuple};

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassInfo<BoundedString, AccountId> {
	/// Arbitrary data about a class, e.g. IPFS hash
	pub issuer: AccountId,
	pub metadata: BoundedString,
	pub max: u32,
	pub symbol: BoundedString,
}

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
pub struct InstanceInfo<AccountId, BoundedString> {
	/// The rootowner of the account, must be an account
	pub rootowner: AccountId,
	/// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
	pub owner: AccountIdOrCollectionNftTuple<AccountId>,
	/// The user account which receives the royalty
	pub recipient: AccountId,
	/// Royalty in per mille (1/1000)
	pub royalty: Permill,
	/// Arbitrary data about an instance, e.g. IPFS hash
	pub metadata: BoundedString,
}
