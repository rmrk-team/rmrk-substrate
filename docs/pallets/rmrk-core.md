## Events

Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-core/src/lib.rs#L67-L149)

- CollectionCreated
- NftMinted
- NFTBurned
- CollectionDestroyed
- NFTSent
- NFTAccepted
- NFTRejected
- IssuerChanged
- PropertySet
- CollectionLocked
- ResourceAdded
- ResourceAccepted
- PrioritySet

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
