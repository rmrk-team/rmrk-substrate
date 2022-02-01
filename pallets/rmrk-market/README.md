# Market Pallet
Market pallet design for the RMRK NFT Market. The Market pallet should extend [NFT Core](https://hackmd.io/GNJXyhXnTJiXvg3X-r3l3Q).

## Calls
- `buy(origin, collection_id, nft_id)`
- `list(origin, collection_id, nft_id, amount: T::Balance, expires: Option<T::BlockNumber>)`
- `unlist(origin, collection_id, nft_id)`
- `make_offer(origin, collection_id, nft_id, amount: BalanceOf<T>, expires: Option<T::BlockNumber>)`
- `withdraw_offer(origin, collection_id, nft_id)`

## Storages
```rust
#[pallet::storage]
#[pallet::getter(fn listed_nfts)]
/// Stores listed NFT price info
pub type ListedNfts<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    CollectionId,
    Blake2_128Concat,
    NftId,
    ListInfoOf<T>,
    OptionQuery,
>;

#[pallet::storage]
#[pallet::getter(fn offers)]
/// Stores offer on a NFT info
pub type Offers<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    (CollectionId, NftId),
    Blake2_128Concat,
    T::AccountId,
    OfferOf<T>,
    OptionQuery,
>;
```

## Types
```rust
pub type ListInfoOf<T> = ListInfo<
    <T as frame_system::Config>::AccountId, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

pub type OfferOf<T> = Offer<
    <T as frame_system::Config>::AccountId, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;
```

## Events

```rust
pub enum Event<T: Config> {
    /// The price for a token was updated \[owner, collection_id, nft_id, price\]
    TokenPriceUpdated {
        owner: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
        price: Option<BalanceOf<T>>,
    },
    /// Token was sold to a new owner
    /// \[owner, buyer, collection_id, nft_id, price, author\]
    TokenSold {
        owner: T::AccountId,
        buyer: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
        price: BalanceOf<T>,
    },
    /// Token listed on Marketplace \[owner, collection_id, nft_id\]
    TokenListed {
        owner: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
        price: BalanceOf<T>,
    },
    /// Token unlisted on Marketplace \[owner, collection_id, nft_id\]
    TokenUnlisted {
        owner: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
    },
    /// Offer was placed on a token \[offerer, collection_id, nft_id, price\]
    OfferPlaced {
        offerer: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
        price: BalanceOf<T>,
    },
    /// Offer was withdrawn \[sender, collection_id, nft_id\]
    OfferWithdrawn {
        sender: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
    },
    /// Offer was accepted \[owner, buyer, collection_id, nft_id\]
    OfferAccepted {
        owner: T::AccountId,
        buyer: T::AccountId,
        collection_id: CollectionId,
        nft_id: NftId,
    },
}
```

## RMRK Spec
- [LIST](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/list.md)
- [BUY](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/buy.md)