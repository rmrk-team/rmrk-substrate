//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::{sync::Arc, marker::PhantomData};

use jsonrpsee::RpcModule;
use rmrk_substrate_runtime::{opaque::Block, AccountId, Balance, Index};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{ProvideRuntimeApi, BlockT, BlockId, ApiExt, Encode, Decode};
use rmrk_substrate_runtime::Runtime as RmrkRuntime;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult},
	proc_macros::rpc
};

use rmrk_rpc::{RmrkApi as RmrkRuntimeApi, PropertyKey, ThemeName};
use rmrk_traits::{primitives::*, NftChild};

use pallet_rmrk_core::{CollectionInfoOf, InstanceInfoOf, ResourceInfoOf, PropertyInfoOf};
use pallet_rmrk_equip::{ThemeOf, BaseInfoOf, PartTypeOf};

macro_rules! pass_method {
	(
		$method_name:ident(
			$($(#[map(|$map_arg:ident| $map:expr)])? $name:ident: $ty:ty),*
			$(,)?
		) -> $result:ty
	) => {
		fn $method_name(
			&self,
			$(
				$name: $ty,
			)*
			at: Option<<Block as BlockT>::Hash>,
		) -> RpcResult<$result> {
			let api = self.client.runtime_api();
			let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
			let _api_version = if let Ok(Some(api_version)) =
				api.api_version::<
					dyn RmrkRuntimeApi<
						Block,
						AccountId,
						CollectionInfo,
						NftInfo,
						ResourceInfo,
						PropertyInfo,
						BaseInfo,
						PartType,
						Theme
					>
				>(&at)
			{
				api_version
			} else {
				unreachable!("The RMRK API is always available; qed");
			};

			let result = api.$method_name(&at, $($((|$map_arg: $ty| $map))? ($name)),*);

			let result = result.map_err(|e| RpcError::Custom(format!("sp_api error: {:?}", e)))?;

			result.map_err(|e| RpcError::Custom(format!("runtime error: {:?}", e)))
		}
	};
}

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// RMRK RPC API instance
pub struct RmrkApi<C, Block> {
	client: Arc<C>,
	_marker: PhantomData<Block>,
}

impl<C, Block> RmrkApi<C, Block> {
	/// Make new RMRK RPC API instance
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default()
		}
	}
}

/// RMRK RPC API
#[rpc(server, namespace = "rmrk")]
pub trait RmrkApi<
	BlockHash,
	AccountId,
	CollectionInfo,
	NftInfo,
	ResourceInfo,
	PropertyInfo,
	BaseInfo,
	PartType,
	Theme
>
{
	#[method(name = "lastCollectionIdx")]
	/// Get the latest created collection id
	fn last_collection_idx(&self, at: Option<BlockHash>) -> RpcResult<CollectionId>;

	#[method(name = "collectionById")]
	/// Get collection by id
	fn collection_by_id(&self, id: CollectionId, at: Option<BlockHash>) -> RpcResult<Option<CollectionInfo>>;

	#[method(name = "nftById")]
	/// Get NFT by collection id and NFT id
	fn nft_by_id(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>
	) -> RpcResult<Option<NftInfo>>;

	#[method(name = "accountTokens")]
	/// Get tokens owned by an account in a collection
	fn account_tokens(
		&self,
		account_id: AccountId,
		collection_id: CollectionId,
		at: Option<BlockHash>
	) -> RpcResult<Vec<NftId>>;

	#[method(name = "nftChildren")]
	/// Get NFT children
	fn nft_children(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>
	) -> RpcResult<Vec<NftChild>>;

	#[method(name = "collectionProperties")]
	/// Get collection properties
	fn collection_properties(
		&self,
		collection_id: CollectionId,
		filter_keys: Option<Vec<String>>,
		at: Option<BlockHash>
	) -> RpcResult<Vec<PropertyInfo>>;

	#[method(name = "nftProperties")]
	/// Get NFT properties
	fn nft_properties(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		filter_keys: Option<Vec<String>>,
		at: Option<BlockHash>
	) -> RpcResult<Vec<PropertyInfo>>;

	#[method(name = "nftResources")]
	/// Get NFT resources
	fn nft_resources(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>
	) -> RpcResult<Vec<ResourceInfo>>;

	#[method(name = "nftResourcePriorities")]
	/// Get NFT resource priorities
	fn nft_resource_priorities(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>
	) -> RpcResult<Vec<ResourceId>>;

	#[method(name = "base")]
	/// Get base info
	fn base(&self, base_id: BaseId, at: Option<BlockHash>) -> RpcResult<Option<BaseInfo>>;

	#[method(name = "baseParts")]
	/// Get all Base's parts
	fn base_parts(&self, base_id: BaseId, at: Option<BlockHash>) -> RpcResult<Vec<PartType>>;

	#[method(name = "themeNames")]
	/// Get Base's theme names
	fn theme_names(&self, base_id: BaseId, at: Option<BlockHash>) -> RpcResult<Vec<ThemeName>>;

	#[method(name = "themes")]
	/// Get Theme's keys values
	fn theme(
		&self,
		base_id: BaseId,
		theme_name: String,
		filter_keys: Option<Vec<String>>,
		at: Option<BlockHash>
	) -> RpcResult<Option<Theme>>;
}

