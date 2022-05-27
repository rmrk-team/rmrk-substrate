import { expect } from 'chai';
import { getApiConnection } from './substrate/substrate-api';
import { getNft, NftIdTuple } from './util/fetch';
import { expectTxFailure } from './util/helpers';
import { executeTransaction } from "./substrate/substrate-api";

import {
    acceptNft,
    addNftBasicResource,
    createBase,
    createCollection,
    mintNft,
    sendNft,
    removeNftResource,
    acceptResourceRemoval
} from "./util/tx";

import privateKey from "./substrate/privateKey";


describe('Integration test: remove nft resource', () => {
    let api: any;
    before(async () => { api = await getApiConnection(); });

    const Alice = "//Alice";
    const Bob = "//Bob";
    const resourceId = "resid0";
    const src = "test-basic-src";
    const metadata = "test-basic-metadata";
    const license = "test-basic-license";
    const thumb = "test-basic-thumb";

    it('Deleting a resource directly by the NFT owner', async () => {
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

        await removeNftResource(api, 'removed', Alice, collectionIdAlice, nftAlice, resourceId);
    });

    it('Deleting resources indirectly by the NFT owner', async () => {
        const collectionIdAlice = await createCollection(
            api,
            Alice,
            "test-metadata",
            null,
            "test-symbol"
        );
        
        const parentNftId = await mintNft(api, Alice, Alice, collectionIdAlice, "parent-nft-metadata");
        const childNftId = await mintNft(api, Alice, Alice, collectionIdAlice, "child-nft-metadata");

        await addNftBasicResource(
            api,
            Alice,
            "added",
            collectionIdAlice,
            childNftId,
            resourceId,
            src,
            metadata,
            license,
            thumb
        );

        const newOwnerNFT: NftIdTuple = [collectionIdAlice, parentNftId];

        await sendNft(api, "sent", Alice, collectionIdAlice, childNftId, newOwnerNFT);

        await removeNftResource(api, 'removed', Alice, collectionIdAlice, childNftId, resourceId);
    });

    it('Deleting a resource by the collection owner', async () => {
        const collectionIdAlice = await createCollection(
            api,
            Alice,
            "test-metadata",
            null,
            "test-symbol"
        );
      
        const nftBob = await mintNft(
            api,
            Alice,
            Bob,
            collectionIdAlice,
            "nft-metadata"
        );
      
        await addNftBasicResource(
            api,
            Alice,
            "pending",
            collectionIdAlice,
            nftBob,
            resourceId,
            src,
            metadata,
            license,
            thumb
        );

        await removeNftResource(api, 'pending', Alice, collectionIdAlice, nftBob, resourceId);
        await acceptResourceRemoval(api, Bob, collectionIdAlice, nftBob, resourceId);
    });

    it('[Negative test]: cant delete a resource in a non-existing collection', async () => {
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

        const tx = removeNftResource(api, 'removed', Alice, 0xFFFFFFFF, nftAlice, resourceId);
        await expectTxFailure(/rmrkCore.CollectionUnknown/, tx); // FIXME:
    });

    it('[Negative test]: only collection owner can delete a resource', async () => {
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

        const issuer = privateKey(Alice);

        const tx = removeNftResource(api, 'removed', Bob, collectionIdAlice, nftAlice, resourceId);
        await expectTxFailure(/rmrkCore.NoPermission/, tx);
    });

    it('[Negative test]: cannot delete a resource that does not exist', async () => {
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

        const issuer = privateKey(Alice);

        const tx = removeNftResource(api, 'removed', Alice, collectionIdAlice, nftAlice, resourceId);
        await expectTxFailure(/rmrkCore.ResourceDoesntExist/, tx);
    });

    it('[Negative test]: Cannot accept deleting resource without owner attempt do delete it', async () => {
        const collectionIdAlice = await createCollection(
            api,
            Alice,
            "test-metadata",
            null,
            "test-symbol"
        );
      
        const nftBob = await mintNft(
            api,
            Alice,
            Bob,
            collectionIdAlice,
            "nft-metadata"
        );
      
        await addNftBasicResource(
            api,
            Alice,
            "pending",
            collectionIdAlice,
            nftBob,
            resourceId,
            src,
            metadata,
            license,
            thumb
        );

        const tx = acceptResourceRemoval(api, Bob, collectionIdAlice, nftBob, resourceId);
        await expectTxFailure(/rmrkCore.ResourceNotPending/, tx);
    });

    it('[Negative test]: cannot confirm the deletion of a non-existing resource', async () => {
        const collectionIdAlice = await createCollection(
            api,
            Alice,
            "test-metadata",
            null,
            "test-symbol"
        );
      
        const nftBob = await mintNft(
            api,
            Alice,
            Bob,
            collectionIdAlice,
            "nft-metadata"
        );

        const tx = acceptResourceRemoval(api, Bob, collectionIdAlice, nftBob, 'test');
        await expectTxFailure(/rmrkCore.ResourceDoesntExist/, tx);
    });

    it('[Negative test]: Non-owner user cannot confirm the deletion of resource', async () => {
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

        const tx = acceptResourceRemoval(api, Bob, collectionIdAlice, nftAlice, resourceId);
        await expectTxFailure(/rmrkCore.NoPermission/, tx);
    });

    after(() => {
        api.disconnect();
    });
});