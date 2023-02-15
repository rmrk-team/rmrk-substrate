use std::{marker::PhantomData, sync::Arc};

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use sp_api::{ApiExt, BlockId, BlockT, Decode, Encode, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;

use pallet_rmrk_rpc_runtime_api::{PropertyKey, RmrkApi as RmrkRuntimeApi, ThemeName};
use rmrk_traits::{
	primitives::{BaseId, CollectionId, NftId, ResourceId},
	NftChild,
};

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

			let result = result.map_err(|e| JsonRpseeError::Custom(format!("sp_api error: {:?}", e)))?;

			result.map_err(|e| JsonRpseeError::Custom(format!("runtime error: {:?}", e)))
		}
	};
}

/// RMRK RPC API instance
pub struct Rmrk<Block: BlockT, C> {
	client: Arc<C>,
	_marker: PhantomData<Block>,
}

impl<Block: BlockT, C> Rmrk<Block, C> {
	/// Make new RMRK RPC API instance
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
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
	Theme,
>
{
	#[method(name = "collectionById")]
	/// Get collection by id
	fn collection_by_id(
		&self,
		id: CollectionId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<CollectionInfo>>;

	#[method(name = "nftById")]
	/// Get NFT by collection id and NFT id
	fn nft_by_id(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<NftInfo>>;

	#[method(name = "accountTokens")]
	/// Get tokens owned by an account in a collection
	fn account_tokens(
		&self,
		account_id: AccountId,
		collection_id: CollectionId,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<NftId>>;

	#[method(name = "nftChildren")]
	/// Get NFT children
	fn nft_children(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<NftChild<CollectionId, NftId>>>;

	#[method(name = "nftsOwnedBy")]
	/// Get all of the NFTs of the provided account. Supports pagination by
	/// specifying an optional `start_index` and `count`.
	///
	/// The `start_index` parameter defines the number of collections after
	/// which we start reading the NFTs. The collections in which the user
	/// doesn't own any NFTs are not counted.
	///
	/// The `count` parameter specifies the number of collections to read from.
	fn nfts_owned_by(
		&self,
		account_id: AccountId,
		start_index: Option<u32>,
		count: Option<u32>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<(CollectionId, NftId, NftInfo)>>;

	#[method(name = "propertiesOfNftsOwnedBy")]
	/// Get all of the properties of the NFTs owned by the specified account.
	/// Supports pagination by specifying an optional `start_index` and `count`.
	///
	/// The `start_index` parameter defines the number of collections after which we
	/// start reading the NFT properties. The collections in which the user
	/// doesn't own any NFTs are not counted.
	///
	/// The `count` parameter specifies the number of collections to read from.
	fn properties_of_nfts_owned_by(
		&self,
		account_id: AccountId,
		start_index: Option<u32>,
		count: Option<u32>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<(CollectionId, NftId, Vec<PropertyInfo>)>>;

	#[method(name = "collectionProperties")]
	/// Get collection properties
	fn collection_properties(
		&self,
		collection_id: CollectionId,
		filter_keys: Option<Vec<String>>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<PropertyInfo>>;

	#[method(name = "nftProperties")]
	/// Get NFT properties
	fn nft_properties(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		filter_keys: Option<Vec<String>>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<PropertyInfo>>;

	#[method(name = "nftResources")]
	/// Get NFT resources
	fn nft_resources(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<ResourceInfo>>;

	#[method(name = "nftResourcePriority")]
	/// Get NFT resource priority
	fn nft_resource_priority(
		&self,
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<u32>>;

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
	/// Get Theme info -- name, properties, and inherit flag
	fn theme(
		&self,
		base_id: BaseId,
		theme_name: String,
		filter_keys: Option<Vec<String>>,
		at: Option<BlockHash>,
	) -> RpcResult<Option<Theme>>;
}

#[async_trait]
impl<
		Block,
		C,
		AccountId,
		CollectionInfo,
		NftInfo,
		ResourceInfo,
		PropertyInfo,
		BaseInfo,
		PartType,
		Theme,
	>
	RmrkApiServer<
		<Block as BlockT>::Hash,
		AccountId,
		CollectionInfo,
		NftInfo,
		ResourceInfo,
		PropertyInfo,
		BaseInfo,
		PartType,
		Theme,
	> for Rmrk<Block, C>
where
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: RmrkRuntimeApi<
		Block,
		AccountId,
		CollectionInfo,
		NftInfo,
		ResourceInfo,
		PropertyInfo,
		BaseInfo,
		PartType,
		Theme,
	>,
	AccountId: Encode,
	CollectionInfo: Decode,
	NftInfo: Decode,
	ResourceInfo: Decode,
	PropertyInfo: Decode,
	BaseInfo: Decode,
	PartType: Decode,
	Theme: Decode,
	Block: BlockT,
{
	pass_method!(collection_by_id(id: CollectionId) -> Option<CollectionInfo>);
	pass_method!(nft_by_id(collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo>);
	pass_method!(account_tokens(account_id: AccountId, collection_id: CollectionId) -> Vec<NftId>);
	pass_method!(nft_children(collection_id: CollectionId, nft_id: NftId) -> Vec<NftChild<CollectionId, NftId>>);
	pass_method!(
		nfts_owned_by(
			account_id: AccountId,
			start_index: Option<CollectionId>,
			count: Option<u32>
		) -> Vec<(CollectionId, NftId, NftInfo)>
	);
	pass_method!(
		properties_of_nfts_owned_by(
			account_id: AccountId,
			start_index: Option<CollectionId>,
			count: Option<u32>
		) -> Vec<(CollectionId, NftId, Vec<PropertyInfo>)>
	);
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
	pass_method!(nft_resource_priority(collection_id: CollectionId, nft_id: NftId, resource_id: ResourceId) -> Option<u32>);
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
