import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import {
    createCollection,
    mintNft,
    sendNft,
    rejectNft
} from "./util/tx";
import { getChildren, NftIdTuple } from "./util/fetch";
import { isNftChildOfAnother, expectTxFailure } from "./util/helpers";

describe("Integration test: reject NFT", () => {
    let api: any;
    before(async () => { api = await getApiConnection(); });

    const alice = "//Alice";
    const bob = "//Bob";

    const createTestCollection = async (issuerUri: string) => {
        return await createCollection(
            api,
            issuerUri,
            "reject-metadata",
            null,
            "reject-collection"
        );
    }

    it("reject NFT", async () => {
        const ownerAlice = alice;
        const ownerBob = bob;

        const aliceCollectionId = await createTestCollection(alice);
        const bobCollectionId = await createTestCollection(bob);

        const parentNftId = await mintNft(api, alice, ownerAlice, aliceCollectionId, "parent-nft-metadata");
        const childNftId = await mintNft(api, bob, ownerBob, bobCollectionId, "child-nft-metadata");

        const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

        await sendNft(api, "pending", ownerBob, bobCollectionId, childNftId, newOwnerNFT);
        await rejectNft(api, alice, bobCollectionId, childNftId);

        const isChild = await isNftChildOfAnother(api, bobCollectionId, childNftId, newOwnerNFT);
        expect(isChild, 'Error: rejected NFT is still a child of the target NFT').to.be.false; // FIXME: fails, the NFT is still a child
    });

    it("[negative] unable to reject NFT by a not-an-owner", async () => {
        const ownerAlice = alice;
        const ownerBob = bob;

        const aliceCollectionId = await createTestCollection(alice);
        const bobCollectionId = await createTestCollection(bob);

        const parentNftId = await mintNft(api, alice, ownerAlice, aliceCollectionId, "parent-nft-metadata");
        const childNftId = await mintNft(api, bob, ownerBob, bobCollectionId, "child-nft-metadata");

        const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

        await sendNft(api, "pending", ownerBob, bobCollectionId, childNftId, newOwnerNFT);
        const tx = rejectNft(api, bob, bobCollectionId, childNftId);

        await expectTxFailure(/rmrkCore.CannotRejectNonOwnedNft/, tx);
    });

    it("[negative] unable to reject non-existing NFT", async () => {
        const collectionId = 0;
        const maxNftId = 0xFFFFFFFF;

        const owner = alice;
        const aliceCollectionId = await createTestCollection(alice);

        const parentNftId = await mintNft(api, alice, owner, aliceCollectionId, "parent-nft-metadata");

        const tx = rejectNft(api, alice, collectionId, maxNftId);

        await expectTxFailure(/rmrkCore.NoAvailableNftId/, tx);
    });

    it("[negative] unable to reject NFT which is not sent", async () => {
        const ownerAlice = alice;
        const ownerBob = bob;

        const aliceCollectionId = await createTestCollection(alice);
        const bobCollectionId = await createTestCollection(bob);

        const parentNftId = await mintNft(api, alice, ownerAlice, aliceCollectionId, "parent-nft-metadata");
        const childNftId = await mintNft(api, bob, ownerBob, bobCollectionId, "child-nft-metadata");

        const tx = rejectNft(api, alice, bobCollectionId, childNftId);

        await expectTxFailure(/rmrkCore.CannotRejectNonOwnedNft/, tx);

        const possibleParentNFT: NftIdTuple = [aliceCollectionId, parentNftId];
        const isChild = await isNftChildOfAnother(api, bobCollectionId, childNftId, possibleParentNFT);
        expect(isChild).to.be.false;
    });

    after(() => { api.disconnect(); });
});
