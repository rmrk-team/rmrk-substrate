import { getApiConnection } from "./substrate/substrate-api";
import { NftIdTuple } from "./util/fetch";
import { expectTxFailure } from "./util/helpers";
import {
  addNftBasicResource,
  acceptNftResource,
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
      src,
      metadata,
      license,
      thumb
    );
  });

  it('add a resource to the nested NFT', async () => {
    const collectionIdAlice = await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const parentNftId = await mintNft(api, Alice, Alice, collectionIdAlice, "parent-nft-metadata");
    const childNftId = await mintNft(api, Alice, Alice, collectionIdAlice, "child-nft-metadata");

    const newOwnerNFT: NftIdTuple = [collectionIdAlice, parentNftId];

    await sendNft(api, "sent", Alice, collectionIdAlice, childNftId, newOwnerNFT);

    await addNftBasicResource(
      api,
      Alice,
      "added",
      collectionIdAlice,
      childNftId,
      src,
      metadata,
      license,
      thumb
    );
  });

  it('[Negative test]: unable to add a resource to the non-existing NFT', async () => {
    const collectionIdAlice = await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const tx = addNftBasicResource(
      api,
      Alice,
      "added",
      collectionIdAlice,
      nonexistentId,
      src,
      metadata,
      license,
      thumb
    );
  
    await expectTxFailure(/rmrkCore\.NoAvailableNftId/, tx);
  });

  it('[Negative test]: unable to add a resource by a not-an-owner user', async () => {
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

    const tx = addNftBasicResource(
      api,
      Bob,
      "added",
      collectionIdAlice,
      nftAlice,
      src,
      metadata,
      license,
      thumb
    );
  
    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it('[Negative test]: unable to add a resource to the nested NFT if it isnt root owned by the caller', async () => {
    const collectionIdAlice = await createCollection(
      api,
      Alice,
      "test-metadata",
      null,
      "test-symbol"
    );

    const parentNftId = await mintNft(api, Alice, Alice, collectionIdAlice, "parent-nft-metadata");
    const childNftId = await mintNft(api, Alice, Alice, collectionIdAlice, "child-nft-metadata");

    const newOwnerNFT: NftIdTuple = [collectionIdAlice, parentNftId];

    await sendNft(api, "sent", Alice, collectionIdAlice, childNftId, newOwnerNFT);

    const tx = addNftBasicResource(
      api,
      Bob,
      "added",
      collectionIdAlice,
      childNftId,
      src,
      metadata,
      license,
      thumb
    );
    
    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("Accept resource", async () => {
    const collectionIdBob = await createCollection(
      api,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdBob,
        "nft-metadata"
    );

    const resourceId = await addNftBasicResource(
      api,
      Bob,
      "pending",
      collectionIdBob,
      nftAlice,
      src,
      metadata,
      license,
      thumb
    );

    await acceptNftResource(api, Alice, collectionIdBob, nftAlice, resourceId);
  });

  it("[Negative test]: unable to accept a non-existing resource", async () => {
    const collectionIdBob = await createCollection(
      api,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdBob,
        "nft-metadata"
    );

    const tx = acceptNftResource(api, Alice, collectionIdBob, nftAlice, nonexistentId);
    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });

  it("[Negative test]: unable to accept a resource by a not-an-NFT-owner user", async () => {
    const collectionIdBob = await createCollection(
      api,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdBob,
        "nft-metadata"
    );

    const resourceId = await addNftBasicResource(
      api,
      Bob,
      "pending",
      collectionIdBob,
      nftAlice,
      src,
      metadata,
      license,
      thumb
    );

    const tx = acceptNftResource(api, Bob, collectionIdBob, nftAlice, resourceId);

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("[Negative test]: unable to accept a resource to a non-target NFT", async () => {
    const collectionIdBob = await createCollection(
      api,
      Bob,
      "test-metadata",
      null,
      "test-symbol"
    );

    const nftAlice = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdBob,
        "nft-metadata"
    );

    const wrongNft = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdBob,
        "nft-metadata"
    );
    
    const resourceId = await addNftBasicResource(
      api,
      Bob,
      "pending",
      collectionIdBob,
      nftAlice,
      src,
      metadata,
      license,
      thumb
    );

    const tx = acceptNftResource(api, Bob, collectionIdBob, wrongNft, resourceId);

    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });


  after(() => {
    api.disconnect();
  });
});
