import { ApiPromise } from "@polkadot/api";
import { Bytes, Option, u32, Vec } from '@polkadot/types-codec';
import {
    UpDataStructsRmrkAccountIdOrCollectionNftTuple as NftOwner,
    UpDataStructsRmrkBasicResource as BasicResource,
    UpDataStructsRmrkComposableResource as ComposableResource,
    UpDataStructsRmrkSlotResource as SlotResource,
    UpDataStructsRmrkResourceInfo as ResourceInfo,
    UpDataStructsRmrkEquippableList as EquippableList,
    UpDataStructsRmrkPartType as PartType,
    UpDataStructsRmrkTheme as Theme
} from "@polkadot/types/lookup";
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import privateKey from "../substrate/privateKey";
import { executeTransaction } from "../substrate/substrate-api";
import '../interfaces/augment-api';
import {
  getBase,
  getCollection,
  getCollectionsCount,
  getEquippableList,
  getNft,
  getParts,
  getCollectionProperties,
  getResourcePriorities,
  getTheme,
  NftIdTuple,
  getResources,
} from "./fetch";
import {
  extractRmrkCoreTxResult,
  extractRmrkEquipTxResult, isNftOwnedBy, isTxResultSuccess, makeNftOwner,
  isCollectionPropertyExists,
  isNftPropertyExists
} from "./helpers";
import { IKeyringPair } from "@polkadot/types/types";

chai.use(chaiAsPromised);
const expect = chai.expect;

export async function createCollection(
    api: ApiPromise,
    issuerUri: string,
    metadata: string,
    max: number | null,
    symbol: string
  ): Promise<number> {
    let collectionId = 0;

    const oldCollectionCount = await getCollectionsCount(api);
    const maxOptional = max ? max.toString() : null;

    const issuer = privateKey(issuerUri);
    const tx = api.tx.rmrkCore.createCollection(metadata, maxOptional, symbol);
    const events = await executeTransaction(api, issuer, tx);

    const collectionResult = extractRmrkCoreTxResult(
        events, 'CollectionCreated', (data) => {
            return parseInt(data[1].toString(), 10)
        }
    );
    expect(collectionResult.success, 'Error: unable to create a collection').to.be.true;

    collectionId = collectionResult.successData!;

    const newCollectionCount = await getCollectionsCount(api);
    const collectionOption = await getCollection(api, collectionId);

    expect(newCollectionCount).to.be.equal(oldCollectionCount + 1, 'Error: NFT collection count should increase');
    expect(collectionOption.isSome, 'Error: unable to fetch created NFT collection').to.be.true;

    const collection = collectionOption.unwrap();

    expect(collection.metadata.toUtf8()).to.be.equal(metadata, "Error: Invalid NFT collection metadata");
    expect(collection.max.isSome).to.be.equal(max !== null, "Error: Invalid NFT collection max");

    if (collection.max.isSome) {
        expect(collection.max.unwrap().toNumber()).to.be.equal(max, "Error: Invalid NFT collection max");
    }
    expect(collection.symbol.toUtf8()).to.be.equal(symbol, "Error: Invalid NFT collection's symbol");
    expect(collection.nftsCount.toNumber()).to.be.equal(0, "Error: NFT collection shoudn't have any tokens");
    expect(collection.issuer.toString()).to.be.equal(issuer.address, "Error: Invalid NFT collection issuer");

    return collectionId;
}

export async function changeIssuer(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    newIssuer: string
) {
    const alice = privateKey(issuerUri);
    const bob = privateKey(newIssuer);

    let tx = api.tx.uniques.setAcceptOwnership(
        api.createType('Option<u32>', collectionId)
    );
    let events = await executeTransaction(api, bob, tx);
    expect(isTxResultSuccess(events), 'Error: Unable to accept ownership').to.be.true;

    tx = api.tx.rmrkCore.changeCollectionIssuer(collectionId, bob.address);
    events = await executeTransaction(api, alice, tx);
    expect(isTxResultSuccess(events), 'Error: Unable to change NFT collection issuer').to.be.true;

    await getCollection(api, collectionId).then((collectionOption) => {
        const collection = collectionOption.unwrap();
        expect(collection.issuer.toString())
            .to.be.deep.eq(bob.address, 'Error: Invalid NFT collection issuer');
    });
}

