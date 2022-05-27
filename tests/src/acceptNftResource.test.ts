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

  const nonexistentResourceId = "nexistepas";

  let collectionIdAlice: number;
  let nftAlice: number;

  let api: any;
  before(async () => {
    api = await getApiConnection();

    collectionIdAlice = await createCollection(
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
        collectionIdAlice,
        "nft-metadata"
    );
  });

  it("Accept resource", async () => {
    const resourceId = "resid0";
    
    await addNftBasicResource(
      api,
      Bob,
      "pending",
      collectionIdAlice,
      nftAlice,
      resourceId,
      src,
      metadata,
      license,
      thumb
    );

    await acceptNftResource(api, Alice, collectionIdAlice, nftAlice, resourceId);
  });

  it("Negative: unable to accept a non-existing resource", async () => {
    const tx = acceptNftResource(api, Alice, collectionIdAlice, nftAlice, nonexistentResourceId);
    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });

  it("Negative: unable to accept a resource by a not-an-NFT-owner user", async () => {
    const resourceId = "resid1";
    
    await addNftBasicResource(
      api,
      Bob,
      "pending",
      collectionIdAlice,
      nftAlice,
      resourceId,
      src,
      metadata,
      license,
      thumb
    );

    const tx = acceptNftResource(api, Bob, collectionIdAlice, nftAlice, resourceId);

    await expectTxFailure(/rmrkCore\.NoPermission/, tx);
  });

  it("Negative: unable to accept a resource to a non-target NFT", async () => {
    const resourceId = "resid2";

    const wrongNft = await mintNft(
        api,
        Bob,
        Alice,
        collectionIdAlice,
        "nft-metadata"
    );
    
    await addNftBasicResource(
      api,
      Bob,
      "pending",
      collectionIdAlice,
      nftAlice,
      resourceId,
      src,
      metadata,
      license,
      thumb
    );

    const tx = acceptNftResource(api, Bob, collectionIdAlice, wrongNft, resourceId);

    await expectTxFailure(/rmrkCore\.ResourceDoesntExist/, tx);
  });

  after(() => {
    api.disconnect();
  });
});
