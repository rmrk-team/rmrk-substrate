import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { NftIdTuple } from "./util/fetch";
import { expectTxFailure, getResourceById } from "./util/helpers";
import {
  addNftBasicResource,
  acceptNftResource,
  createCollection,
  mintNft,
  sendNft,
  addNftSlotResource,
  addNftComposableResource,
  replaceResource,
} from "./util/tx";
import { RmrkTraitsResourceResourceInfo as ResourceInfo } from "@polkadot/types/lookup";

describe("integration test: add NFT resource", () => {
  const Alice = "//Alice";
  const Bob = "//Bob";
  const metadata = "test-res-metadata";

  const nonexistentId = 99999;

  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  it("add resource", async () => {
    const collectionIdAlice = await createCollection(
      api,
      10,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Alice,
      Alice,
      collectionIdAlice,
      "nft-metadata"
    );

    await addNftBasicResource(
      api,
      0,
      Alice,
      "added",
      collectionIdAlice,
      nftAlice,
      metadata
    );
  });

  it("add a resource to the nested NFT", async () => {
    const collectionIdAlice = await createCollection(
      api,
      11,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const parentNftId = await mintNft(
      api,
      1,
      Alice,
      Alice,
      collectionIdAlice,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      2,
      Alice,
      Alice,
      collectionIdAlice,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionIdAlice, parentNftId];

    await sendNft(
      api,
      "sent",
      Alice,
      collectionIdAlice,
      childNftId,
      newOwnerNFT
    );

    await addNftBasicResource(
      api,
      1,
      Alice,
      "added",
      collectionIdAlice,
      childNftId,
      metadata
    );
  });

  it("add multiple resources", async () => {
    const collectionIdAlice = await createCollection(
      api,
      12,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Alice,
      Alice,
      collectionIdAlice,
      "nft-metadata"
    );

    const baseId = 42;
    const slotId = 10;
    const parts = [0, 5, 2];

    let resourcesInfo = [];
    const resourceNum = 4;

    const checkResource = async (
      resource: ResourceInfo,
      resType: string,
      expectedId: number,
      expected: {
        metadata: string;
      }
    ) => {
      // FIXME A workaround. It seems it is a PolkadotJS bug.
      // All of the following are `false`.
      //
      // console.log('>>> basic:', resource.resource.isBasic);
      // console.log('>>> composable:', resource.resource.isComposable);
      // console.log('>>> slot:', resource.resource.isSlot);
      const resourceJson = (resource.resource.toHuman() as any)[resType];

      expect(resource.id.toNumber(), "Error: Invalid resource Id").to.be.eq(
        expectedId
      );

      expect(
        resourceJson.metadata,
        "Error: Invalid resource metadata"
      ).to.be.eq(expected.metadata);
    };

    for (let i = 0; i < resourceNum; i++) {
      resourcesInfo.push({
        metadata: metadata + "r-" + i,
      });
    }

    const firstBasicResourceId = await addNftBasicResource(
      api,
      0,
      Alice,
      "added",
      collectionIdAlice,
      nftAlice,
      resourcesInfo[0].metadata
    );

    const secondBasicResourceId = await addNftBasicResource(
      api,
      1,
      Alice,
      "added",
      collectionIdAlice,
      nftAlice,
      resourcesInfo[1].metadata
    );

    const composableResourceId = await addNftComposableResource(
      api,
      2,
      Alice,
      "added",
      collectionIdAlice,
      nftAlice,
      parts,
      baseId,
      resourcesInfo[2].metadata,
      0
    );

    const slotResourceId = await addNftSlotResource(
      api,
      3,
      Alice,
      "added",
      collectionIdAlice,
      nftAlice,
      baseId,
      slotId,
      resourcesInfo[3].metadata
    );

    const firstResource = await getResourceById(
      api,
      collectionIdAlice,
      nftAlice,
      firstBasicResourceId
    );
    await checkResource(
      firstResource,
      "Basic",
      firstBasicResourceId,
      resourcesInfo[0]
    );

    const secondResource = await getResourceById(
      api,
      collectionIdAlice,
      nftAlice,
      secondBasicResourceId
    );
    await checkResource(
      secondResource,
      "Basic",
      secondBasicResourceId,
      resourcesInfo[1]
    );

    const composableResource = await getResourceById(
      api,
      collectionIdAlice,
      nftAlice,
      composableResourceId
    );
    await checkResource(
      composableResource,
      "Composable",
      composableResourceId,
      resourcesInfo[2]
    );

    const slotResource = await getResourceById(
      api,
      collectionIdAlice,
      nftAlice,
      slotResourceId
    );
    await checkResource(slotResource, "Slot", slotResourceId, resourcesInfo[3]);
  });

  it("[negative]: unable to add a resource to the non-existing NFT", async () => {
    const collectionIdAlice = await createCollection(
      api,
      13,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const tx = addNftBasicResource(
      api,
      0,
      Alice,
      "added",
      collectionIdAlice,
      nonexistentId,
      metadata
    );

    await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  });

  it("[negative]: unable to add a resource by a not-an-owner user", async () => {
    const collectionIdAlice = await createCollection(
      api,
      14,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Alice,
      Alice,
      collectionIdAlice,
      "nft-metadata"
    );

    const tx = addNftBasicResource(
      api,
      0,
      Bob,
      "added",
      collectionIdAlice,
      nftAlice,
      metadata
    );

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("[negative]: unable to add a resource to the nested NFT if it isnt root owned by the caller", async () => {
    const collectionIdAlice = await createCollection(
      api,
      15,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const parentNftId = await mintNft(
      api,
      0,
      Alice,
      Alice,
      collectionIdAlice,
      "parent-nft-metadata"
    );
    const childNftId = await mintNft(
      api,
      1,
      Alice,
      Alice,
      collectionIdAlice,
      "child-nft-metadata"
    );

    const newOwnerNFT: NftIdTuple = [collectionIdAlice, parentNftId];

    await sendNft(
      api,
      "sent",
      Alice,
      collectionIdAlice,
      childNftId,
      newOwnerNFT
    );

    const tx = addNftBasicResource(
      api,
      0,
      Bob,
      "added",
      collectionIdAlice,
      childNftId,
      metadata
    );

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("accept resource", async () => {
    const collectionIdBob = await createCollection(
      api,
      16,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Bob,
      Alice,
      collectionIdBob,
      "nft-metadata"
    );

    const resourceId = await addNftBasicResource(
      api,
      0,
      Bob,
      "pending",
      collectionIdBob,
      nftAlice,
      metadata
    );

    await acceptNftResource(api, Alice, collectionIdBob, nftAlice, resourceId);
  });

  it("[negative]: unable to accept a non-existing resource", async () => {
    const collectionIdBob = await createCollection(
      api,
      17,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Bob,
      Alice,
      collectionIdBob,
      "nft-metadata"
    );

    const tx = acceptNftResource(
      api,
      Alice,
      collectionIdBob,
      nftAlice,
      nonexistentId
    );
    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });

  it("[negative]: unable to accept a resource by a not-an-NFT-owner user", async () => {
    const collectionIdBob = await createCollection(
      api,
      18,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Bob,
      Alice,
      collectionIdBob,
      "nft-metadata"
    );

    const resourceId = await addNftBasicResource(
      api,
      0,
      Bob,
      "pending",
      collectionIdBob,
      nftAlice,
      metadata
    );

    const tx = acceptNftResource(
      api,
      Bob,
      collectionIdBob,
      nftAlice,
      resourceId
    );

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("[negative]: unable to accept a resource to a non-target NFT", async () => {
    const collectionIdBob = await createCollection(
      api,
      19,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      0,
      Bob,
      Alice,
      collectionIdBob,
      "nft-metadata"
    );

    const wrongNft = await mintNft(
      api,
      1,
      Bob,
      Alice,
      collectionIdBob,
      "nft-metadata"
    );

    const resourceId = await addNftBasicResource(
      api,
      0,
      Bob,
      "pending",
      collectionIdBob,
      nftAlice,
      metadata
    );

    const tx = acceptNftResource(
      api,
      Alice,
      collectionIdBob,
      wrongNft,
      resourceId
    );

    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });

  after(() => {
    api.disconnect();
  });
});
