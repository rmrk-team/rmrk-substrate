import { ApiPromise } from "@polkadot/api";
import {
    RmrkTraitsNftAccountIdOrCollectionNftTuple as NftOwner,
    RmrkTraitsPropertyPropertyInfo as Property,
} from "@polkadot/types/lookup";
import type { EventRecord } from '@polkadot/types/interfaces';
import type { GenericEventData } from '@polkadot/types';
import privateKey from "../substrate/privateKey";
import { NftIdTuple, getChildren, getOwnedNfts, getCollectionProperties, getNftProperties } from './fetch';
import chaiAsPromised from 'chai-as-promised';
import chai from 'chai';

chai.use(chaiAsPromised);
const expect = chai.expect;

interface TxResult<T> {
    success: boolean;
    successData: T | null;
}

export function makeNftOwner(api: ApiPromise, owner: string | NftIdTuple): NftOwner {
    const isNftSending = (typeof owner !== "string");

    if (isNftSending) {
        return api.createType("RmrkTraitsNftAccountIdOrCollectionNftTuple", {
            "CollectionAndNftTuple": owner
        });
    } else {
        return api.createType("RmrkTraitsNftAccountIdOrCollectionNftTuple", {
            "AccountId": privateKey(owner).address
        });
    }
}

export async function isNftOwnedBy(
    api: ApiPromise,
    owner: string | NftIdTuple,
    collectionId: number,
    nftId: number
): Promise<boolean> {
    if (typeof owner === "string") {
        return (await getOwnedNfts(api, owner, collectionId))
            .find(ownedNftId => {
                return ownedNftId === nftId;
            }) !== undefined;
    } else {
        return (await getChildren(api, owner[0], owner[1]))
            .find(child => {
                return collectionId === child.collectionId.toNumber()
                    && nftId === child.nftId.toNumber();
            }) !== undefined;
    }
}

export function isPropertyExists(
    key: string,
    value: string,
    props: Property[]
): boolean {
    let isPropFound = false;
    for (var i = 0; i < props.length && !isPropFound; i++) {
        const fetchedProp = props[i];

        isPropFound = fetchedProp.key.eq(key)
                    && fetchedProp.value.eq(value);
    }

    return isPropFound;
}

export async function isCollectionPropertyExists(
    api: ApiPromise,
    collectionId: number,
    key: string,
    value: string
): Promise<boolean> {
    const fetchedProps = await getCollectionProperties(api, collectionId);
    return isPropertyExists(key, value, fetchedProps);
}

export async function isNftPropertyExists(
    api: ApiPromise,
    collectionId: number,
    nftId: number,
    key: string,
    value: string
): Promise<boolean> {
    const fetchedProps = await getNftProperties(api, collectionId, nftId);
    return isPropertyExists(key, value, fetchedProps);
}

export async function isNftChildOfAnother(
    api: ApiPromise,
    collectionId: number,
    nftId: number,
    parentNft: NftIdTuple
): Promise<boolean> {
    return (await getChildren(api, parentNft[0], parentNft[1]))
        .find((childNft) => {
            return childNft.collectionId.toNumber() === collectionId
                && childNft.nftId.toNumber() === nftId;
        }) !== undefined;
}

export function isTxResultSuccess(events: EventRecord[]): boolean {
    let success = false;

    events.forEach(({event: {data, method, section}}) => {
        if (method == 'ExtrinsicSuccess') {
            success = true;
        }
    });

    return success;
}

export async function expectTxFailure(expectedError: RegExp, promise: Promise<any>) {
    await expect(promise).to.be.rejectedWith(expectedError);
}

export function extractTxResult<T>(
    events: EventRecord[],
    expectSection: string,
    expectMethod: string,
    extractAction: (data: GenericEventData) => T
): TxResult<T> {
    let success = false;
    let successData = null;
    events.forEach(({event: {data, method, section}}) => {
        if (method == 'ExtrinsicSuccess') {
            success = true;
        } else if ((expectSection == section) && (expectMethod == method)) {
            successData = extractAction(data);
        }
    });
    const result: TxResult<T> = {
        success,
        successData,
    };
    return result;
}

export function extractRmrkCoreTxResult<T>(
    events: EventRecord[],
    expectMethod: string,
    extractAction: (data: GenericEventData) => T
): TxResult<T> {
    return extractTxResult(events, 'rmrkCore', expectMethod, extractAction);
}

export function extractRmrkEquipTxResult<T>(
    events: EventRecord[],
    expectMethod: string,
    extractAction: (data: GenericEventData) => T
): TxResult<T> {
    return extractTxResult(events, 'rmrkEquip', expectMethod, extractAction);
}
