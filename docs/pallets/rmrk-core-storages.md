**Events** implementation [rmrk-core/src/lib.rs#L67-L149](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-core/src/lib.rs#L67-L149)

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

---

**Storages** implementation [rmrk-core/src/lib.rs#L159-L223](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-core/src/lib.rs#L159-L223)

### NextNftId

Get next NFT id

```rust
	pub type NextNftId<T: Config> = StorageMap<_, Twox64Concat, CollectionId, NftId, ValueQuery>;
```

### CollectionIndex

Get next Collection index

```rust
	pub type CollectionIndex<T: Config> = StorageValue<_, CollectionId, ValueQuery>;
```

### NextResourceId

Get next Resource id

```rust
	pub type NextResourceId<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CollectionId,
		Twox64Concat,
		NftId,
		ResourceId,
		ValueQuery,
	>;
```

### Collections

Stores collections info

```rust
	pub type Collections<T: Config> = StorageMap<
		_,
		Twox64Concat,
		CollectionId,
		CollectionInfo<StringLimitOf<T>, BoundedCollectionSymbolOf<T>, T::AccountId>,
	>;
```

### Nfts

Stores nft info

```rust
	pub type Nfts<T: Config> =
		StorageDoubleMap<_, Twox64Concat, CollectionId, Twox64Concat, NftId, InstanceInfoOf<T>>;
```

### Children

Stores nft children nesting info. Allows NFT to own other NFT

```rust
	pub type Children<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		(CollectionId, NftId),
		Twox64Concat,
		(CollectionId, NftId),
		(),
	>;
```

### Resources

Stores resource info. Allows NFTs to contain multiple resources.

```rust
	pub type Resources<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, ResourceId>,
		),
		ResourceInfoOf<T>,
		OptionQuery,
	>;
```

### EquippableBases

Stores the existence of a base for a particular NFT. This is populated on `add_composable_resource`, and is used in the rmrk-equip pallet when equipping a resource.

```rust
	pub type EquippableBases<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, BaseId>,
		),
		(),
	>;
```

### EquippableSlots

Stores the existence of a Base + Slot for a particular NFT's particular resource. This is populated on `add_slot_resource`, and is used in the rmrk-equip pallet when equipping a resource.

```rust
	pub type EquippableSlots<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, NftId>,
			NMapKey<Blake2_128Concat, ResourceId>,
			NMapKey<Blake2_128Concat, BaseId>,
			NMapKey<Blake2_128Concat, SlotId>,
		),
		(),
	>;
```

### Properties

Arbitrary properties / metadata of an asset.

```rust
	pub(super) type Properties<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CollectionId>,
			NMapKey<Blake2_128Concat, Option<NftId>>,
			NMapKey<Blake2_128Concat, KeyLimitOf<T>>,
		),
		ValueLimitOf<T>,
		OptionQuery,
	>;
```
