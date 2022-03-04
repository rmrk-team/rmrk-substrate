# Equip NFT Pallet Design

## Table of Contents

[TOC]


## Diagram

![](https://static.swimlanes.io/9b72cd84a9ca752cbb7f2c6348d6a50b.png)
[Full Screen Diagram](https://swimlanes.io/u/vXpX_IxFd)

> Nested NFTs include anything non-stackably-visual too: bundles of NFTs, mortgage NFT with photos of the house, music albums with songs, user-created curated collections for galleries, and more. None of these need equip. Equip, on the other hand, has many implications - per-slot limits, nested rendering across multiple depths (configurable cap on configurable bird), unequip on sale, burn, etc. Equip "addon lego" not useful by itself, it needs the core.

Should extend NFT Core. [Original RMRK2 spec](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)



## Calls



* **equip**
    * equip a child NFT into a parent's slot, or unequip
* **equippable**
    * changes the list of equippable collections on a base's part    
* **theme_add**(base_id)
    * add a new theme to a base
* **create_base**(origin, symbol, type, parts) [**TBD**](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/entities/base.md)
    * create a base. catalogue of parts. It is not an NFT
    
## Storages
> Defines how to access on-chain storage

### Base
    
```#rust
type Base<T: Config> = StorageMap<
    _,
    Twox64Concat,
    T::BaseId,
    BaseDetails<T::AccountId>,
>;
```

### Part
    

StorageDoubleMap structure `BaseId -> PartId -> PartDetail`
```#rust
type Base<T: Config> = StorageDoubleMap<
    _,
    Twox64Concat,
    T::BaseId,
    Twox64Concat,
    T::PartId,    
    PartDetail,
>;
```

    
## Events
> Defines the events that could be emitted by this pallet to indicate what happened

Every call should implement relevant Event. ie Auctions pallet events:
```
pub enum Event<T: Config> {
    BaseCreated(T::AccountId, T::BaseId),
    ThemeAdded(T::BaseId),
    ResourceAdded(T::BaseId, T::ResourceId),
    ResourceAccepted(T::BaseId, T::ResourceId),
    PriorityUpdated(T::ResourceId)
}
```

## Types

* **BaseDetails**
```#rust
pub struct BaseDetails<AccountId> {
    issuer: AccountId,
    symbol: BoundedVec<u8, StringLimit>,
    type: BoundedVec<u8, StringLimit>,
}
```

* **PartDetail**
```#rust
pub struct PartDetail<AccountId> {
    type: BoundedVec<u8, StringLimit>,
    src: BoundedVec<u8, StringLimit>
}
```