impl<
	C, Block, AccountId,
	CollectionInfo,
	NftInfo,
	ResourceInfo,
	PropertyInfo,
	BaseInfo,
	PartType,
	Theme
> RmrkApiServer<
	<Block as BlockT>::Hash,
	AccountId,
	CollectionInfo,
	NftInfo,
	ResourceInfo,
	PropertyInfo,
	BaseInfo,
	PartType,
	Theme,
> for RmrkApi<C, Block>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C: Send + Sync + 'static,
	C::Api: RmrkRuntimeApi<
		Block, AccountId,
		CollectionInfo,
		NftInfo,
		ResourceInfo,
		PropertyInfo,
		BaseInfo,
		PartType,
		Theme
	>,
	AccountId: Encode,
	CollectionInfo: Decode,
	NftInfo: Decode,
	ResourceInfo: Decode,
	PropertyInfo: Decode,
	BaseInfo: Decode,
	PartType: Decode,
	Theme: Decode,
	Block: BlockT
{
	pass_method!(last_collection_idx() -> CollectionId);
	pass_method!(collection_by_id(id: CollectionId) -> Option<CollectionInfo>);
	pass_method!(nft_by_id(collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo>);
	pass_method!(account_tokens(account_id: AccountId, collection_id: CollectionId) -> Vec<NftId>);
	pass_method!(nft_children(collection_id: CollectionId, nft_id: NftId) -> Vec<NftChild>);
	pass_method!(
		collection_properties(
			collection_id: CollectionId,

			#[map(|keys| keys.map(string_keys_to_bytes_keys))]
			filter_keys: Option<Vec<String>>
		) -> Vec<PropertyInfo>
	);
	pass_method!(
		nft_properties(
			collection_id: CollectionId,
			nft_id: NftId,

			#[map(|keys| keys.map(string_keys_to_bytes_keys))]
			filter_keys: Option<Vec<String>>
		) -> Vec<PropertyInfo>
	);
	pass_method!(nft_resources(collection_id: CollectionId, nft_id: NftId) -> Vec<ResourceInfo>);
	pass_method!(nft_resource_priorities(collection_id: CollectionId, nft_id: NftId) -> Vec<ResourceId>);
	pass_method!(base(base_id: BaseId) -> Option<BaseInfo>);
	pass_method!(base_parts(base_id: BaseId) -> Vec<PartType>);
	pass_method!(theme_names(base_id: BaseId) -> Vec<ThemeName>);
	pass_method!(
		theme(
			base_id: BaseId,

			#[map(|n| n.into_bytes())]
			theme_name: String,

			#[map(|keys| keys.map(string_keys_to_bytes_keys))]
			filter_keys: Option<Vec<String>>
		) -> Option<Theme>
	);
}

fn string_keys_to_bytes_keys(keys: Vec<String>) -> Vec<PropertyKey> {
	keys.into_iter().map(|key| key.into_bytes()).collect()
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: RmrkRuntimeApi<
		Block,
		AccountId,
		CollectionInfoOf<RmrkRuntime>,
		InstanceInfoOf<RmrkRuntime>,
		ResourceInfoOf<RmrkRuntime>,
		PropertyInfoOf<RmrkRuntime>,
		BaseInfoOf<RmrkRuntime>,
		PartTypeOf<RmrkRuntime>,
		ThemeOf<RmrkRuntime>,
	>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPaymentApiServer, TransactionPayment};
	use substrate_frame_rpc_system::{SystemApiServer, System};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(SystemRpc::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPaymentRpc::new(client.clone()).into_rpc())?;
	module.merge(RmrkApi::new(client).into_rpc())?;

	Ok(module)
}