export async function deleteCollection(
    api: ApiPromise,
    issuerUri: string,
    collectionId: string
): Promise<number> {
    const issuer = privateKey(issuerUri);
    const tx = api.tx.rmrkCore.destroyCollection(collectionId);
    const events = await executeTransaction(api, issuer, tx);

    const collectionTxResult = extractRmrkCoreTxResult(
        events,
        "CollectionDestroy",
        (data) => {
        return parseInt(data[1].toString(), 10);
        }
    );
    expect(collectionTxResult.success, 'Error: Unable to delete NFT collection').to.be.true;

    const collection = await getCollection(
        api,
        parseInt(collectionId, 10)
    );
    expect(collection.isEmpty, 'Error: NFT collection should be deleted').to.be.true;

    return 0;
}

export async function negativeDeleteCollection(
    api: ApiPromise,
    issuerUri: string,
    collectionId: string
): Promise<number> {
    const issuer = privateKey(issuerUri);
    const tx = api.tx.rmrkCore.destroyCollection(collectionId);
    await expect(executeTransaction(api, issuer, tx)).to.be.rejected;

    return 0;
}

export async function setNftProperty(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
    key: string,
    value: string
) {
    const issuer = privateKey(issuerUri);
    const nftIdOpt = api.createType('Option<u32>', nftId);
    const tx = api.tx.rmrkCore.setProperty(
        collectionId,
        nftIdOpt,
        key,
        value
    );
    const events = await executeTransaction(api, issuer, tx);

    const propResult = extractRmrkCoreTxResult(
        events, 'PropertySet', (data) => {
            return {
                collectionId: parseInt(data[0].toString(), 10),
                nftId: data[1] as Option<u32>,
                key: data[2] as Bytes,
                value: data[3] as Bytes
            };
        }
    );

    expect(propResult.success, 'Error: Unable to set NFT property').to.be.true;
    if (propResult.successData) {
        const eventData = propResult.successData;
        const eventDescription = 'from set NFT property event';

        expect(eventData.collectionId, 'Error: Invalid collection ID ' + eventDescription)
            .to.be.equal(collectionId);

        expect(eventData.nftId.eq(nftIdOpt), 'Error: Invalid NFT ID ' + eventDescription)
            .to.be.true;

        expect(eventData.key.eq(key), 'Error: Invalid property key ' + eventDescription)
            .to.be.true;

        expect(eventData.value.eq(value), 'Error: Invalid property value ' + eventDescription)
            .to.be.true;
    }

    expect(
        await isNftPropertyExists(api, collectionId, nftId, key, value),
        'Error: NFT property is not found'
    ).to.be.true;
}

export async function mintNft(
    api: ApiPromise,
    issuerUri: string,
    ownerUri: string,
    collectionId: number,
    metadata: string,
    recipientUri: string | null = null,
    royalty: number | null = null,
    transferable: boolean = true,
): Promise<number> {
    let nftId = 0;

    const issuer = privateKey(issuerUri);
    const owner = privateKey(ownerUri).address;
    const recipient = recipientUri ? privateKey(recipientUri).address : null;
    const royaltyOptional = royalty ? royalty.toString() : null;

    const collectionOpt = await getCollection(api, collectionId);

    const tx = api.tx.rmrkCore.mintNft(
        owner,
        collectionId,
        recipient,
        royaltyOptional,
        metadata,
        transferable
    );

    const events = await executeTransaction(api, issuer, tx);
    const nftResult = extractRmrkCoreTxResult(
        events, 'NftMinted', (data) => {
            return parseInt(data[2].toString(), 10);
        }
    );

    expect(nftResult.success, 'Error: Unable to mint NFT').to.be.true;

    const newCollectionNftsCount = (await getCollection(api, collectionId))
        .unwrap()
        .nftsCount
        .toNumber();

    const oldCollectionNftsCount = collectionOpt
      .unwrap()
      .nftsCount.toNumber();

    expect(newCollectionNftsCount, 'Error: NFTs count should increase')
        .to.be.equal(oldCollectionNftsCount + 1);

    nftId = nftResult.successData!;

    const nftOption = await getNft(api, collectionId, nftId);

    expect(nftOption.isSome, 'Error: Unable to fetch created NFT').to.be.true;

    const nft = nftOption.unwrap();

    // FIXME the ownership is the uniques responsibility
    // so the `owner` field should be removed from the NFT info.
    expect(nft.owner.isAccountId, 'Error: NFT owner should be some user').to.be.true;
    expect(nft.owner.asAccountId.toString()).to.be.equal(owner, "Error: Invalid NFT owner");

    const isOwnedInUniques = await isNftOwnedBy(api, ownerUri, collectionId, nftId);
    expect(isOwnedInUniques, `Error: created NFT is not actually owned by ${ownerUri}`)
        .to.be.true;

    if (recipient === null && royalty === null) {
        expect(nft.royalty.isNone, 'Error: Invalid NFT recipient')
            .to.be.true;
    } else {
        expect(nft.royalty.isSome, 'Error: NFT royalty not found')
            .to.be.true;

        const nftRoyalty = nft.royalty.unwrap();
        expect(nftRoyalty.recipient.eq(recipient), 'Error: Invalid NFT recipient')
            .to.be.true;

        expect(nftRoyalty.amount.eq(royalty), 'Error: Invalid NFT royalty')
            .to.be.true;
    }

    expect(nft.metadata.toUtf8()).to.be.equal(metadata, "Error: Invalid NFT metadata");

    return nftId;
}

