import {default as usingApi, executeTransaction} from '../substrate/substrate-api';
import privateKey from '../substrate/privateKey';
import chaiAsPromised from 'chai-as-promised';
import chai, { expect } from 'chai';
import {extractRmrkCoreTxResult} from './txResult';
import { ApiPromise } from '@polkadot/api';

export async function addResource(
    api: ApiPromise,
    issuerUri: string,
    collectionId: number,
    nftId: number,
    resourceId: string,
    baseId: number | null,
    src: string | null,
    metadata: string | null,
    slotId: number | null,
    license: string | null,
    thumb: string | null,
    parts: number[] | null
) {
    const issuer = privateKey(issuerUri);

    const baseIdOptional = api.createType('Option<u32>', baseId);
    const slotIdOptional = api.createType('Option<u32>', slotId);

    const tx = api.tx.rmrkCore.addResource(
        collectionId,
        nftId,
        resourceId,
        baseIdOptional,
        src,
        metadata,
        slotIdOptional,
        license,
        thumb,
        parts
    );

    const events = await executeTransaction(api, issuer, tx);

    const resourceResult = extractRmrkCoreTxResult(
        events, 'ResourceAdded', (data) => {}
    );

    expect(resourceResult.success).to.be.true;
}
