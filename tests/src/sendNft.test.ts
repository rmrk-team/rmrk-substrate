import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { createCollection, mintNft, sendNft } from "./util/tx";
import { NftIdTuple } from "./util/fetch";
import { isNftChildOfAnother, expectTxFailure } from "./util/helpers";

describe("integration test: send NFT", () => {
  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  const maxNftId = 0xffffffff;

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
      "nft-collection-metadata",
      null,
      "nft-collection"
    );
  };

  it("send NFT to another user", async () => {
    const originalOwnerUri = alice;
    const newOwnerUri = bob;

    const collectionId = await createTestCollection(alice, 140);

    const nftId = await mintNft(
      api,
      300,
      alice,
      originalOwnerUri,
      collectionId,
      "nft-metadata"
    );

    await sendNft(
      api,
      "sent",
      originalOwnerUri,
      collectionId,
      nftId,
      newOwnerUri
    );
  });

  it("[negative] unable to send non-existing NFT", async () => {
    const originalOwnerUri = alice;
    const newOwnerUri = bob;

    const collectionId = 0;
    const tx = sendNft(
      api,
      "sent",
      originalOwnerUri,
      collectionId,
      maxNftId,
      newOwnerUri
    );

    await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  });

  it("[negative] unable to send NFT by a not-an-owner", async () => {
    const originalOwnerUri = alice;
    const newOwnerUri = bob;

    const collectionId = await createTestCollection(alice, 141);

    const nftId = await mintNft(
      api,
      310,
      alice,
      originalOwnerUri,
      collectionId,
      "nft-metadata"
    );

    const tx = sendNft(
      api,
      "sent",
      newOwnerUri,
      collectionId,
      nftId,
      newOwnerUri
    );
    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("send NFT to another NFT (same owner)", async () => {
    const originalOwnerUri = alice;

    const collectionId = await createTestCollection(alice, 142);

    const parentNftId = await mintNft(
      api,
      320,
      alice,
      originalOwnerUri,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      1,
      alice,
      originalOwnerUri,
      collectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionId, parentNftId];

    await sendNft(api, "sent", alice, collectionId, childNftId, newOwnerNFT);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.true;
  });

  it("[negative] send non-existing NFT to another NFT", async () => {
    const originalOwnerUri = alice;

    const collectionId = await createTestCollection(alice, 143);

    const parentNftId = await mintNft(
      api,
      330,
      alice,
      originalOwnerUri,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = maxNftId;

    const newOwnerNFT: NftIdTuple = [collectionId, parentNftId];

    const tx = sendNft(
      api,
      "sent",
      alice,
      collectionId,
      childNftId,
      newOwnerNFT
    );

    await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.false;
  });

  it("send NFT to another NFT (by not-an-owner)", async () => {
    const originalOwnerUri = alice;

    const collectionId = await createTestCollection(alice, 144);

    const author = alice;
    const attacker = bob;

    const parentNftId = await mintNft(
      api,
      340,
      author,
      originalOwnerUri,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      341,
      author,
      originalOwnerUri,
      collectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionId, parentNftId];

    const tx = sendNft(
      api,
      "sent",
      attacker,
      collectionId,
      childNftId,
      newOwnerNFT
    );

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.false;
  });

  it("[negative] send NFT to non-existing NFT", async () => {
    const originalOwnerUri = alice;

    const collectionId = await createTestCollection(alice, 145);

    const parentNftId = maxNftId;
    const childNftId = await mintNft(
      api,
      350,
      alice,
      originalOwnerUri,
      collectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionId, parentNftId];

    const tx = sendNft(
      api,
      "sent",
      alice,
      collectionId,
      childNftId,
      newOwnerNFT
    );

    await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.false;
  });

  it("send NFT to another NFT owned by another user", async () => {
    const ownerAlice = alice;
    const ownerBob = bob;

    const aliceCollectionId = await createTestCollection(alice, 146);
    const bobCollectionId = await createTestCollection(bob, 147);

    const parentNftId = await mintNft(
      api,
      360,
      alice,
      ownerAlice,
      aliceCollectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      361,
      bob,
      ownerBob,
      bobCollectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [aliceCollectionId, parentNftId];

    await sendNft(
      api,
      "pending",
      bob,
      bobCollectionId,
      childNftId,
      newOwnerNFT
    );
  });

  it("[negative] unable to send NFT to itself", async () => {
    const nftOwner = alice;
    const collectionId = await createTestCollection(alice, 148);

    const nftId = await mintNft(
      api,
      370,
      alice,
      nftOwner,
      collectionId,
      "ouroboros-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionId, nftId];

    const tx = sendNft(api, "sent", alice, collectionId, nftId, newOwnerNFT);

    await expectTxFailure(/rmrkCore\.CannotSendToDescendentOrSelf/, tx);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      nftId,
      newOwnerNFT
    );
    expect(isChild).to.be.false;
  });

  it("[negative] unable to send NFT to child NFT", async () => {
    const originalOwnerUri = alice;

    const collectionId = await createTestCollection(alice, 149);

    const parentNftId = await mintNft(
      api,
      380,
      alice,
      originalOwnerUri,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      381,
      alice,
      originalOwnerUri,
      collectionId,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionId, parentNftId];

    await sendNft(api, "sent", alice, collectionId, childNftId, newOwnerNFT);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      childNftId,
      newOwnerNFT
    );
    expect(isChild).to.be.true;

    const descendentOwner: NftIdTuple = [collectionId, childNftId];
    const tx = sendNft(
      api,
      "sent",
      alice,
      collectionId,
      parentNftId,
      descendentOwner
    );

    await expectTxFailure(/rmrkCore\.CannotSendToDescendentOrSelf/, tx);
    const isOuroboros = await isNftChildOfAnother(
      api,
      collectionId,
      parentNftId,
      descendentOwner
    );
    expect(isOuroboros).to.be.false;
  });

  it("[negative] unable to send NFT to descendent NFT", async () => {
    const originalOwnerUri = alice;

    const collectionId = await createTestCollection(alice, 150);

    const parentNftId = await mintNft(
      api,
      390,
      alice,
      originalOwnerUri,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      391,
      alice,
      originalOwnerUri,
      collectionId,
      "child-nft-metadata"
    );
    const grandsonNftId = await mintNft(
      api,
      392,
      alice,
      originalOwnerUri,
      collectionId,
      "grandson-nft-metadata"
    );

    const ownerParentNFT: NftIdTuple = [collectionId, parentNftId];

    await sendNft(api, "sent", alice, collectionId, childNftId, ownerParentNFT);

    const isChild = await isNftChildOfAnother(
      api,
      collectionId,
      childNftId,
      ownerParentNFT
    );
    expect(isChild).to.be.true;

    const ownerChildNFT: NftIdTuple = [collectionId, childNftId];
    await sendNft(
      api,
      "sent",
      alice,
      collectionId,
      grandsonNftId,
      ownerChildNFT
    );

    const isGrandson = await isNftChildOfAnother(
      api,
      collectionId,
      grandsonNftId,
      ownerChildNFT
    );
    expect(isGrandson).to.be.true;

    const ownerGrandsonNFT: NftIdTuple = [collectionId, grandsonNftId];
    const tx = sendNft(
      api,
      "sent",
      alice,
      collectionId,
      parentNftId,
      ownerGrandsonNFT
    );

    await expectTxFailure(/rmrkCore\.CannotSendToDescendentOrSelf/, tx);
    const isOuroboros = await isNftChildOfAnother(
      api,
      collectionId,
      parentNftId,
      ownerGrandsonNFT
    );
    expect(isOuroboros).to.be.false;
  });

  it("send nested NFT to another user", async () => {
    const originalOwner = alice;
    const newOwner = bob;

    const collectionId = await createTestCollection(alice, 151);

    const parentNftId = await mintNft(
      api,
      395,
      alice,
      originalOwner,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      396,
      alice,
      originalOwner,
      collectionId,
      "child-nft-metadata"
    );

    const parentNftTuple: NftIdTuple = [collectionId, parentNftId];

    await sendNft(
      api,
      "sent",
      originalOwner,
      collectionId,
      childNftId,
      parentNftTuple
    );

    await sendNft(
      api,
      "sent",
      originalOwner,
      collectionId,
      childNftId,
      newOwner
    );
  });

  it("[negative] send nested NFT to another user (by a not-root-owner)", async () => {
    const originalOwner = alice;
    const newOwner = bob;

    const collectionId = await createTestCollection(alice, 152);

    const parentNftId = await mintNft(
      api,
      397,
      alice,
      originalOwner,
      collectionId,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      398,
      alice,
      originalOwner,
      collectionId,
      "child-nft-metadata"
    );

    const parentNftTuple: NftIdTuple = [collectionId, parentNftId];

    await sendNft(
      api,
      "sent",
      originalOwner,
      collectionId,
      childNftId,
      parentNftTuple
    );

    const tx = sendNft(
      api,
      "sent",
      newOwner,
      collectionId,
      childNftId,
      newOwner
    );

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  after(() => {
    api.disconnect();
  });
});
