import { getApiConnection } from "./substrate/substrate-api";
import { NftIdTuple } from "./util/fetch";
import { expectTxFailure } from "./util/helpers";
import {
  acceptNft,
  addNftBasicResource,
  createBase,
  createCollection,
  mintNft,
  sendNft,
} from "./util/tx";

describe("Integration test: Add top-level NFT resource (by the same user)", () => {
  const Alice = "//Alice";
  const Bob = "//Bob";
  const resourceId = "resid0";
  const src = "test-basic-src";
  const metadata = "test-basic-metadata";
  const license = "test-basic-license";
  const thumb = "test-basic-thumb";

  const nonexistentId = 99999;
  let nftAliceId: number;
  let baseId: number;

  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  it("Add resource", async () => {
    const collectionIdAlice = await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
      api,
      Alice,
      Alice,
      collectionIdAlice,
      "nft-metadata"
    );

    await addNftBasicResource(
      api,
      Alice,
      "added",
      collectionIdAlice,
      nftAlice,
      resourceId,
      src,
      metadata,
      license,
      thumb
    );
  });

  // it("Negative: unable to accept a non-existing resource", async () => {
  //   const collectionIdAlice = await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     nftAliceId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     return collectionId;
  //   });

  //   await createCollection(api, Bob, "test-metadata", null, "test-symbol").then(
  //     async (collectionIdBob) => {
  //       const nftBob = await mintNft(
  //         api,
  //         Bob,
  //         Bob,
  //         collectionIdBob,
  //         "nft-metadata"
  //       );
  //       const newOwnerNFT: NftIdTuple = [collectionIdAlice, nftAliceId];

  //       await sendNft(
  //         api,
  //         "pending",
  //         Bob,
  //         collectionIdBob,
  //         nftBob,
  //         newOwnerNFT
  //       );
  //       await acceptNft(api, Alice, collectionIdBob, nftBob, newOwnerNFT);

  //       baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //         {
  //           SlotPart: {
  //             id: slotId,
  //             equippable: "All",
  //             z: 1,
  //             src: slotSrc,
  //           },
  //         },
  //       ]);

  //       const tx = addNftResource(
  //         api,
  //         nonexistentId,
  //         resourceId,
  //         collectionIdAlice,
  //         baseId.toString(),
  //         Alice,
  //         slotId
  //       );

  //       await expectTxFailure(/rmrkCore.NoAvailableNftId/, tx);
  //     }
  //   );
  // });

  // it("Negative: unable to accept a resource by a not-an-NFT-owner user", async () => {
  //   const collectionIdAlice = await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     nftAliceId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     return collectionId;
  //   });

  //   await createCollection(api, Bob, "test-metadata", null, "test-symbol").then(
  //     async (collectionIdBob) => {
  //       const nftBob = await mintNft(
  //         api,
  //         Bob,
  //         Bob,
  //         collectionIdBob,
  //         "nft-metadata"
  //       );
  //       const newOwnerNFT: NftIdTuple = [collectionIdAlice, nftAliceId];

  //       await sendNft(
  //         api,
  //         "pending",
  //         Bob,
  //         collectionIdBob,
  //         nftBob,
  //         newOwnerNFT
  //       );
  //       const tx = acceptNft(api, Bob, collectionIdBob, nftBob, newOwnerNFT);

  //       await expectTxFailure(/rmrkCore.NoPermission/, tx);
  //     }
  //   );
  // });

  // it("Negative: unable to accept a resource to not-a-target NFT", async () => {
  //   const collectionIdAlice = await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     nftAliceId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     return collectionId;
  //   });

  //   await createCollection(api, Bob, "test-metadata", null, "test-symbol").then(
  //     async (collectionIdBob) => {
  //       const nftBob = await mintNft(
  //         api,
  //         Bob,
  //         Bob,
  //         collectionIdBob,
  //         "nft-metadata"
  //       );
  //       const newOwnerNFT: NftIdTuple = [collectionIdAlice, nftAliceId];

  //       await sendNft(
  //         api,
  //         "pending",
  //         Bob,
  //         collectionIdBob,
  //         nftBob,
  //         newOwnerNFT
  //       );

  //       const newOwnerNFTerror: NftIdTuple = [collectionIdAlice, nonexistentId];

  //       const tx = acceptNft(
  //         api,
  //         Alice,
  //         collectionIdBob,
  //         nftBob,
  //         newOwnerNFTerror
  //       );

  //       await expectTxFailure(/rmrkCore.NoAvailableNftId/, tx);
  //     }
  //   );
  // });

  after(() => {
    api.disconnect();
  });
});
