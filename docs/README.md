# RMRK Substrate Pallets

## Main features

- NFTs owning NFTs
- NFTs having multiple priority-ordered client-dependent resources (a book with a cover and an audio
  version)
- NFTs with conditional rendering (on-NFT logic allowing different visuals and resources to trigger
  depending on on-chain and off-chain conditions).
- NFTs governed as DAOs via shareholder tokens (issue commands to NFTs democratically)
- NFTs, accounts, and other on-chain entities being emoted to (on-chain emotes), allowing early
  price discovery without listing, selling, bidding.

## Overview

Initial implementation extends [Substrate Uniques](https://github.com/paritytech/substrate/tree/master/frame/uniques) pallet as 'low level' NFT dependency. 
Mechanics and interactions are based on [RMRK 2 standard](https://github.com/rmrk-team/rmrk-spec/tree/master/standards/rmrk2.0.0)

![](https://camo.githubusercontent.com/1202d3852b7eba4ae73a6e90021e2006984e349f392665c34897fda846fe5b57/68747470733a2f2f7374617469632e7377696d6c616e65732e696f2f36383731663161343233386533663637363265623738343132663062383363322e706e67)

## Pallets 
* [Core](/pallets/rmrk-core)
* [Equip](/pallets/rmrk-equip)
* [Market](/pallets/rmrk-market)
* Auctions (coming soon)
* Emotes (coming soon)
* Fractionalization (coming later)