export async function sendNft(
    api: ApiPromise,
    expectedStatus: "pending" | "sent",
    originalOwnerUri: string,
    collectionId: number,
    nftId: number,
    newOwner: string | NftIdTuple
) {
    const originalOwner = privateKey(originalOwnerUri);
    const newOwnerObj = makeNftOwner(api, newOwner);

    const nftBeforeSendingOpt = await getNft(api, collectionId, nftId);

    const tx = api.tx.rmrkCore.send(collectionId, nftId, newOwnerObj);
    const events = await executeTransaction(api, originalOwner, tx);

    const sendResult = extractRmrkCoreTxResult(events, "NFTSent", (data) => {
        return {
            dstOwner: data[1] as NftOwner,
            collectionId: parseInt(data[2].toString(), 10),
            nftId: parseInt(data[3].toString(), 10)
        };
    });

    expect(sendResult.success, 'Error: Unable to send NFT').to.be.true;
    if (sendResult.successData) {
        const sendData = sendResult.successData;

        expect(sendData.dstOwner.eq(newOwnerObj), 'Error: Invalid target user (from event data)')
            .to.be.true;

        expect(sendData.collectionId)
            .to.be.equal(collectionId, 'Error: Invalid collection ID (from event data)');

        expect(sendData.nftId).to.be.equal(nftId, 'Error: Invalid NFT ID (from event data)');
    }

    expect(nftBeforeSendingOpt.isSome, 'Error: Unable to fetch NFT before sending').to.be.true;

    const nftBeforeSending = nftBeforeSendingOpt.unwrap();

    const nftAfterSendingOpt = await getNft(api, collectionId, nftId);

    expect(nftAfterSendingOpt.isSome, 'Error: Unable to fetch NFT after sending').to.be.true;

    const nftAfterSending = nftAfterSendingOpt.unwrap();

    // FIXME the ownership is the uniques responsibility
    // so the `owner` field should be removed from the NFT info.
    // expect(nftAfterSending.owner.eq(newOwnerObj), 'Error: Invalid NFT owner after sending')
    //     .to.be.true;

    const isOwnedInUniques = await isNftOwnedBy(api, newOwner, collectionId, nftId);
    expect(isOwnedInUniques, `Error: created NFT is not actually owned by ${newOwner.toString()}`)
        .to.be.true;

    expect(nftAfterSending.royalty.eq(nftBeforeSending.royalty), 'Error: Invalid NFT royalty after sending')
        .to.be.true;

    expect(nftAfterSending.metadata.eq(nftBeforeSending.metadata), 'Error: Invalid NFT metadata after sending')
        .to.be.true;

    expect(nftAfterSending.equipped.eq(nftBeforeSending.equipped), 'Error: Invalid NFT equipped status after sending')
        .to.be.true;

    expect(nftAfterSending.pending.eq(expectedStatus === "pending"), 'Error: Invalid NFT pending state')
        .to.be.true;
}

