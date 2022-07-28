### **create_collection**

Create a collection of assets.

```rust
    metadata: BoundedVec<u8, T::StringLimit>, // e.g. IPFS hash
    max: Option<u32>, // How many NFTs will ever belong to this collection. 0 for infinite.
    symbol: BoundedVec<u8, T::StringLimit> // Ticker symbol by which to represent the token in wallets and UIs, e.g. ZOMB
```

### **mint_nft**

Mints an NFT in the specified collection. Sets metadata and the royalty attribute.

```rust
	owner: T::AccountId,
	collection_id: CollectionId, // The collection of the asset to be minted.
	royalty_recipient: Option<T::AccountId>, // Receiver of the royalty
	royalty: Option<Permill>, // Permillage reward from each trade for the Recipient
	metadata: BoundedVec<u8, T::StringLimit> // Arbitrary data about an nft, e.g. IPFS hash
	transferable: bool // Non transferable NFT (aka "Soulbound"),
	resources: Option<BoundedResourceTypeOf<T>> // Add resources during mint
```

### **mint_nft_directly_to_nft**

Mints an NFT in the specified collection directly to another NFT. Sets metadata and the royalty attribute.

```rust
	owner: (CollectionId, NftId), // Owner is a tuple of CollectionId, NftId
	collection_id: CollectionId, // The collection of the asset to be minted.
	royalty_recipient: Option<T::AccountId>, // Receiver of the royalty
	royalty: Option<Permill>, // Permillage reward from each trade for the Recipient
	metadata: BoundedVec<u8, T::StringLimit> // Arbitrary data about an nft, e.g. IPFS hash
	transferable: bool // Non transferable NFT (aka "Soulbound"),
	resources: Option<BoundedResourceTypeOf<T>> // Add resources during mint
```

### **burn_nft**

Burn a NFT

```rust
	collection_id: CollectionId,
	nft_id: NftId,
	max_burns: u32, // Max depth of nested assets recursive burning
```

### **destroy_collection**

destroy a collection

```rust
    collection_id: CollectionId
```

### **send**

Transfers a NFT from an Account or NFT A to another Account or NFT B

```rust
    collection_id: CollectionId, // collection id of the nft to be transferred
    nft_id: NftId, // nft id of the nft to be transferred
    new_owner: AccountIdOrCollectionNftTuple<T::AccountId> // new owner of the nft which can be either an account or a NFT
```

### **accept_nft**

Accepts an NFT sent from another account to self or owned NFT.

```rust
    collection_id: CollectionId, // collection id of the nft to be accepted
    nft_id: NftId, // nft id of the nft to be accepted
    new_owner: AccountIdOrCollectionNftTuple<T::AccountId> // either origin's account ID or origin-owned NFT, whichever the NFT was sent to
```

### **reject_nft**

Rejects an NFT sent from another account to self or owned NFT

```rust
    collection_id: CollectionId, // collection id of the nft to be accepted
    nft_id: NftId // nft id of the nft to be accepted
```

### **change_collection_issuer**

Change the issuer of a collection.

```rust
    collection_id: CollectionId, // collection id of the nft to change issuer of
    new_issuer: <T::Lookup as StaticLookup>::Source // Collection's new issuer
```

### **set_property**

Set a custom value on an NFT

```rust
    collection_id: CollectionId,
    maybe_nft_id: Option<NftId>,
    key: KeyLimitOf<T>,
    value: ValueLimitOf<T>
```

### **lock_collection**

Lock collection

```rust
    collection_id: CollectionId
```

---

### **add_basic_resource**

Create a basic resource. [BasicResource](https://github.com/rmrk-team/rmrk-substrate/blob/3f4f1a7613be81828697347d3e297a460fca5ec5/traits/src/resource.rs#L25)

```rust
	collection_id: CollectionId,
	nft_id: NftId,
	resource: BasicResource<StringLimitOf<T>>,
```

### **add_composable_resource**

Create s composable resource. [ComposableResource](https://github.com/rmrk-team/rmrk-substrate/blob/3f4f1a7613be81828697347d3e297a460fca5ec5/traits/src/resource.rs#L60)

```rust
	collection_id: CollectionId,
	nft_id: NftId,
	resource: ComposableResource<StringLimitOf<T>, BoundedVec<PartId, T::PartsLimit>>,
```

### **add_slot_resource**

Create a slot resource. [SlotResource](https://github.com/rmrk-team/rmrk-substrate/blob/3f4f1a7613be81828697347d3e297a460fca5ec5/traits/src/resource.rs#L107)

```rust
	collection_id: CollectionId,
	nft_id: NftId,
	resource: SlotResource<StringLimitOf<T>>,
```

### **accept_resource**

Accept the addition of a new resource to an existing NFT.

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    resource_id: ResourceId
```

### **remove_resource**

Remove a resource.

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    resource_id: ResourceId
```

### **accept_resource_removal**

Accept the removal of a resource of an existing NFT.

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    resource_id: ResourceId
```

### **set_priority**

set a different order of resource priority

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    priorities: BoundedVec<ResourceId, T::MaxPriorities>,
```
