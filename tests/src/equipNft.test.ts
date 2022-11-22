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
  unequipNft,
} from "./util/tx";

const Alice = "//Alice";
const Bob = "//Bob";

const composableParts: number[] = [5, 2, 7];
const composableMetadata = "test-cmp-metadata";

const slotSrc = "test-slot-src";
const slotMetadata = "test-slot-metadata";

const slotId = 1;

async function createTestCollection(
  api: ApiPromise,
  collectionId: number
): Promise<number> {
  return createCollection(
    api,
    collectionId,
    Alice,
    "test-metadata",
    null,
    "test-symbol"
  );
}

async function mintTestNft(
  api: ApiPromise,
  id: number,
  collectionId: number
): Promise<number> {
  return await mintNft(api, id, Alice, Alice, collectionId, "nft-metadata");
}

async function mintChildNft(
  api: ApiPromise,
  id: number,
  collectionId: number,
  parentNftId: number
): Promise<number> {
  const nftChildId = await mintTestNft(api, id, collectionId);

  const parentNFT: NftIdTuple = [collectionId, parentNftId];

  await sendNft(api, "sent", Alice, collectionId, nftChildId, parentNFT);

  return nftChildId;
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

async function addTestComposable(
  api: ApiPromise,
  collectionId: number,
  nftId: number,
  baseId: number
) {
  await addNftComposableResource(
    api,
    0,
    Alice,
    "added",
    collectionId,
    nftId,
    composableParts,
    baseId,
    composableMetadata,
    0
  );
}

async function addTestSlot(
  api: ApiPromise,
  collectionId: number,
  nftId: number,
  baseId: number,
  slotId: number
): Promise<number> {
  return await addNftSlotResource(
    api,
    0,
    Alice,
    "added",
    collectionId,
    nftId,
    baseId,
    slotId,
    slotMetadata
  );
}

async function checkEquipStatus(
  api: ApiPromise,
  expectedStatus: "equipped" | "unequipped",
  collectionId: number,
  nftId: number,
  expectedResourceId?: number,
  expectedSlotId?: number
) {
  const itemNftDataOpt = await getNft(api, collectionId, nftId);
  expect(itemNftDataOpt.isSome, "Error: unable to fetch item NFT data");

  const itemNftData = itemNftDataOpt.unwrap();
  expect(
    itemNftData.equipped.isSome,
    `Error: item NFT should be ${expectedStatus}`
  ).to.be.equal(expectedStatus === "equipped");

  if (expectedStatus === "equipped" && expectedResourceId && expectedSlotId) {
    expect(
      itemNftData.equipped.unwrap()[0].toNumber(),
      `Error: item NFT should be equipped at ${expectedResourceId} resource`
    ).to.be.equal(expectedResourceId);
  
    expect(
      itemNftData.equipped.unwrap()[1].toNumber(),
      `Error: item NFT should be equipped at ${expectedSlotId} slot`
    ).to.be.equal(expectedSlotId);
  }
}

describe("integration test: Equip NFT", () => {
  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  it("equip nft", async () => {
    const collectionId = await createTestCollection(api, 70);
    const nftParentId = await mintTestNft(api, 101, collectionId);
    const nftChildId = await mintChildNft(api, 102, collectionId, nftParentId);

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    await equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );

    await checkEquipStatus(api, "equipped", collectionId, nftChildId, resourceId, slotId);
  });

  it("unequip nft", async () => {
    const collectionId = await createTestCollection(api, 71);
    const nftParentId = await mintTestNft(api, 105, collectionId);
    const nftChildId = await mintChildNft(api, 106, collectionId, nftParentId);

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    await equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );

    await checkEquipStatus(api, "equipped", collectionId, nftChildId, resourceId, slotId);

    await unequipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await checkEquipStatus(api, "unequipped", collectionId, nftChildId);
  });

  it("[negative] equip NFT onto non-existing NFT", async () => {
    const collectionId = await createTestCollection(api, 72);

    const nftChildId = await mintNft(
      api,
      110,
      Alice,
      Alice,
      collectionId,
      "nft-metadata"
    );

    const itemNFT: NftIdTuple = [collectionId, nftChildId];
    const invalidEquipperNFT: NftIdTuple = [collectionId, 9999999];

    const baseId = 0;
    const resourceId = 0;

    const tx = equipNft(
      api,
      Alice,
      itemNFT,
      invalidEquipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.EquipperDoesntExist/, tx);
  });

  it("[negative] equip non-existing NFT", async () => {
    const collectionId = await createTestCollection(api, 73);
    const nftParentId = await mintNft(
      api,
      120,
      Alice,
      Alice,
      collectionId,
      "nft-metadata"
    );

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const invalidItemNFT: NftIdTuple = [collectionId, 99999999];

    const resourceId = 0;

    const tx = equipNft(
      api,
      Alice,
      invalidItemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.ItemDoesntExist/, tx);
  });

  it("[negative] equip NFT by a not-an-owner user", async () => {
    const collectionId = await createTestCollection(api, 74);
    const nftParentId = await mintTestNft(api, 130, collectionId);
    const nftChildId = await mintChildNft(api, 131, collectionId, nftParentId);

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const tx = equipNft(
      api,
      Bob,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.PermissionError/, tx);
  });

  it("[negative] unable to equip NFT onto indirect parent NFT", async () => {
    const collectionId = await createTestCollection(api, 75);
    const nftParentId = await mintTestNft(api, 140, collectionId);
    const nftChildId = await mintChildNft(api, 141, collectionId, nftParentId);
    const nftGrandchildId = await mintChildNft(
      api,
      142,
      collectionId,
      nftChildId
    );

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftGrandchildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftGrandchildId];

    const tx = equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.MustBeDirectParent/, tx);
  });

  it("[negative] unable to equip NFT onto parent NFT with another base", async () => {
    const collectionId = await createTestCollection(api, 76);
    const nftParentId = await mintTestNft(api, 150, collectionId);
    const nftChildId = await mintChildNft(api, 151, collectionId, nftParentId);

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    const invalidBaseId = 99999;

    const tx = equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      invalidBaseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.NoResourceForThisBaseFoundOnNft/, tx);
  });

  it("[negative] unable to equip NFT into slot with another id", async () => {
    const collectionId = await createTestCollection(api, 77);
    const nftParentId = await mintTestNft(api, 160, collectionId);
    const nftChildId = await mintChildNft(api, 161, collectionId, nftParentId);

    const baseId = await createTestBase(api);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    const incorrectSlotId = slotId + 1;
    const tx = equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      incorrectSlotId
    );
    await expectTxFailure(/rmrkEquip\.ItemHasNoResourceToEquipThere/, tx);
  });

  it("[negative] unable to equip NFT with incorrect slot (fixed part)", async () => {
    const collectionId = await createTestCollection(api, 78);
    const nftParentId = await mintTestNft(api, 170, collectionId);
    const nftChildId = await mintChildNft(api, 171, collectionId, nftParentId);

    const baseId = await createBase(api, Alice, "test-base", "DTBase", [
      {
        FixedPart: {
          id: slotId,
          equippable: "All",
          z: 1,
          src: slotSrc,
        },
      },
    ]);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    const tx = equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.CantEquipFixedPart/, tx);
  });

  it("[negative] unable to equip NFT from a collection that is not allowed by the slot", async () => {
    const collectionId = await createTestCollection(api, 79);
    const nftParentId = await mintTestNft(api, 180, collectionId);
    const nftChildId = await mintChildNft(api, 181, collectionId, nftParentId);

    const baseId = await createBase(api, Alice, "test-base", "DTBase", [
      {
        SlotPart: {
          id: 1,
          z: 1,
          equippable: "Empty",
          src: slotSrc,
        },
      },
    ]);

    await addTestComposable(api, collectionId, nftParentId, baseId);
    const resourceId = await addTestSlot(
      api,
      collectionId,
      nftChildId,
      baseId,
      slotId
    );

    const equipperNFT: NftIdTuple = [collectionId, nftParentId];
    const itemNFT: NftIdTuple = [collectionId, nftChildId];

    const tx = equipNft(
      api,
      Alice,
      itemNFT,
      equipperNFT,
      resourceId,
      baseId,
      slotId
    );
    await expectTxFailure(/rmrkEquip\.CollectionNotEquippable/, tx);
  });

  after(() => {
    api.disconnect();
  });
});
