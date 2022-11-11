import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { createCollection, mintNft, sendNft, acceptNft } from "./util/tx";
import { NftIdTuple } from "./util/fetch";
import { isNftChildOfAnother, expectTxFailure } from "./util/helpers";

describe("integration test: accept NFT", () => {
  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  const alice = "//Alice";
  const bob = "//Bob";

  const createTestCollection = async (
    issuerUri: string,
    collectionId: number
  ) => {
    return await createCollection(
      api,
      collectionId,
      issuerUri,
      "accept-metadata",
      null,
      "acpt"
    );
  };

  it("accept NFT", async () => {
    const ownerAlice = alice;
    const ownerBob = bob;

    const aliceCollectionId = await createTestCollection(alice, 0);
    const bobCollectionId = await createTestCollection(bob, 1);

    const parentNftId = await mintNft(
      api,
      0,
      alice,
      ownerAlice,
      aliceCollectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      0,
      bob,
      ownerBob,
      bobCollectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

    await sendNft(
      api,
      "pending",
      ownerBob,
      bobCollectionId,
      childNftId,
      newOwnerNFT
    );
    await acceptNft(api, alice, bobCollectionId, childNftId, newOwnerNFT);

    const isChild = await isNftChildOfAnother(
      api,
      bobCollectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.true;
  });

  it("[negative] unable to accept NFT by a not-an-owner", async () => {
    const ownerAlice = alice;
    const ownerBob = bob;

    const aliceCollectionId = await createTestCollection(alice, 2);
    const bobCollectionId = await createTestCollection(bob, 3);

    const parentNftId = await mintNft(
      api,
      0,
      alice,
      ownerAlice,
      aliceCollectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      0,
      bob,
      ownerBob,
      bobCollectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

    await sendNft(
      api,
      "pending",
      ownerBob,
      bobCollectionId,
      childNftId,
      newOwnerNFT
    );
    const tx = acceptNft(api, bob, bobCollectionId, childNftId, newOwnerNFT);

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("[negative] unable to accept non-existing NFT", async () => {
    const collectionId = 0;
    const maxNftId = 0xffffffff;

    const owner = alice;
    const aliceCollectionId = await createTestCollection(alice, 4);

    const parentNftId = await mintNft(
      api,
      0,
      alice,
      owner,
      aliceCollectionId,
      "parent-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

    const tx = acceptNft(api, alice, collectionId, maxNftId, newOwnerNFT);

    await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  });

  it("[negative] unable to accept NFT which is not sent", async () => {
    const ownerAlice = alice;
    const ownerBob = bob;

    const aliceCollectionId = await createTestCollection(alice, 5);
    const bobCollectionId = await createTestCollection(bob, 6);

    const parentNftId = await mintNft(
      api,
      0,
      alice,
      ownerAlice,
      aliceCollectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      0,
      bob,
      ownerBob,
      bobCollectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

    const tx = acceptNft(api, alice, bobCollectionId, childNftId, newOwnerNFT);

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);

    const isChild = await isNftChildOfAnother(
      api,
      bobCollectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.false;
  });

  it("[negative] accept NFT", async () => {
    const ownerAlice = alice;
    const ownerBob = bob;

    const aliceCollectionId = await createTestCollection(alice, 7);
    const bobCollectionId = await createTestCollection(bob, 8);

    const parentNftId = await mintNft(
      api,
      0,
      alice,
      ownerAlice,
      aliceCollectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      0,
      bob,
      ownerBob,
      bobCollectionId,
      "child-nft-metadata"
    );

    const parentNftId2 = await mintNft(
      api,
      1,
      alice,
      ownerAlice,
      aliceCollectionId,
      "parent-nft-metadata2"
    );

    const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];
    const notNewOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId2];

    await sendNft(
      api,
      "pending",
      ownerBob,
      bobCollectionId,
      childNftId,
      newOwnerNFT
    );
    const tx = acceptNft(
      api,
      alice,
      bobCollectionId,
      childNftId,
      notNewOwnerNFT
    );

    await expectTxFailure(/rmrkCore\.CannotAcceptToNewOwner/, tx);

    const isChild = await isNftChildOfAnother(
      api,
      bobCollectionId,
      childNftId,
      notNewOwnerNFT
    );
    expect(isChild).to.be.false;
  });

  after(() => {
    api.disconnect();
  });
});