export async function acceptNft(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
    newOwner: string | [number, number]
) {
    const issuer = privateKey(issuerUri);
    const newOwnerObj = makeNftOwner(api, newOwner);

    let nftBeforeOpt = await getNft(api, collectionId, nftId);

    const tx = api.tx.rmrkCore.acceptNft(collectionId, nftId, newOwnerObj);
    const events = await executeTransaction(api, issuer, tx);

    const acceptResult = extractRmrkCoreTxResult(events, "NFTAccepted", (data) => {
        return {
            recipient: data[1] as NftOwner,
            collectionId: parseInt(data[2].toString(), 10),
            nftId: parseInt(data[3].toString(), 10)
        };
    });

    expect(acceptResult.success, 'Error: Unable to accept NFT').to.be.true;
    if (acceptResult.successData) {
        const acceptData = acceptResult.successData;

        expect(acceptData.recipient.eq(newOwnerObj), 'Error: Invalid NFT recipient (from event data)')
            .to.be.true;

        expect(acceptData.collectionId)
            .to.be.equal(collectionId, 'Error: Invalid collection ID (from event data)');

        expect(acceptData.nftId)
            .to.be.equal(nftId, 'Error: Invalid NFT ID (from event data)');
    }

    const nftBefore = nftBeforeOpt.unwrap();

    const isPendingBeforeAccept = nftBefore.pending.isTrue;

    const nftAfter = (await getNft(api, collectionId, nftId)).unwrap();
    const isPendingAfterAccept = nftAfter.pending.isTrue;

    expect(isPendingBeforeAccept, 'Error: NFT should be pending to be accepted')
        .to.be.true;

    expect(isPendingAfterAccept, 'Error: NFT should NOT be pending after accept')
        .to.be.false;

    const isOwnedInUniques = await isNftOwnedBy(api, newOwner, collectionId, nftId);
    expect(isOwnedInUniques, `Error: created NFT is not actually owned by ${newOwner.toString()}`)
        .to.be.true;
}

export async function rejectNft(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
) {
    const issuer = privateKey(issuerUri);
    let nftBeforeOpt = await getNft(api, collectionId, nftId);

    const tx = api.tx.rmrkCore.rejectNft(collectionId, nftId);
    const events = await executeTransaction(api, issuer, tx);
    const rejectResult = extractRmrkCoreTxResult(events, "NFTRejected", (data) => {
        return {
            collectionId: parseInt(data[1].toString(), 10),
            nftId: parseInt(data[2].toString(), 10)
        };
    });

    if (rejectResult.successData) {
        const rejectData = rejectResult.successData;

        expect(rejectData.collectionId)
            .to.be.equal(collectionId, 'Error: Invalid collection ID (from event data)');

        expect(rejectData.nftId)
            .to.be.equal(nftId, 'Error: Invalid NFT ID (from event data)');
    }

    const nftBefore = nftBeforeOpt.unwrap();

    const isPendingBeforeReject = nftBefore.pending.isTrue;

    const nftAfter = await getNft(api, collectionId, nftId);

    expect(isPendingBeforeReject, 'Error: NFT should be pending to be rejected')
        .to.be.true;

    expect(nftAfter.isNone, 'Error: NFT should be burned after reject')
        .to.be.true;
}

export async function createBase(
    api: ApiPromise,
    issuerUri: string,
    baseType: string,
    symbol: string,
    parts: object[]
): Promise<number> {
    let baseId = 0;

    const issuer = privateKey(issuerUri);

    const partTypes = api.createType("Vec<RmrkTraitsPartPartType>", parts) as Vec<PartType>;

    const tx = api.tx.rmrkEquip.createBase(baseType, symbol, partTypes);
    const events = await executeTransaction(api, issuer, tx);

    const baseResult = extractRmrkEquipTxResult(
        events, 'BaseCreated', (data) => {
            return parseInt(data[1].toString(), 10);
        }
    );

    expect(baseResult.success, 'Error: Unable to create Base')
        .to.be.true;

    baseId = baseResult.successData!;
    const baseOptional = await getBase(api, baseId);

    expect(baseOptional.isSome, 'Error: Unable to fetch created Base')
        .to.be.true;

    const base = baseOptional.unwrap();
    const baseParts = await getParts(api, baseId);

    expect(base.issuer.toString()).to.be.equal(issuer.address, "Error: Invalid Base issuer");
    expect(base.baseType.toUtf8()).to.be.equal(baseType, "Error: Invalid Base type");
    expect(base.symbol.toUtf8()).to.be.equal(symbol, "Error: Invalid Base symbol");
    expect(partTypes.eq(baseParts), "Error: Received invalid base parts").to.be.true;

    return baseId;
}

