// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq)]
pub struct Theme<BoundedString, BoundedThemeProperties> {
	/// Name of the theme
	pub name: BoundedString,
	/// Theme properties
	pub properties: BoundedThemeProperties,
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
