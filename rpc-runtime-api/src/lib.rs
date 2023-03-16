#![cfg_attr(not(feature = "std"), no_std)]

use rmrk_traits::{
	primitives::{BaseId, CollectionId, NftId, ResourceId},
	NftChild,
};
use sp_api::{Decode, Encode};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub type Result<T> = core::result::Result<T, DispatchError>;

pub type RpcString = Vec<u8>;

pub type PropertyKey = RpcString;

pub type ThemeName = RpcString;

sp_api::decl_runtime_apis! {
	pub trait RmrkApi<
		AccountId,
		CollectionInfo,
		NftInfo,
		ResourceInfo,
		PropertyInfo,
		BaseInfo,
		PartType,
		Theme
	>
	where
		AccountId: Encode,
		CollectionInfo: Decode,
		NftInfo: Decode,
		ResourceInfo: Decode,
		PropertyInfo: Decode,
		BaseInfo: Decode,
		PartType: Decode,
		Theme: Decode,
	{
		/// Get collection by id
		fn collection_by_id(id: CollectionId) -> Result<Option<CollectionInfo>>;

		/// Get NFT by collection id and NFT id
		fn nft_by_id(collection_id: CollectionId, nft_id: NftId) -> Result<Option<NftInfo>>;

		/// Get tokens owned by an account in a collection
		fn account_tokens(account_id: AccountId, collection_id: CollectionId) -> Result<Vec<NftId>>;

		/// Get NFT children
		fn nft_children(collection_id: CollectionId, nft_id: NftId) -> Result<Vec<NftChild<CollectionId, NftId>>>;

		/// Get all of the NFTs of the provided account. Supports pagination by
		/// specifying an optional `start_index` and `count`.
		///
		/// The `start_index` parameter defines the number of collections after
		/// which we start reading the NFTs. The collections in which the user
		/// doesn't own any NFTs are not counted.
		///
		/// The `count` parameter specifies the number of collections to read from.
		fn nfts_owned_by(account_id: AccountId, start_index: Option<u32>, count: Option<u32>) -> Result<Vec<(CollectionId, NftId, NftInfo)>>;

		/// Get all of the properties of the NFTs owned by the specified
		/// account. Supports pagination by specifying an optional `start_index`
		/// and `count`.
		///
		/// The `start_index` parameter defines the number of collections after
		/// which we start reading the NFT properties. The collections in which
		/// the user doesn't own any NFTs are not counted.
		///
		/// The `count` parameter specifies the number of collections to read from.
		fn properties_of_nfts_owned_by(
			account_id: AccountId,
			start_index: Option<u32>,
			count: Option<u32>
		) -> Result<Vec<(CollectionId, NftId, Vec<PropertyInfo>)>>;

		/// Get collection properties
		fn collection_properties(collection_id: CollectionId, filter_keys: Option<Vec<PropertyKey>>) -> Result<Vec<PropertyInfo>>;

		/// Get NFT properties
		fn nft_properties(collection_id: CollectionId, nft_id: NftId, filter_keys: Option<Vec<PropertyKey>>) -> Result<Vec<PropertyInfo>>;

		/// Get NFT resources
		fn nft_resources(collection_id: CollectionId, nft_id: NftId) -> Result<Vec<ResourceInfo>>;

		/// Get NFT resource priority
		fn nft_resource_priority(collection_id: CollectionId, nft_id: NftId, resource_id: ResourceId) -> Result<Option<u32>>;

		/// Get base info
		fn base(base_id: BaseId) -> Result<Option<BaseInfo>>;

		/// Get all Base's parts
		fn base_parts(base_id: BaseId) -> Result<Vec<PartType>>;

		/// Get Base's theme names
		fn theme_names(base_id: BaseId) -> Result<Vec<ThemeName>>;

		/// Get Theme info -- name, properties, and inherit flag
		fn theme(base_id: BaseId, theme_name: ThemeName, filter_keys: Option<Vec<PropertyKey>>) -> Result<Option<Theme>>;
	}
}