export async function setResourcePriorities(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
    priorities: number[]
) {
    const issuer = privateKey(issuerUri);

    const prioritiesVec = api.createType('Vec<u32>', priorities);
    const tx = api.tx.rmrkCore.setPriority(collectionId, nftId, prioritiesVec);
    const events = await executeTransaction(api, issuer, tx);

    const prioResult = extractRmrkCoreTxResult(events, 'PrioritySet', (data) => {
        return {
            collectionId: parseInt(data[0].toString(), 10),
            nftId: parseInt(data[1].toString(), 10)
        };
    });

    expect(prioResult.success, 'Error: Unable to set resource priorities').to.be.true;
    if (prioResult.successData) {
        const eventData = prioResult.successData;

        expect(eventData.collectionId)
            .to.be.equal(collectionId, 'Error: Invalid collection ID (set priorities event data)');

        expect(eventData.nftId).to.be.equal(nftId, 'Error: Invalid NFT ID (set priorities event data');
    }

    const fetchedPrios = await getResourcePriorities(api, collectionId, nftId);

    expect(fetchedPrios).to.be.deep.equal(priorities, 'Error: Invalid priorities are set');
}

export async function setEquippableList(
    api: ApiPromise,
    issuerUri: string,
    baseId: number,
    slotId: number,
    equippableList: "All" | "Empty" | { 'Custom': number[] }
) {
    const issuer = privateKey(issuerUri);
    const equippable = api.createType('RmrkTraitsPartEquippableList', equippableList) as EquippableList;

    const tx = api.tx.rmrkEquip.equippable(baseId, slotId, equippable);
    const events = await executeTransaction(api, issuer, tx);

    const equipListResult = extractRmrkEquipTxResult(events, 'EquippablesUpdated', (data) => {
        return {
            baseId: parseInt(data[0].toString(), 10),
            slotId: parseInt(data[1].toString(), 10)
        };
    });

    expect(equipListResult.success, 'Error: unable to update equippable list').to.be.true;
    if (equipListResult.successData) {
        const updateEvent = equipListResult.successData;

        expect(updateEvent.baseId)
            .to.be.equal(baseId, 'Error: invalid base ID from update equippable event');

        expect(updateEvent.slotId)
            .to.be.equal(slotId, 'Error: invalid base ID from update equippable event');
    }

    const fetchedEquippableList = await getEquippableList(api, baseId, slotId);

    expect(fetchedEquippableList, 'Error: unable to fetch equippable list').to.be.not.null;
    if (fetchedEquippableList) {
        expect(fetchedEquippableList)
            .to.be.deep.equal(equippableList, 'Error: invalid equippable list was set');
    }
}

export async function addTheme(
    api: ApiPromise,
    issuerUri: string,
    baseId: number,
    themeObj: object,
    filterKeys: string[] | null = null
) {
    const issuer = privateKey(issuerUri);
    const theme = api.createType('RmrkTraitsTheme', themeObj) as Theme;

    const tx = api.tx.rmrkEquip.themeAdd(baseId, theme);
    const events = await executeTransaction(api, issuer, tx);

    expect(isTxResultSuccess(events), 'Error: Unable to add Theme').to.be.true;

    const fetchedThemeOpt = await getTheme(api, baseId, theme.name.toUtf8(), null);

    expect(fetchedThemeOpt.isSome, 'Error: Unable to fetch theme').to.be.true;

    const fetchedTheme = fetchedThemeOpt.unwrap();

    expect(theme.name.eq(fetchedTheme.name), 'Error: Invalid theme name').to.be.true;

    for (var i = 0; i < theme.properties.length; i++) {
        const property = theme.properties[i];
        const propertyKey = property.key.toUtf8();

        const propertyFoundCount = fetchedTheme.properties.filter(
            (fetchedProp) => property.key.eq(fetchedProp.key)
        ).length;

        expect(propertyFoundCount > 1, `Error: Too many properties with key ${propertyKey} found`)
            .to.be.false;

        if (filterKeys) {
            const isFiltered = fetchedTheme.properties.find(
                (fetchedProp) => fetchedProp.key.eq(property.key)
            ) === undefined;

            if (isFiltered) {
                expect(propertyFoundCount === 0, `Error: Unexpected filtered key ${propertyKey}`)
                    .to.be.true;
                continue;
            }
        }

        expect(propertyFoundCount === 1, `Error: The property with key ${propertyKey} is not found`)
            .to.be.true;
    }
}

