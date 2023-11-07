// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-market.
// License: Apache 2.0 modified by RMRK, see LICENSE.md

use super::*;
use crate as pallet_rmrk_market;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, Everything},
	weights::Weight,
};
use frame_system as system;
use frame_system::EnsureRoot;
use sp_core::{crypto::AccountId32, H256};

use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

mod rmrk_market {
	// Re-export needed for `impl_outer_event!`
	pub use super::super::*;
}

type AccountId = AccountId32;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		RmrkCore: pallet_rmrk_core::{Pallet, Call, Event<T>, Storage},
		RmrkMarket: pallet_rmrk_market::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = Weight::from_ref_time(1024);
	pub const MaximumBlockLength: u32 = 2 * 1024;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Test>;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub MaxMetadataLength: u32 = 256;
	pub const ResourceSymbolLimit: u32 = 10;
	pub const PartsLimit: u32 = 10;
	pub const MaxPriorities: u32 = 3;
	pub const CollectionSymbolLimit: u32 = 100;
	pub const MaxResourcesOnMint: u32 = 3;
	pub const PropertiesLimit: u32 = 15;
	pub const NestingBudget: u32 = 10;
}

#[cfg(feature = "runtime-benchmarks")]
use pallet_rmrk_core::RmrkBenchmark;
use system::EnsureSigned;

impl pallet_rmrk_core::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ProtocolOrigin = EnsureSigned<AccountId>;
	type ResourceSymbolLimit = ResourceSymbolLimit;
	type PartsLimit = PartsLimit;
	type MaxPriorities = MaxPriorities;
	type CollectionSymbolLimit = CollectionSymbolLimit;
	type MaxResourcesOnMint = MaxResourcesOnMint;
	type PropertiesLimit = PropertiesLimit;
	type NestingBudget = NestingBudget;
	type WeightInfo = pallet_rmrk_core::weights::SubstrateWeight<Test>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = RmrkBenchmark;
	type TransferHooks = ();
}

parameter_types! {
	pub const CollectionDeposit: Balance = 10_000 * RMRK; // 1 UNIT deposit to create asset class
	pub const ItemDeposit: Balance = 100 * RMRK; // 1/100 UNIT deposit to create asset instance
	pub const KeyLimit: u32 = 32;	// Max 32 bytes per key
	pub const ValueLimit: u32 = 64;	// Max 64 bytes per value
	pub const UniquesMetadataDepositBase: Balance = 1000 * RMRK;
	pub const AttributeDepositBase: Balance = 100 * RMRK;
	pub const DepositPerByte: Balance = 10 * RMRK;
	pub const UniquesStringLimit: u32 = 32;
}

impl pallet_uniques::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Locker = pallet_rmrk_core::Pallet<Test>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = UniquesMetadataDepositBase;
	type AttributeDepositBase = AttributeDepositBase;
	type DepositPerByte = DepositPerByte;
	type StringLimit = UniquesStringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
	// type InstanceReserveStrategy = NFT;
}

parameter_types! {
	pub const MinimumOfferAmount: Balance = 50 * UNITS;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ProtocolOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type MinimumOfferAmount = MinimumOfferAmount;
	type WeightInfo = weights::SubstrateWeight<Test>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = RmrkBenchmark;
}

pub const ALICE: AccountId = AccountId::new([1u8; 32]);
pub const BOB: AccountId = AccountId::new([2u8; 32]);
pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
pub const UNITS: Balance = 100_000_000_000;
pub const RMRK: Balance = 1;
pub const COLLECTION_ID_0: <Test as pallet_uniques::Config>::CollectionId = 0;
pub const NFT_ID_0: <Test as pallet_uniques::Config>::ItemId = 0;
pub const NFT_ID_1: <Test as pallet_uniques::Config>::ItemId = 1;
pub const NOT_EXISTING_NFT_ID: <Test as pallet_uniques::Config>::ItemId = 999;
pub const MIN_OFFER_ON_NFT: Balance = 50 * UNITS;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(ALICE, 100_000 * UNITS),
			(BOB, 200_000 * UNITS),
			(CHARLIE, 300_000 * UNITS),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
