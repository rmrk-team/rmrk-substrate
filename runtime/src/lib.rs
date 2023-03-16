#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, Verify},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchError, MultiSignature,
};
use sp_std::{collections::btree_set::BTreeSet, prelude::*};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{
		tokens::nonfungibles::*, AsEnsureOriginWithArg, Contains, KeyOwnerProofSystem, Randomness,
		StorageInfo,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		IdentityFee, Weight,
	},
	BoundedVec, StorageValue,
};
use frame_system::EnsureSigned;

#[cfg(any(feature = "std", test))]
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

use pallet_rmrk_core::{CollectionInfoOf, InstanceInfoOf, PropertyInfoOf, ResourceInfoOf};
use pallet_rmrk_equip::{BaseInfoOf, BoundedThemeOf, PartTypeOf};
use rmrk_traits::{
	primitives::{BaseId, CollectionId, NftId, ResourceId},
	NftChild,
};

/// Import the template pallet.
pub use pallet_template;

pub use pallet_rmrk_core;

pub use pallet_rmrk_equip;
pub use pallet_rmrk_market;

/// An index to a block.
pub type BlockNumber = u32;

pub type Moment = u64;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

/// Constant values used within the runtime.
pub mod constants;
use constants::currency::*;

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	)
}

// To learn more about runtime versioning and what each of the following value means:
//   https://docs.substrate.io/v3/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("rmrk-substrate"),
	impl_name: create_runtime_str!("rmrk-substrate"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 1000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;
const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, MAX_POV_SIZE);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