export async function lockCollection(
  api: ApiPromise,
  issuerUri: string,
  collectionId: number,
  max: number = 0
) {
  const issuer = privateKey(issuerUri);
  const tx = api.tx.rmrkCore.lockCollection(collectionId);
  const events = await executeTransaction(api, issuer, tx);
  expect(isTxResultSuccess(events)).to.be.true;

  await getCollection(api, collectionId).then((collectionOption) => {
    const collection = collectionOption.unwrap();
    expect(collection.max.unwrap().toNumber()).to.be.equal(max);
  });
}

export async function setPropertyCollection(
  api: ApiPromise,
  issuerUri: string,
  collectionId: number,
  key: string,
  value: string
) {
  const alice = privateKey(issuerUri);

  const tx = api.tx.rmrkCore.setProperty(collectionId, null, key, value);
  const events = await executeTransaction(api, alice, tx);
  expect(isTxResultSuccess(events)).to.be.true;

  expect(await isCollectionPropertyExists(api, collectionId, key, value))
    .to.be.true;
}

export async function burnNft(
  api: ApiPromise,
  issuerUri: string,
  collectionId: number,
  nftId: number
) {
  const issuer = privateKey(issuerUri);
  const tx = api.tx.rmrkCore.burnNft(collectionId, nftId);
  const events = await executeTransaction(api, issuer, tx);
  expect(isTxResultSuccess(events)).to.be.true;

  const nftBurned = await getNft(api, collectionId, nftId);
  expect(nftBurned.isSome).to.be.false;
}

async function findResourceById(
    api: ApiPromise,
    collectionId: number,
    nftId: number,
    resourceId: number,
): Promise<ResourceInfo> {
    const resources = await getResources(api, collectionId, nftId);

    let resource = null;

    for (let i = 0; i < resources.length; i++) {
        const res = resources[i];

        if (res.id.eq(resourceId)) {
            resource = res;
            break;
        }
    }

    return resource!;
}

function checkResourceStatus(
    resource: ResourceInfo,
    expectedStatus: "pending" | "added"
) {
    expect(resource.pending.isTrue, `Error: added resource should be ${expectedStatus}`)
          .to.be.equal(expectedStatus === "pending");
}

export async function acceptNftResource(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
    resourceId: number,
) {
    const issuer = privateKey(issuerUri);

    const tx = api.tx.rmrkCore.acceptResource(
        collectionId,
        nftId,
        resourceId,
    );

    const events = await executeTransaction(api, issuer, tx);
    expect(isTxResultSuccess(events)).to.be.true;

    const resource = await findResourceById(api, collectionId, nftId, resourceId);
    checkResourceStatus(resource!, "added");
}

async function executeResourceCreation(
    api: ApiPromise,
    issuer: IKeyringPair,
    tx: any,
    collectionId: number,
    nftId: number,
    expectedStatus: "pending" | "added"
): Promise<number> {
    const events = await executeTransaction(api, issuer, tx);

    const resourceResult = extractRmrkCoreTxResult(
        events, 'ResourceAdded', (data) => {
            return parseInt(data[1].toString(), 10);
        }
    );
    expect(resourceResult.success, 'Error: Unable to add resource').to.be.true;
    const resourceId = resourceResult.successData!;

    const resource = await findResourceById(api, collectionId, nftId, resourceId);
    expect(resource !== null, 'Error: resource was not found').to.be.true;
    checkResourceStatus(resource!, expectedStatus);

    return resourceId;
}

export async function addNftBasicResource(
    api: ApiPromise,
    issuerUri: string,
    expectedStatus: "pending" | "added",
    collectionId: number,
    nftId: number,
    src: string | null,
    metadata: string | null,
    license: string | null,
    thumb: string | null
): Promise<number> {
    const issuer = privateKey(issuerUri);

    const basicResource = api.createType('RmrkTraitsResourceBasicResource', {
        src: src,
        metadata: metadata,
        license: license,
        thumb: thumb
    }) as BasicResource;

    const tx = api.tx.rmrkCore.addBasicResource(
        collectionId,
        nftId,
        basicResource
    );
    
    const resourceId = executeResourceCreation(api, issuer, tx, collectionId, nftId, expectedStatus);
    return resourceId;
}

