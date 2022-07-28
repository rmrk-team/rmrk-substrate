Essential functionality for nested and multi-resourced NFTs.

Typical use cases are:

- Nested NFTs include anything non-stackably-visual: bundles of NFTs, mortgage NFT with photos of the house, music albums with songs, user-created curated collections for galleries, and more.
- A set of NFTs can be combined as a single object that can be send and sell in an auction as a whole.
- By following some special rules (defined in BASE), some NFTs can be combined in a meaningful way that produce some special effects. E.g. glasses can be equipped to a Kanaria bird and can be rendered as a complete portrait.

Ownership model for nested NFTs ( NFT owning another NFT ) is based on [this](https://github.com/rmrk-team/rmrk-substrate/issues/27) proposal using `pallet-unique` to trace hierarchy of the NFTs and virtual accounts trick.

![](https://static.swimlanes.io/15201cbf30d5a669d71beee38813e5a5.png)
