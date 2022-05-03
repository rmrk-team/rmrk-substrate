# NFT Core Pallet Design

## Overview

Essential functionality for nested and multi-resourced NFTs.

Typical use cases are:
* Nested NFTs include anything non-stackably-visual: bundles of NFTs, mortgage NFT with photos of the house, music albums with songs, user-created curated collections for galleries, and more.
* A set of NFTs can be combined as a single object that can be send and sell in an auction as a whole.
* By following some special rules (defined in BASE), some NFTs can be combined in a meaningful way that produce some special effects. E.g. glasses can be equipped to a Kanaria bird and can be rendered as a complete portrait.

Ownership model for nested NFTs ( NFT owning another NFT ) is based on [this](https://github.com/rmrk-team/rmrk-substrate/issues/27) proposal using `pallet-unique` to trace hierarchy of the NFTs and virtual accounts trick. 



![](https://static.swimlanes.io/15201cbf30d5a669d71beee38813e5a5.png)



## Calls

### **create_collection**
Create a collection of NFTs
```rust
    metadata: BoundedVec<u8, T::StringLimit>, // e.g. IPFS hash
    max: Option<u32>, // How many NFTs will ever belong to this collection. 0 for infinite.
    symbol: BoundedVec<u8, T::StringLimit> // Ticker symbol by which to represent the token in wallets and UIs, e.g. ZOMB
```


### **mint_nft** 
Minting an NFT inside a collection
```rust
owner: T::AccountId,
collection_id: CollectionId,
recipient: Option<T::AccountId>, // Receiver of the royalty
royalty: Option<Permill>, // Reward in permills from each trade for the author
metadata: BoundedVec<u8, T::StringLimit> // e.g. IPFS hash
```

### **burn_nft** 
Destroy a NFT
```rust
    collection_id: CollectionId,
    nft_id: NftId
```

### **destroy_collection** 
destroy a collection
```rust
    collection_id: CollectionId
```
### **send** 
Transfers a NFT from an Account or NFT A to another Account or NFT B
```rust
    collection_id: CollectionId,
    nft_id: NftId,
    new_owner: AccountIdOrCollectionNftTuple<T::AccountId>
```

### **accept_nft** 
Accepts an NFT sent from another account to self or owned NFT
```rust
    collection_id: CollectionId,
    nft_id: NftId,
    new_owner: AccountIdOrCollectionNftTuple<T::AccountId>
```

### **reject_nft** 
Rejects an NFT sent from another account to self or owned NFT
```rust
    collection_id: CollectionId,
    nft_id: NftId
```

### **change_collection_issuer** 
changing the issuer of a collection
```rust
    collection_id: CollectionId,
    new_issuer: <T::Lookup as StaticLookup>::Source
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
    
Multi resource calls.


### **add_resource** 
Create a resource
```rust
    collection_id: CollectionId,
    nft_id: NftId,
    resource_id: BoundedResource<T::ResourceSymbolLimit>,
    base: Option<BaseId>,
    src: Option<BoundedVec<u8, T::StringLimit>>,
    metadata: Option<BoundedVec<u8, T::StringLimit>>,
    slot: Option<SlotId>,
    license: Option<BoundedVec<u8, T::StringLimit>>,
    thumb: Option<BoundedVec<u8, T::StringLimit>>,
    parts: Option<Vec<PartId>>
```


### **accept** 
Accept the addition of a new resource to an existing NFT or addition of a child into a parent NFT
```rust
    collection_id: CollectionId,
    nft_id: NftId,
    resource_id: BoundedResource<T::ResourceSymbolLimit>
```


### **set_priority** 
set a different order of resource priority
```rust
    collection_id: CollectionId,
    nft_id: NftId,
    priorities: Vec<Vec<u8>>
```

    
    
## Storages
Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-core/src/lib.rs#L159-L223)

* NextNftId
* CollectionIndex
* NextResourceId
* Collections
* NftsByOwner
* Nfts
* PendingNfts
* Priorities
* Children
* Resources
* Properties

## Events
Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-core/src/lib.rs#L67-L149)
* CollectionCreated
* NftMinted
* NFTBurned
* CollectionDestroyed
* NFTSent
* NFTAccepted
* NFTRejected
* IssuerChanged
* PropertySet
* CollectionLocked
* ResourceAdded
* ResourceAccepted
* PrioritySet

## Traits / Types
Set of re-usable traits describing the total interface located [here](https://github.com/rmrk-team/rmrk-substrate/tree/main/traits/src)

### Primitives
```rust
type CollectionId = u32;
type ResourceId = u32;
type NftId = u32;
```


### **CollectionInfo**
```rust
pub struct CollectionInfo<BoundedString, AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,
	pub metadata: BoundedString,
	pub max: u32,
	pub symbol: BoundedString,
	pub nfts_count: u32,
}
```
    
### NftInfo
```rust
pub struct NftInfo<AccountId, BoundedString> {
	/// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
	pub owner: AccountIdOrCollectionNftTuple<AccountId>,
	/// The user account which receives the royalty
	pub recipient: AccountId,
	/// Royalty in per mille (1/1000)
	pub royalty: Permill,
	/// Arbitrary data about an instance, e.g. IPFS hash
	pub metadata: BoundedString,
	/// Equipped state
	pub equipped: bool,
}
```

### AccountIdOrCollectionNftTuple
```rust
pub enum AccountIdOrCollectionNftTuple<AccountId> {
	AccountId(AccountId),
	CollectionAndNftTuple(CollectionId, NftId),
}
```

### ResourceInfo
```rust
pub struct ResourceInfo<BoundedResource, BoundedString> {
	/// id is a 5-character string of reasonable uniqueness.
	/// The combination of base ID and resource id should be unique across the entire RMRK
	/// ecosystem which
	pub id: BoundedResource,

	/// If resource is sent to non-rootowned NFT, pending will be false and need to be accepted
	pub pending: bool,

	/// If a resource is composed, it will have an array of parts that compose it
	pub parts: Option<Vec<PartId>>,

	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: Option<BaseId>,
	/// If the resource is Media, the base property is absent. Media src should be a URI like an
	/// IPFS hash.
	pub src: Option<BoundedString>,
	pub metadata: Option<BoundedString>,
	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	/// The baseslot will be composed of two dot-delimited values, like so:
	/// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
	/// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird
	pub slot: Option<SlotId>,
	/// The license field, if present, should contain a link to a license (IPFS or static HTTP
	/// url), or an identifier, like RMRK_nocopy or ipfs://ipfs/someHashOfLicense.
	pub license: Option<BoundedString>,
	/// If the resource has the thumb property, this will be a URI to a thumbnail of the given
	/// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
	/// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
	/// another bird, showing the full render of one bird inside the other's inventory might be a
	/// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
	/// image that is lighter and faster to load but representative of this resource.
	pub thumb: Option<BoundedString>,
}
```


### Property
```rust
pub trait Property<KeyLimit, ValueLimit, AccountId> {
	fn property_set(
		sender: AccountId,
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: KeyLimit,
		value: ValueLimit,
	) -> DispatchResult;
}
```