export async function addNftComposableResource(
    api: ApiPromise,
    issuerUri: string,
    expectedStatus: "pending" | "added",
    collectionId: number,
    nftId: number,
    parts: number[],
    baseId: number,
    src: string | null,
    metadata: string | null,
    license: string | null,
    thumb: string | null
): Promise<number> {
    const issuer = privateKey(issuerUri);

    const composableResource = api.createType('RmrkTraitsResourceComposableResource', {
        parts: parts, // api.createType('Vec<u32>', parts),
        base: baseId,
        src: src,
        metadata: metadata,
        license: license,
        thumb: thumb
    }) as ComposableResource;

    const tx = api.tx.rmrkCore.addComposableResource(
        collectionId,
        nftId,
        "",
        composableResource
    );

    const resourceId = executeResourceCreation(api, issuer, tx, collectionId, nftId, expectedStatus);
    return resourceId;
}

export async function addNftSlotResource(
  api: ApiPromise,
  issuerUri: string,
  expectedStatus: "pending" | "added",
  collectionId: number,
  nftId: number,
  baseId: number,
  slotId: number,
  src: string | null,
  license: string | null,
  thumb: string | null
): Promise<number>  {
  const issuer = privateKey(issuerUri);

  const slotResource = api.createType('RmrkTraitsResourceSlotResource', {
      base: baseId,
      src: src,
      metadata: "slot-resource-metadata",
      slot: slotId,
      license: license,
      thumb: thumb
  }) as SlotResource;

  const tx = api.tx.rmrkCore.addSlotResource(
    collectionId,
    nftId,
    slotResource
  );
  
  const resourceId = executeResourceCreation(api, issuer, tx, collectionId, nftId, expectedStatus);
  return resourceId;
}

export async function equipNft(
  api: ApiPromise,
  issuerUri: string,
  item: any,
  equipper: any,
  resource: number,
  base: number,
  slot: number
) {
  const issuer = privateKey(issuerUri);
  const tx = api.tx.rmrkEquip.equip(item, equipper, resource, base, slot);
  const events = await executeTransaction(api, issuer, tx);
  expect(isTxResultSuccess(events)).to.be.true;
}

export async function unequipNft(
  api: ApiPromise,
  issuerUri: string,
  item: any,
  equipper: any,
  resource: number,
  base: number,
  slot: number
) {
  const issuer = privateKey(issuerUri);
  const tx = api.tx.rmrkEquip.equip(item, equipper, resource, base, slot);
  const events = await executeTransaction(api, issuer, tx);

  const result = extractRmrkEquipTxResult(
    events,
    "SlotUnequipped",
    (data) => {
      return parseInt(data[1].toString(), 10);
    }
  );

  expect(result.success).to.be.true;
  expect(isTxResultSuccess(events)).to.be.true;
}

export async function removeNftResource(
    api: ApiPromise, 
    expectedStatus: "pending" | "removed", 
    issuerUri: string, 
    collectionId: number, 
    nftId: number, 
    resourceId: number
) {
    const issuer = privateKey(issuerUri);
    
    const tx = api.tx.rmrkCore.removeResource(collectionId, nftId, resourceId);
    const events = await executeTransaction(api, issuer, tx);
    expect(isTxResultSuccess(events)).to.be.true;

    let afterDeleting = await findResourceById(api, collectionId, nftId, resourceId);

    if (expectedStatus === 'pending') {
        expect(afterDeleting).not.to.be.null;
        expect(afterDeleting?.pendingRemoval.isTrue).to.be.equal(true);
    } else {
        expect(afterDeleting).to.be.null;
    }
}

export async function acceptResourceRemoval(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
    resourceId: number
) {
    const issuer = privateKey(issuerUri);

    const tx = api.tx.rmrkCore.acceptResourceRemoval(collectionId, nftId, resourceId);
    const events = await executeTransaction(api, issuer, tx);
    expect(isTxResultSuccess(events)).to.be.true;

    let afterDeleting = await findResourceById(api, collectionId, nftId, resourceId);
    expect(afterDeleting, 'Error: resource deleting failed').to.be.null;
}
