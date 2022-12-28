
//! Autogenerated weights for `pallet_rmrk_core`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ilias-MBP.localdomain`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/rmrk-substrate
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_rmrk_core
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/rmrk-core/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn mint_nft() -> Weight;
	fn mint_nft_directly_to_nft() -> Weight;
	fn destroy_collection() -> Weight;
	fn send_to_account() -> Weight;
	fn send_to_nft() -> Weight;
	fn burn_nft(n: u32) -> Weight;
	fn accept_nft() -> Weight;
	fn reject_nft(n: u32) -> Weight;
	fn change_collection_issuer() -> Weight;
	fn set_property() -> Weight;
	fn lock_collection() -> Weight;
	fn add_basic_resource() -> Weight;
	fn add_composable_resource() -> Weight;
	fn add_slot_resource() -> Weight;
	fn accept_resource() -> Weight;
	fn remove_resource() -> Weight;
	fn accept_resource_removal() -> Weight;
	fn set_priority(n: u32) -> Weight;
	fn replace_resource() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Uniques Class (r:1 w:1)
	// Storage: RmrkCore Collections (r:0 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(38_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: RmrkCore Nfts (r:1 w:1)
	// Storage: RmrkCore Collections (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint_nft() -> Weight {
		Weight::from_ref_time(55_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: RmrkCore Nfts (r:1 w:1)
	// Storage: RmrkCore Collections (r:1 w:1)
	// Storage: Uniques Asset (r:2 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: RmrkCore Children (r:0 w:1)
	// Storage: Uniques Account (r:0 w:1)
	fn mint_nft_directly_to_nft() -> Weight {
		Weight::from_ref_time(60_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	fn destroy_collection() -> Weight {
		Weight::from_ref_time(61_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: RmrkCore Nfts (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn send_to_account() -> Weight {
		Weight::from_ref_time(50_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Uniques Asset (r:2 w:1)
	// Storage: RmrkCore Nfts (r:2 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: RmrkCore Children (r:0 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn send_to_nft() -> Weight {
		Weight::from_ref_time(58_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: RmrkCore Nfts (r:1 w:1)
	// Storage: RmrkCore Children (r:1 w:0)
	// Storage: RmrkCore Collections (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// The range of component `n` is `[1, 20]`.
	fn burn_nft(n: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 400_000
			.saturating_add(Weight::from_ref_time(79_057_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((5 as u64).saturating_mul(n as u64)))
	}
	// Storage: Uniques Asset (r:2 w:0)
	// Storage: RmrkCore Nfts (r:1 w:1)
	fn accept_nft() -> Weight {
		Weight::from_ref_time(35_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Uniques Asset (r:2 w:1)
	// Storage: RmrkCore Nfts (r:1 w:1)
	// Storage: RmrkCore Children (r:2 w:1)
	// Storage: RmrkCore Collections (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// The range of component `n` is `[1, 20]`.
	fn reject_nft(n: u32, ) -> Weight {
		Weight::from_ref_time(82_707_000 as u64)
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(128_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:1)
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn change_collection_issuer() -> Weight {
		Weight::from_ref_time(56_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Properties (r:0 w:1)
	fn set_property() -> Weight {
		Weight::from_ref_time(29_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:1)
	fn lock_collection() -> Weight {
		Weight::from_ref_time(22_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: RmrkCore Resources (r:1 w:1)
	fn replace_resource() -> Weight {
		Weight::from_ref_time(25_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}	
	// Storage: RmrkCore Collections (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: RmrkCore Resources (r:1 w:1)
	fn add_basic_resource() -> Weight {
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: RmrkCore Resources (r:1 w:1)
	// Storage: RmrkCore EquippableBases (r:0 w:1)
	fn add_composable_resource() -> Weight {
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: RmrkCore Resources (r:1 w:1)
	// Storage: RmrkCore EquippableSlots (r:0 w:1)
	fn add_slot_resource() -> Weight {
		Weight::from_ref_time(35_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: RmrkCore Resources (r:1 w:1)
	fn accept_resource() -> Weight {
		Weight::from_ref_time(32_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: RmrkCore Collections (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Resources (r:1 w:1)
	fn remove_resource() -> Weight {
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Resources (r:1 w:1)
	fn accept_resource_removal() -> Weight {
		Weight::from_ref_time(30_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: RmrkCore Lock (r:1 w:0)
	// Storage: RmrkCore Priorities (r:0 w:1)
	/// The range of component `n` is `[1, 25]`.
	fn set_priority(n: u32, ) -> Weight {
		Weight::from_ref_time(21_901_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(2_436_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}