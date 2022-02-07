use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError};
use crate::primitives::*;

/// Fixed parts are references to static content, like an IPFS hash of an SVG file
/// Slot parts have a type of slot and an optional static resource src. 
/// They are meant to visually accept the resources of other NFTs into them.
pub enum PartType {
	Fixed,
	Slot
}

/// Parts is a full catalogue of Parts which an NFT 
/// using this base as a resource can cherry-pick from in order to achieve a composite render.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct PartInfo<BoundedString> {
	/// IPFS source
	pub src: BoundedString,
}

// Abstraction over a Part system.
pub trait Part<AccountId, BoundedString> {
	fn part_create(
		src: BoundedString,
	) -> Result<PartId, DispatchError>;
}
