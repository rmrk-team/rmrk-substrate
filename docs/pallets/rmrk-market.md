# Market Pallet Design

Marketplace pallet. Should extend RMRK Core pallet.
## Calls

### **buy**
Buy a listed NFT. Ensure that the NFT is available for purchase and has not recently been purchased, sent, or burned.

```rust 
    collection_id: CollectionId,
    nft_id: NftId
```

### **list**
List a RMRK NFT on the Marketplace for purchase. A listing can be cancelled, and is
automatically considered cancelled when a `buy` is executed on top of a given listing.
An NFT that has another NFT as its owner CANNOT be listed. An NFT owned by a NFT must
first be sent to an account before being listed.

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    amount: BalanceOf<T>,
    expires: Option<T::BlockNumber>
```


### **unlist** 
Unlist a RMRK NFT on the Marketplace and remove from storage in `Listings`.

```rust
    collection_id: CollectionId,
    nft_id: NftId
```

### **make_offer**
Make an offer on a RMRK NFT for purchase. An offer can be set with an expiration where the offer can no longer be accepted by the RMRK NFT owner.

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    amount: BalanceOf<T>,
    expires: Option<T::BlockNumber>
```

### **withdraw_offer**
Withdraw an offer on a RMRK NFT, such that it is no longer available to be accepted by the NFT owner.
```rust
    collection_id: CollectionId,
    nft_id: NftId
```

### **accept_offer**
Accept an offer on a RMRK NFT from a potential buyer.

```rust
    collection_id: CollectionId,
    nft_id: NftId,
    offerer: T::AccountId // Account that made the offer
```

## Storages
Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-market/src/lib.rs#L74-L98)

* ListedNfts
* Offers

## Events
Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-market/src/lib.rs#L102-L151)
* TokenPriceUpdated
* TokenSold
* TokenListed
* TokenUnlisted
* OfferPlaced
* OfferWithdrawn
* OfferAccepted

## Types

### ListInfo
```rust
pub struct ListInfo<AccountId, Balance, BlockNumber> {
    /// Owner who listed the NFT at the time
    pub(super) listed_by: AccountId,
    /// Listed amount
    pub(super) amount: Balance,
    /// After this block the listing can't be bought
    pub(super) expires: Option<BlockNumber>,
}
```

### Offer
```rust
pub struct Offer<AccountId, Balance, BlockNumber> {
    /// User who made the offer
    pub(super) maker: AccountId,
    /// Offered amount
    pub(super) amount: Balance,
    /// After this block the offer can't be accepted
    pub(super) expires: Option<BlockNumber>,
}
```