# RMRK RPC

> NOTE: All of the RMRK types can be found at `rmrk-substrate/traits/src`

- `CollectionInfo`: `traits/src/collection.rs`
- `NftInfo`: `traits/src/nft.rs`
- `ResourceInfo`: `traits/src/resource.rs`
- `BaseInfo`: `traits/src/base.rs`
- `PartType`: `traits/src/part.rs`
- `Theme`: `traits/src/theme.rs`
- `PropertyInfo`: `traits/src/property.rs`
- `Bytes == Vec<u8>`

### Get last collection index (get collections count)

The frontend can fetch and show the overall collection's count

```rust
lastCollectionIdx() -> CollectionId
```

### Get collection by id

The frontend can fetch and show the collection info

```rust
collectionById(collectionId: CollectionId) -> Option<CollectionInfo>
```

### Get owned NFTs within a collection

The frontend can fetch all NFTs within a collection owned by a specific user

```rust
accountTokens(accountId: AccountId, collectionId: CollectionId) -> Vec<NftId>
```

### Get NFT info by id

The frontent can fetch and show NFT info

```rust
nftById(collectionId: CollectionId, nftId: NftId) -> Option<NftInfo>
```

### Get all of the NFTs owned by user

The frontend can fetch several NFTs at once. Pagination is supported.

```rust
fn nfts_owned_by(account_id: AccountId, start_index: Option<u32>, count: Option<u32>) -> Result<Vec<(CollectionId, NftId, NftInfo)>>;
```

### Get the properties of all of the NFTs owned by user

The frontend can fetch several properties of multiple NFTs at once. Pagination is supported.

```rust
fn properties_of_nfts_owned_by(
	account_id: AccountId,
	start_index: Option<u32>,
	count: Option<u32>
) -> Result<Vec<(CollectionId, NftId, Vec<PropertyInfo>)>>;
```

### Get property keys' values

The frontend can fetch several properties at once

```rust
collectionProperties(collectionId: CollectionId, filterKeys: Option<Vec<u32>>) -> Vec<PropertyInfo>

nftProperties(collectionId: CollectionId, nftId: NftId, filterKeys: Option<Vec<u32>>) -> Vec<PropertyInfo>
```

### Get NFT children

The frotnend can fetch chlidren of an NFT

```rust
nftChildren(collectionId: CollectionId, nftId: NftId) -> Vec<NftChild>
```

### Get NFT Resources

The frontend can fetch NFT resources

```rust
nftResources(collectionId: CollectionId, nftId: NftId) -> Vec<ResourceInfo>
```

### Get NFT Resource Priority

The frontend can fetch NFT resource priorities

```rust
nftResourcePriority(collectionId: CollectionId, nftId: NftId, resourceId: ResourceId) -> Option<u32> /* resource priority */
```

### Get NFT Base

The frotnend can fetch the NFT Base info

```rust
base(baseId: BaseId) -> Option<BaseInfo>
```

### Get Base parts

The frontend can fetch all Base's parts

```rust
baseParts(baseId: BaseId) -> Vec<PartType>
```

### Get Base Theme names

The frontend can fetch all Base's theme names

```rust
themeNames(baseId: BaseId) -> Vec<Bytes>
```

### Get Base Theme

The frontend can fetch Base's Theme info -- name, properties, and inherit flag

```rust
theme(baseId: BaseId, themeName: Bytes, filterKeys: Option<Vec<Bytes>>) -> Option<Theme>
```