pub struct BaseFilter;
impl Contains<RuntimeCall> for BaseFilter {
	fn contains(call: &RuntimeCall) -> bool {
		// Disable direct calls to pallet_uniques
		!matches!(
			call,
			RuntimeCall::Uniques(pallet_uniques::Call::approve_transfer { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::burn { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::cancel_approval { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::clear_collection_metadata { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::clear_metadata { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::create { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::destroy { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::force_item_status { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::force_create { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::freeze_collection { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::mint { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::redeposit { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::set_collection_metadata { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::thaw_collection { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::transfer { .. }) |
				RuntimeCall::Uniques(pallet_uniques::Call::transfer_ownership { .. })
		)
	}
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = BaseFilter;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub const MaxAuthorities: u32 = 32;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;

	type KeyOwnerProofSystem = ();

	type HandleEquivocation = ();
	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxLocks: u32 = 50;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = frame_support::traits::ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const TransactionByteFee: Balance = 1;
	pub OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

/// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const ResourceSymbolLimit: u32 = 10;
	pub const PartsLimit: u32 = 25;
	pub const MaxPriorities: u32 = 25;
	pub const CollectionSymbolLimit: u32 = 100;
	pub const MaxResourcesOnMint: u32 = 100;
	pub const PropertiesLimit: u32 = 25;
	pub const NestingBudget: u32 = 20;
}

#[cfg(feature = "runtime-benchmarks")]
use pallet_rmrk_core::RmrkBenchmark;

impl pallet_rmrk_core::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ProtocolOrigin = frame_system::EnsureRoot<AccountId>;
	type ResourceSymbolLimit = ResourceSymbolLimit;
	type PartsLimit = PartsLimit;
	type MaxPriorities = MaxPriorities;
	type CollectionSymbolLimit = CollectionSymbolLimit;
	type MaxResourcesOnMint = MaxResourcesOnMint;
	type PropertiesLimit = PropertiesLimit;
	type NestingBudget = NestingBudget;
	type WeightInfo = pallet_rmrk_core::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = RmrkBenchmark;
	type TransferHooks = ();
}

parameter_types! {
	pub const MinimumOfferAmount: Balance = UNITS / 10_000;
}

impl pallet_rmrk_market::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ProtocolOrigin = frame_system::EnsureRoot<AccountId>;
	type Currency = Balances;
	type MinimumOfferAmount = MinimumOfferAmount;
	type WeightInfo = pallet_rmrk_market::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = RmrkBenchmark;
}

parameter_types! {
	pub const CollectionDeposit: Balance = 10 * CENTS;
	pub const ItemDeposit: Balance = DOLLARS;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
	pub const UniquesMetadataDepositBase: Balance = 10 * CENTS;
	pub const AttributeDepositBase: Balance = 10 * CENTS;
	pub const DepositPerByte: Balance = CENTS;
	pub const UniquesStringLimit: u32 = 128;
	pub const MaxPropertiesPerTheme: u32 = 100;
	pub const MaxCollectionsEquippablePerPart: u32 = 100;
}

impl pallet_rmrk_equip::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxPropertiesPerTheme = MaxPropertiesPerTheme;
	type MaxCollectionsEquippablePerPart = MaxCollectionsEquippablePerPart;
	type WeightInfo = pallet_rmrk_equip::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = RmrkBenchmark;
}

impl pallet_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = pallet_rmrk_core::Pallet<Runtime>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = UniquesMetadataDepositBase;
	type AttributeDepositBase = AttributeDepositBase;
	type DepositPerByte = DepositPerByte;
	type StringLimit = UniquesStringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		// Include the custom logic from the pallet-template in the runtime.
		TemplateModule: pallet_template,
		RmrkEquip: pallet_rmrk_equip::{Pallet, Call, Event<T>, Storage},
		RmrkCore: pallet_rmrk_core::{Pallet, Call, Event<T>, Storage},
		RmrkMarket: pallet_rmrk_market::{Pallet, Call, Storage, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		Utility: pallet_utility::{Pallet, Call, Storage, Event},
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

fn option_filter_keys_to_set<StringLimit: frame_support::traits::Get<u32>>(
	filter_keys: Option<Vec<pallet_rmrk_rpc_runtime_api::PropertyKey>>,
) -> pallet_rmrk_rpc_runtime_api::Result<Option<BTreeSet<BoundedVec<u8, StringLimit>>>> {
	match filter_keys {
		Some(filter_keys) => {
			let tree = filter_keys
				.into_iter()
				.map(|filter_keys| -> pallet_rmrk_rpc_runtime_api::Result<BoundedVec<u8, StringLimit>> {
					filter_keys
						.try_into()
						.map_err(|_| DispatchError::Other("Can't read filter key"))
				})
				.collect::<pallet_rmrk_rpc_runtime_api::Result<BTreeSet<_>>>()?;
			Ok(Some(tree))
		},
		None => Ok(None),
	}
}

impl_runtime_apis! {
	impl pallet_rmrk_rpc_runtime_api::RmrkApi<
		Block,
		AccountId,
		CollectionInfoOf<Runtime>,
		InstanceInfoOf<Runtime>,
		ResourceInfoOf<Runtime>,
		PropertyInfoOf<Runtime>,
		BaseInfoOf<Runtime>,
		PartTypeOf<Runtime>,
		BoundedThemeOf<Runtime>
	> for Runtime
	{
		fn collection_by_id(id: CollectionId) -> pallet_rmrk_rpc_runtime_api::Result<Option<CollectionInfoOf<Runtime>>> {
			Ok(RmrkCore::collections(id))
		}

		fn nft_by_id(collection_id: CollectionId, nft_id: NftId) -> pallet_rmrk_rpc_runtime_api::Result<Option<InstanceInfoOf<Runtime>>> {
			Ok(RmrkCore::nfts(collection_id, nft_id))
		}

		fn account_tokens(account_id: AccountId, collection_id: CollectionId) -> pallet_rmrk_rpc_runtime_api::Result<Vec<NftId>> {
			Ok(Uniques::owned_in_collection(&collection_id, &account_id).collect())
		}

		fn nft_children(collection_id: CollectionId, nft_id: NftId) -> pallet_rmrk_rpc_runtime_api::Result<Vec<NftChild<CollectionId, NftId>>> {
			let children = RmrkCore::iterate_nft_children(collection_id, nft_id).collect();

			Ok(children)
		}

		fn nfts_owned_by(
			account_id: AccountId,
			start_index: Option<u32>,
			count: Option<u32>
		) -> pallet_rmrk_rpc_runtime_api::Result<Vec<(CollectionId, NftId, InstanceInfoOf<Runtime>)>> {
			let mut collections: Vec<CollectionId> = RmrkCore::iterate_collections().collect();
			collections.sort();

			let start_index = start_index.unwrap_or_default();
			let limit = count.unwrap_or(collections.len().saturating_sub(start_index as usize) as u32);

			let mut stored_collections = 0;
			let mut nfts = vec![];

			for collection_id in collections {
				// The nft ids owned by the `account_id` from the collection.
				let owned_nfts: Vec<NftId> = Uniques::owned_in_collection(&collection_id, &account_id).collect();

				// check if the user owns any nfts from the collection.
				if !owned_nfts.is_empty() {
					stored_collections += 1;
					if stored_collections <= start_index {
						continue;
					}
					if stored_collections - start_index > limit {
						break;
					}
				}

				// Get more information about the nfts.
				owned_nfts.into_iter().for_each(|nft_id| {
					if let Some(nft_info) = RmrkCore::nfts(collection_id, nft_id) {
						nfts.push((collection_id, nft_id, nft_info));
					}
				});
			}

			Ok(nfts)
		}

		fn properties_of_nfts_owned_by(
			account_id: AccountId,
			start_index: Option<u32>,
			count: Option<u32>
		) -> pallet_rmrk_rpc_runtime_api::Result<Vec<(CollectionId, NftId, Vec<PropertyInfoOf<Runtime>>)>>
		{
			let mut collections: Vec<CollectionId> = RmrkCore::iterate_collections().collect();
			collections.sort();

			let start_index = start_index.unwrap_or_default();
			let limit = count.unwrap_or(collections.len().saturating_sub(start_index as usize) as u32);

			let mut stored_collections = 0;
			let mut props_of_nfts = vec![];

			for collection_id in collections {
				// The nft ids owned by the `account_id` from the collection.
				let owned_nfts: Vec<NftId> = Uniques::owned_in_collection(&collection_id, &account_id).collect();

				// check if the user owns any nfts from the collection.
				if !owned_nfts.is_empty() {
					stored_collections += 1;
					if stored_collections <= start_index {
						continue;
					}
					if stored_collections - start_index > limit {
						break;
					}
				}

				// Get the properties for each of these NFTs.
				owned_nfts.into_iter().for_each(|nft_id| {
					let nft_props: Vec<PropertyInfoOf<Runtime>> = RmrkCore::query_properties(collection_id, Some(nft_id), None).collect();
					props_of_nfts.push((collection_id, nft_id, nft_props));
				});
			};

			Ok(props_of_nfts)
		}

		fn collection_properties(
			collection_id: CollectionId,
			filter_keys: Option<Vec<pallet_rmrk_rpc_runtime_api::PropertyKey>>
		) -> pallet_rmrk_rpc_runtime_api::Result<Vec<PropertyInfoOf<Runtime>>> {
			let nft_id = None;

			let filter_keys = option_filter_keys_to_set::<<Self as pallet_uniques::Config>::KeyLimit>(
				filter_keys
			)?;

			Ok(RmrkCore::query_properties(collection_id, nft_id, filter_keys).collect())
		}

		fn nft_properties(
			collection_id: CollectionId,
			nft_id: NftId,
			filter_keys: Option<Vec<pallet_rmrk_rpc_runtime_api::PropertyKey>>
		) -> pallet_rmrk_rpc_runtime_api::Result<Vec<PropertyInfoOf<Runtime>>> {
			let filter_keys = option_filter_keys_to_set::<<Self as pallet_uniques::Config>::KeyLimit>(
				filter_keys
			)?;

			Ok(RmrkCore::query_properties(collection_id, Some(nft_id), filter_keys).collect())
		}

		fn nft_resources(collection_id: CollectionId, nft_id: NftId) -> pallet_rmrk_rpc_runtime_api::Result<Vec<ResourceInfoOf<Runtime>>> {
			Ok(RmrkCore::iterate_resources(collection_id, nft_id).collect())
		}

		fn nft_resource_priority(collection_id: CollectionId, nft_id: NftId, resource_id: ResourceId) -> pallet_rmrk_rpc_runtime_api::Result<Option<u32>> {
			let priority = RmrkCore::priorities((collection_id, nft_id, resource_id));

			Ok(priority)
		}

		fn base(base_id: BaseId) -> pallet_rmrk_rpc_runtime_api::Result<Option<BaseInfoOf<Runtime>>> {
			Ok(RmrkEquip::bases(base_id))
		}

		fn base_parts(base_id: BaseId) -> pallet_rmrk_rpc_runtime_api::Result<Vec<PartTypeOf<Runtime>>> {
			Ok(RmrkEquip::iterate_part_types(base_id).collect())
		}

		fn theme_names(base_id: BaseId) -> pallet_rmrk_rpc_runtime_api::Result<Vec<pallet_rmrk_rpc_runtime_api::ThemeName>> {
			let names = RmrkEquip::iterate_theme_names(base_id)
				.map(|name| name.into())
				.collect();

			Ok(names)
		}

		fn theme(
			base_id: BaseId,
			theme_name: pallet_rmrk_rpc_runtime_api::ThemeName,
			filter_keys: Option<Vec<pallet_rmrk_rpc_runtime_api::PropertyKey>>
		) -> pallet_rmrk_rpc_runtime_api::Result<Option<BoundedThemeOf<Runtime>>> {
			use pallet_rmrk_equip::StringLimitOf;

			let theme_name: StringLimitOf<Self> = theme_name.try_into()
				.map_err(|_| DispatchError::Other("Can't read theme_name"))?;

			let filter_keys = option_filter_keys_to_set::<<Self as pallet_uniques::Config>::StringLimit>(
				filter_keys
			)?;

			let theme = RmrkEquip::get_theme(base_id, theme_name, filter_keys)?;
			Ok(theme)
		}
	}

	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			_authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmark!(list, extra, frame_benchmarking, BaselineBench::<Runtime>);
			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_balances, Balances);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			list_benchmark!(list, extra, pallet_template, TemplateModule);
			list_benchmark!(list, extra, pallet_rmrk_core, RmrkCore);
			list_benchmark!(list, extra, pallet_rmrk_equip, RmrkEquip);
			list_benchmark!(list, extra, pallet_rmrk_market, RmrkMarket);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_benchmarking, BaselineBench::<Runtime>);
			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pallet_template, TemplateModule);
			add_benchmark!(params, batches, pallet_rmrk_core, RmrkCore);
			add_benchmark!(params, batches, pallet_rmrk_equip, RmrkEquip);
			add_benchmark!(params, batches, pallet_rmrk_market, RmrkMarket);

			Ok(batches)
		}
	}
}
