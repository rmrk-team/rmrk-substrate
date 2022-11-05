import { getApiConnection } from "./substrate/substrate-api";
import { expectTxFailure } from "./util/helpers";
import {
  createCollection,
  createBase,
  setEquippableList,
  addToEquippableList,
  removeFromEquippableList,
} from "./util/tx";

describe("integration test: set slot's Equippable List", () => {
  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  const alice = "//Alice";
  const bob = "//Bob";

  it("set Base's slot Equippable List", async () => {
    const collectionIds = [
      await createCollection(
        api,
        170,
        alice,
        "equiplist-collection-metadata",
        null,
        "equiplist-0"
      ),
      await createCollection(
        api,
        171,
        alice,
        "equiplist-collection-metadata",
        null,
        "equiplist-1"
      ),
    ];

    const slotId = 202;

    const baseId = await createBase(
      api,
      alice,
      "slotpartany-base-type",
      "slotpartany",
      [
        {
          SlotPart: {
            id: slotId,
            equippable: "All",
            z: 1,
            src: "some-fallback-slot-url",
          },
        },
      ]
    );

    await setEquippableList(api, alice, baseId, slotId, "All");
    await setEquippableList(api, alice, baseId, slotId, "Empty");
    await setEquippableList(api, alice, baseId, slotId, {
      Custom: collectionIds,
    });
    await removeFromEquippableList(
      api,
      alice,
      baseId,
      slotId,
      collectionIds[0]
    );
    await addToEquippableList(api, alice, baseId, slotId, collectionIds[0]);
  });

  it("[negative] unable to set equippable list of a slot of non-existing base", async () => {
    const maxBaseId = 0xffffffff;
    const slotId = 0;

    const tx = setEquippableList(api, alice, maxBaseId, slotId, "All");
    await expectTxFailure(/rmrkEquip\.BaseDoesntExist/, tx);
  });

  it("[negative] unable to set equippable list by a not-an-owner", async () => {
    const slotId = 42;

    const baseId = await createBase(
      api,
      alice,
      "slotpartany-base-type",
      "slotpartany",
      [
        {
          SlotPart: {
            id: slotId,
            equippable: "All",
            z: 1,
            src: "some-fallback-slot-url",
          },
        },
      ]
    );

    const tx = setEquippableList(api, bob, baseId, slotId, "All");
    await expectTxFailure(/rmrkEquip\.PermissionError/, tx);
  });

  it("[negative] unable to set equippable list to a fixed part", async () => {
    const fixedPartId = 42;

    const baseId = await createBase(
      api,
      alice,
      "fixedpart-base-type",
      "fixedpart",
      [
        {
          FixedPart: {
            id: fixedPartId,
            z: 0,
            src: "fixed-part-url",
          },
        },
      ]
    );

    const tx = setEquippableList(api, alice, baseId, fixedPartId, "All");
    await expectTxFailure(/rmrkEquip\.NoEquippableOnFixedPart/, tx);
  });

  it("[negative] unable to set equippable list to non-existing slot", async () => {
    const slotId = 777;
    const maxSlotId = 0xffffffff;

    const baseId = await createBase(
      api,
      alice,
      "slotpartany-base-type",
      "slotpartany",
      [
        {
          SlotPart: {
            id: slotId,
            equippable: "All",
            z: 1,
            src: "some-fallback-slot-url",
          },
        },
      ]
    );

    const tx = setEquippableList(api, alice, baseId, maxSlotId, "All");
    await expectTxFailure(/rmrkEquip\.PartDoesntExist/, tx);
  });

  after(() => {
    api.disconnect();
  });
});
