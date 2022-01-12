#![cfg_attr(not(feature = "std"), no_std)]

// use sp_std::prelude::*;

sp_api::decl_runtime_apis! {
	pub trait Nothing {
		fn nada();
	}
}
