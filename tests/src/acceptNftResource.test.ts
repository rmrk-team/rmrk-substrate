import { getApiConnection } from "./substrate/substrate-api";
import { expectTxFailure } from "./util/helpers";
import {
  acceptNftResource,
  addNftBasicResource,
  createCollection,
  mintNft,
} from "./util/tx";

describe("Integration test: Accept a top-level NFT resource (different users)", () => {
  const Alice = "//Alice";
  const Bob = "//Bob";
  const src = "test-basic-src";
  const metadata = "test-basic-metadata";
  const license = "test-basic-license";
  const thumb = "test-basic-thumb";

  const nonexistentResourceId = 127;

  let collectionIdBob: number;
  let nftAlice: number;

  let api: any;
  before(async () => {
    api = await getApiConnection();

    collectionIdBob = await createCollection(
        api,
        Bob,
        "test-metadata",
        null,
        "test-symbol"
    );

    nftAlice = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdBob,
        "nft-metadata"
    );
  });

  it("Accept resource", async () => {
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

  it("Negative: unable to accept a non-existing resource", async () => {
    const tx = acceptNftResource(api, Alice, collectionIdBob, nftAlice, nonexistentResourceId);
    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });

  it("Negative: unable to accept a resource by a not-an-NFT-owner user", async () => {
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

  it("Negative: unable to accept a resource to a non-target NFT", async () => {
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
