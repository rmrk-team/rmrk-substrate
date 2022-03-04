# Market Pallet Design

## Table of Contents

[TOC]

## Calls
> The external dispatchable calls. i.e. The methods user can invoke by sending a
transaction

Marketplace pallet. Should extend [**NFT Core**](https://hackmd.io/GNJXyhXnTJiXvg3X-r3l3Q)

* **buy**
    * buy NFT
* **list/unlist**
    * list NFT for sale
* **make_offer**(origin, collection_id, nft_id, amount: BalanceOf<T>, expires: T::BlockNumber) 
* **withdraw_offer**(origin, collection_id, nft_id)
    
## Storages
> Defines how to access on-chain storage  




## Types
    
    
## Events
> Defines the events that could be emitted by this pallet to indicate what happened

Every call should implement relevant Event. ie Auctions pallet events:
```#rust
pub enum Event<T: Config> {
    /// The price for a token was updated \[owner, collection_id, nft_id, price\]
    TokenPriceUpdated(T::AccountId, T::NftId, T::NftId, Option<BalanceOf<T>>),
    /// Token was sold to a new owner \[owner, buyer, collection_id, nft_id, price, author, royalty, royalty_amount\]
    TokenSold(
        T::AccountId,
        T::AccountId,
        T::CollectionId,
        T::NftId,
        BalanceOf<T>,
        Option<(T::AccountId, u8)>,
        BalanceOf<T>,
    ),
    /// Token listed on Marketplace \[owner, collection_id, nft_id, author royalty\]
    TokenListed(T::AccountId, T::CollectionId, T::NftId, T::AccountId, u8),
    /// Token listed on Marketplace \[collection_id, nft_id\]
    TokenUnlisted(T::CollectionId, T::NftId),
    /// Offer was placed on a token \[offerer, collection_id, nft_id, price\]
    OfferPlaced(T::AccountId, T::CollectionId, T::NftId, BalanceOf<T>),
    /// Offer was withdrawn \[sender, collection_id, nft_id\]
    OfferWithdrawn(T::AccountId, T::CollectionId, T::NftId),
    /// Offer was accepted \[sender, collection_id, nft_id\]
    OfferAccepted(T::AccountId, T::CollectionId, T::NftId),
}
```