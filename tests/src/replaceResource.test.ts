import { getApiConnection } from "./substrate/substrate-api";
import {
  addNftBasicResource,
  createCollection,
  mintNft,
  addNftSlotResource,
  replaceResource,
} from "./util/tx";

describe("integration test: replace NFT resource", () => {
  const Alice = "//Alice";
  const Bob = "//Bob";
  const metadata = "test-res-metadata";

  const baseId = 42;
  const slotId = 10;
  const parts = [0, 5, 2];

  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  it("replace resource ( basic )", async () => {
    const collectionId = await createCollection(
      api,
      483,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nft = await mintNft(
      api,
      0,
      Alice,
      Alice,
      collectionId,
      "nft-metadata"
    );

    const resourceId = await addNftBasicResource(
      api,
      0,
      Alice,
      "added",
      collectionId,
      nft,
      metadata
    );

    const resource = {
      Basic: {
        metadata: "basic-resource-nft-minting",
      },
    };

    await replaceResource(api, Bob, collectionId, nft, resourceId, resource);
  });

  it("replace resource ( slot with basic )", async () => {
    const collectionId = await createCollection(
      api,
      484,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nft = await mintNft(
      api,
      0,
      Alice,
      Alice,
      collectionId,
      "nft-metadata"
    );

    const resourceId = await addNftSlotResource(
      api,
      3,
      Alice,
      "added",
      collectionId,
      nft,
      baseId,
      slotId,
      metadata
    );

    const resource = {
      Basic: {
        metadata: "basic-resource-nft-minting",
      },
    };

    await replaceResource(api, Bob, collectionId, nft, resourceId, resource);
  });

  after(() => {
    api.disconnect();
  });
});
