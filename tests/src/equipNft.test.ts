import { ApiPromise } from "@polkadot/api";
import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { getNft, getParts, NftIdTuple } from "./util/fetch";
import { expectTxFailure } from "./util/helpers";
import {
  addNftComposableResource,
  addNftSlotResource,
  createBase,
  createCollection,
  equipNft,
  mintNft,
  sendNft,
  unequipNft
} from "./util/tx";

const Alice = "//Alice";
const Bob = "//Bob";

// FIXME unable to set anything -- have an error from PolkadotJS
const composableParts: number[] = [];
const composableSrc = null;
const composableMetadata = null;
const composableLicense = null;
const composableThumb = null;

// FIXME unable to set anything -- have an error from PolkadotJS
const slotSrc = null;
const slotLicense = null;
const slotThumb = null;

// const composableResourceId = "comp-0";
// const slotResourceId = "slot-1";
const slotId = 1;

async function createTestCollection(api: ApiPromise): Promise<number> {
  return createCollection(
    api,
    Alice,
    "test-metadata",
    null,
    "test-symbol"
  );
}

async function createTestParentChildNfts(api: ApiPromise, collectionId: number): Promise<[number, number]> {
  const nftParentId = await mintNft(
    api,
    Alice,
    Alice,
    collectionId,
    "nft-metadata"
  );

  const nftChildId = await mintNft(
    api,
    Alice,
    Alice,
    collectionId,
    "nft-metadata"
  );

  const parentNFT: NftIdTuple = [collectionId, nftParentId];

  await sendNft(api, "sent", Alice, collectionId, nftChildId, parentNFT);

  return [nftParentId, nftChildId];
}

async function createTestBase(api: ApiPromise): Promise<number> {
  return createBase(api, Alice, "test-base", "DTBase", [
    {
      SlotPart: {
        id: slotId,
        equippable: "All",
        z: 1,
        src: slotSrc,
      },
    },
  ]);
}

async function addTestComposable(api: ApiPromise, collectionId: number, nftId: number, baseId: number): Promise<number>{
  return await addNftComposableResource(
    api,
    Alice,
    "added",
    collectionId,
    nftId,
    composableParts,
    baseId,
    composableSrc,
    composableMetadata,
    composableLicense,
    composableThumb
  );
}

async function addTestSlot(api: ApiPromise, collectionId: number, nftId: number, baseId: number, slotId: number) {
  await addNftSlotResource(
    api,
    Alice,
    "added",
    collectionId,
    nftId,
    baseId,
    slotId,
    slotSrc,
    slotLicense,
    slotThumb
  );
}

async function checkEquipStatus(
  api: ApiPromise,
  expectedStatus: "equipped" | "unequipped",
  collectionId: number,
  nftId: number
) {
  const itemNftDataOpt = await getNft(api, collectionId, nftId);
  expect(itemNftDataOpt.isSome, 'Error: unable to fetch item NFT data');

  const itemNftData = itemNftDataOpt.unwrap();
  expect(itemNftData.equipped.isTrue, `Error: item NFT should be ${expectedStatus}`)
    .to.be.equal(expectedStatus === "equipped");
}

