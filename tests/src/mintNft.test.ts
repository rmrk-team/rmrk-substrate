import { getApiConnection } from './substrate/substrate-api';
import { mintNft, createCollection } from './util/tx';

describe("Integration test: mint new NFT", () => {
    let api: any;
    before(async () => { api = await getApiConnection(); });

    const alice = '//Alice';
    const bob = '//Bob';

    it("mint NFT", async () => {
        const owner = alice;
        const collectionMetadata = 'mintingCollectionMetadata';
        const collectionMax = null;
        const collectionSymbol = 'mintingCollectionSymbol';
        const recipientUri = null;
        const royalty = null;
        const nftMetadata = 'NFT-test-metadata';

        let collectionId = await createCollection(
            api,
            alice,
            collectionMetadata,
            collectionMax,
            collectionSymbol
        );

        await mintNft(
            api,
            alice,
            owner,
            collectionId,
            nftMetadata,
            recipientUri,
            royalty
        );
    });

    it("mint NFT and set another owner", async () => {
        const owner = bob;
        const collectionMetadata = 'setOwnerCollectionMetadata';
        const collectionMax = null;
        const collectionSymbol = 'setOwnerCollectionSymbol';
        const recipientUri = null;
        const royalty = null;
        const nftMetadata = 'setOwner-NFT-metadata';

        let collectionId = await createCollection(
            api,
            alice,
            collectionMetadata,
            collectionMax,
            collectionSymbol
        );

        await mintNft(
            api,
            alice,
            owner,
            collectionId,
            nftMetadata,
            recipientUri,
            royalty
        );
    });

    it("mint NFT with recipient and roalty", async () => {
        const owner = alice;
        const collectionMetadata = 'mintingCollectionMetadata';
        const collectionMax = null;
        const collectionSymbol = 'mintingCollectionSymbol';
        const recipientUri = bob;
        const royalty = 70000;
        const nftMetadata = 'recipient-royalty-NFT-test-metadata';

        let collectionId = await createCollection(
            api,
            alice,
            collectionMetadata,
            collectionMax,
            collectionSymbol
        );

        await mintNft(
            api,
            alice,
            owner,
            collectionId,
            nftMetadata,
            recipientUri,
            royalty
        );
    });

    after(() => { api.disconnect(); });
});
