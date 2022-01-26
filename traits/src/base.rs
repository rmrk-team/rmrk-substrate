use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug};
use sp_std::cmp::Eq;

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct BaseInfo<AccountId, BoundedString> {
	/// Original creator of the Base
	pub issuer: AccountId,
	/// Specifies how an NFT should be rendered, ie "svg"
	pub base_type: BoundedString,
	/// User provided symbol during Base creation
	pub symbol: BoundedString,
}