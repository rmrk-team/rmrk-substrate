use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct Theme<BoundedString> {
	/// Name of the theme
	pub name: BoundedString,
	/// Theme properties
	pub properties: Vec<ThemeProperty<BoundedString>>,
	/// Inheritability
	pub inherit: bool,
}

#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct ThemeProperty<BoundedString> {
	/// Key of the property
	pub key: BoundedString,
	/// Value of the property
	pub value: BoundedString,
}
