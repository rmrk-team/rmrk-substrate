# RMRK Core

![](https://static.swimlanes.io/15201cbf30d5a669d71beee38813e5a5.png)

Extends uniques pallet. Based on [RMRK2 Spec](https://github.com/rmrk-team/rmrk-spec/tree/master/standards/rmrk2.0.0)

* **create_collection**(origin, metadata: Vec<u8>, max: u16)
    * create a collection
* **mint_nft**(origin, collection_id, author: T::AccountId, royalty: u8, metadata: Vec<u8>)
    * minting NFT inside a collection
    * `author`: Receiver of the royalty
    * `royalty`: Percentage reward from each trade for the author
    * create a base. catalogue of parts. It is not an NFT
* **burn_nft**(origin, collection_id, nft_id)
    * burn a NFT
* **burn_collection**(origin, collection_id)
    * burn a Collection
* **send**(origin, collection_id, nft_id, dest)
    * transfer NFT from account A to account B
* **change_issuer**(origin, collection_id, base_id, dest)
    * changing the issuer of a collection or base  
* **set_property**(collection_id, maybe_nft_id, key, value)
    * key and value of type `BoundedVec<u8, T::KeyLimit>`
    * set a custom value on an NFT. Similar to uniques `set_attribute`
* **lock_collection**(collection_id)
    * locking a collection  
* **destroy_collection**(collection_id)
    * destroying a collection

Multi resource calls.


* **add_resource**(nft_id, resource_id)
    * add a new resource to an NFT as the collection issuer
* **accept**(nft_id, resource_id)
    * accept the addition of a new resource to an existing NFT or  addition of a child into a parent NFT.
* **set_priority**(collection_id, nft_id)
    * set a different order of resource priority



## Storages
> Defines how to access on-chain storage

### Collection

```#rust
type Collection<T: Config> = StorageMap<
    _,
    Twox64Concat,
    T::CollectionId,
    CollectionDetails<T::AccountId>,
>;
```

### NFT


StorageDoubleMap structure `CollectionId -> NftId -> NFTDetails`
```#rust
type NFT<T: Config> = StorageDoubleMap<
    _,
    Twox64Concat,
    T::CollectionId,
    Twox64Concat,
    T::NftId,
    NFTDetails<T::AccountId>,
>;
```

### Attribute


1. Uniques based StorageNMap structure `CollectionId -> Some(NftId) -> Key -> Value`
```#rust
type Attribute<T: Config<I>, I: 'static = ()> = StorageNMap<
    _,
    (
        NMapKey<Blake2_128Concat, T::ClassId>,
        NMapKey<Blake2_128Concat, Option<T::InstanceId>>,
        NMapKey<Blake2_128Concat, BoundedVec<u8, T::KeyLimit>>,
    ),
    (BoundedVec<u8, T::ValueLimit>, DepositBalanceOf<T, I>),
    OptionQuery,
>;
```

### Resource


StorageDoubleMap structure `BaseId -> PartId -> PartDetail`
```#rust
type Base<T: Config> = StorageDoubleMap<
    _,
    Twox64Concat,
    T::ResourceId,
    Twox64Concat,
    T::PartId,  
    ResourceDetail,
>;
```


## Events
> Defines the events that could be emitted by this pallet to indicate what happened

Every call should implement relevant Event. ie Auctions pallet events:
```
pub enum Event<T: Config> {
    CollectionMinted(T::AccountId, T::CollectionId),
    NFTMinted(T::AccountId, T::CollectionId, T::NftId),
    CollectionBurned(T::AccountId, T::CollectionId),
    NFTBurned(T::AccountId, T::CollectionId, T::NftId),
    NFTSent(T::AccountId, T::CollectionId, T::NftId),
    CollectionIssuerChanged(T::AccountId, T::AccountId, T::CollectionId),
    PropertySet(T::AccountId, T::CollectionId, T::NftId),
    CollectionLocked(T::AccountId, T::CollectionId),
}
```

## Types

### **CollectionDetails**
```#rust
pub struct CollectionDetails<AccountId> {
    /// Can mint tokens.
    issuer: AccountId,
    max: u16,
    symbol: BoundedVec<u8, StringLimit>,
    metadata: BoundedVec<u8, StringLimit>
}
```

### NFTDetails (WIP)
```#rust
pub struct NFTDetails<AccountId> {
    owner: Option<NftId>,
    sn: u16,
    root_owner: AccountId,
    symbol: BoundedVec<u8, StringLimit>,
    metadata: BoundedVec<u8, StringLimit>
    children: Option<Vec<NFTDetails>>, // nesting references. To discuss
    parent: Option<Vec<NFTDetails>>,
}
```

License: Apache-2.0
