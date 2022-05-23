import { getApiConnection } from "./substrate/substrate-api";
import { expectTxFailure } from "./util/helpers";
import { burnNft, createCollection, lockCollection, mintNft } from "./util/tx";

describe("Integration test: burn nft", () => {
  const Alice = "//Alice";
  const Bob = "//Bob";

  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  it("Burn nft", async () => {
    await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    ).then(async (collectionId) => {
      const nftId = await mintNft(
        api,
        Alice,
        Alice,
        collectionId,
        "nft-metadata"
      );
      await burnNft(api, nftId, collectionId, Alice);
    });
  });

  it("[Negative] Burn non-existing NFT", async () => {
    await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    ).then(async (collectionId) => {
      const tx = burnNft(api, 99999, collectionId, Alice);
      await expectTxFailure(/rmrkCore.NoAvailableNftId/, tx);
    });
  });

  it("[Negative] Burn not an owner NFT user", async () => {
    await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    ).then(async (collectionId) => {
      const nftId = await mintNft(
        api,
        Alice,
        Alice,
        collectionId,
        "nft-metadata"
      );
      const tx = burnNft(api, nftId, collectionId, Bob);
      await expectTxFailure(/rmrkCore.NoPermission/, tx);
    });
  });

  after(() => {
    api.disconnect();
  });
});
