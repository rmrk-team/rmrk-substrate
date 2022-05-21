// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::Serialize;

use crate::serialize;

#[cfg_attr(feature = "std", derive(Eq, Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, Clone, PartialEq)]
#[cfg_attr(
	feature = "std",
	serde(
		bound = r#"
			BoundedString: AsRef<[u8]>,
			BoundedThemeProperties: AsRef<[ThemeProperty<BoundedString>]>,
		"#
	)
)]
pub struct Theme<BoundedString, BoundedThemeProperties> {
	/// Name of the theme
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub name: BoundedString,

	/// Theme properties
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub properties: BoundedThemeProperties,

	/// Inheritability
	pub inherit: bool,
}

#[cfg_attr(feature = "std", derive(Eq, Serialize))]
#[derive(Encode, Decode, Debug, TypeInfo, Clone, PartialEq)]
#[cfg_attr(
	feature = "std",
	serde(bound = "BoundedString: AsRef<[u8]>")
)]
pub struct ThemeProperty<BoundedString> {
	/// Key of the property
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub key: BoundedString,

	/// Value of the property
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub value: BoundedString,
}