describe("Integration test: Equip NFT", () => {

  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  it("Equip nft", async () => {
    const collectionId = await createTestCollection(api);
    const [nftParentId, nftChildId] = await createTestParentChildNfts(api, collectionId);

    const baseId = await createTestBase(api);

    const resourceId = await addTestComposable(api, collectionId, nftParentId, baseId);
    await addTestSlot(api, collectionId, nftChildId, baseId, slotId);

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    await equipNft(api, Alice, itemNFT, equipperNFT, resourceId, baseId, slotId);

    await checkEquipStatus(api, "equipped", collectionId, nftChildId);
  });

  it("Unequip nft", async () => {
    const collectionId = await createTestCollection(api);
    const [nftParentId, nftChildId] = await createTestParentChildNfts(api, collectionId);

    const baseId = await createTestBase(api);

    const resourceId = await addTestComposable(api, collectionId, nftParentId, baseId);
    await addTestSlot(api, collectionId, nftChildId, baseId, slotId);

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    await equipNft(api, Alice, itemNFT, equipperNFT, resourceId, baseId, slotId);

    await checkEquipStatus(api, "equipped", collectionId, nftChildId);

    await unequipNft(api, Alice, itemNFT, equipperNFT, resourceId, baseId, slotId);
    await checkEquipStatus(api, "unequipped", collectionId, nftChildId);
  });

  // it("Negative: equip NFT into non-existing NFT", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: slotId,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFTError: NftIdTuple = [collectionId, 9999999];

  //     const tx = equipNft(api, Alice, oldOwnerNFT, newOwnerNFTError, baseId, 1);
  //     await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  //   });
  // });

  // it("Negative: equip non-existing NFT", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: slotId,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, 99999999];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const tx = equipNft(api, Alice, oldOwnerNFT, newOwnerNFT, baseId, 1);
  //     await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  //   });
  // });

  // it("Negative: equip NFT by a not-an-owner user", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: slotId,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const tx = equipNft(api, Bob, oldOwnerNFT, newOwnerNFT, baseId, 1);
  //     await expectTxFailure(/rmrkEquip\.PermissionError/, tx);
  //   });
  // });

  // it("Negative: equip NFT into non-existing by a not-an-owner user", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: slotId,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );
  //     const newOwnerNFTError: NftIdTuple = [collectionId, 99999];

  //     const tx = equipNft(api, Bob, oldOwnerNFT, newOwnerNFTError, baseId, 1);
  //     await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  //   });
  // });

  // it("Negative: unable to equip NFT into indirect parent NFT", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: slotId,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );
  //     const tx = equipNft(api, Bob, oldOwnerNFT, newOwnerNFT, baseId, 1);
  //     await expectTxFailure(/rmrkEquip\.PermissionError/, tx);
  //   });
  // });

  // it("Negative: unable to equip NFT onto parent NFT with non-existing base", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: slotId,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );
  //     const tx = equipNft(api, Alice, oldOwnerNFT, newOwnerNFT, 99999, 1);
  //     await expectTxFailure(/rmrkEquip\.NoResourceForThisBaseFoundOnNft/, tx);
  //   });
  // });

  // it("Negative: unable to equip NFT with incorrect slot", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: 1111,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       "999999"
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       "88888"
  //     );
  //     const tx = equipNft(api, Alice, oldOwnerNFT, newOwnerNFT, baseId, 1);
  //     await expectTxFailure(/rmrkEquip\.ItemHasNoResourceToEquipThere/, tx);
  //   });
  // });

  // it("Negative: unable to equip NFT with incorrect slot", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         FixedPart: {
  //           id: 1,
  //           equippable: "All",
  //           z: 1,
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );
  //     const tx = equipNft(api, Alice, oldOwnerNFT, newOwnerNFT, baseId, 1);
  //     await expectTxFailure(/rmrkEquip\.CantEquipFixedPart/, tx);
  //   });
  // });

  // it("Negative: unable to equip NFT from a collection that is not allowed by the slot", async () => {
  //   await createCollection(
  //     api,
  //     Alice,
  //     "test-metadata",
  //     null,
  //     "test-symbol"
  //   ).then(async (collectionId) => {
  //     const nftParentId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const nftChildId = await mintNft(
  //       api,
  //       Alice,
  //       Alice,
  //       collectionId,
  //       "nft-metadata"
  //     );

  //     const baseId = await createBase(api, Alice, "test-base", "DTBase", [
  //       {
  //         SlotPart: {
  //           id: 1,
  //           z: 1,
  //           equippable: "Empty",
  //           src: slotSrc,
  //         },
  //       },
  //     ]);

  //     await addNftResource(
  //       api,
  //       nftParentId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );

  //     const newOwnerNFT: NftIdTuple = [collectionId, nftParentId];
  //     const oldOwnerNFT: NftIdTuple = [collectionId, nftChildId];

  //     await sendNft(api, "sent", Alice, collectionId, nftChildId, newOwnerNFT);

  //     await addNftResource(
  //       api,
  //       nftChildId,
  //       resourceId,
  //       collectionId,
  //       baseId.toString(),
  //       Alice,
  //       slotId
  //     );
  //     const tx = equipNft(api, Alice, oldOwnerNFT, newOwnerNFT, baseId, 1);
  //     await expectTxFailure(/rmrkEquip\.CollectionNotEquippable/, tx);
  //   });
  // });

  after(() => {
    api.disconnect();
  });
});
