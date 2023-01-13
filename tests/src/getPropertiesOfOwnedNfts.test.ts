import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { getPropertiesOfOwnedNfts } from "./util/fetch";
import { mintNft, createCollection, setNftProperty } from "./util/tx";

describe("integration test: get properties of owned NFTs", () => {
  let api: any;
  let collections: any;
  let collectionId1: any;
  let collectionId2: any;
  let collectionId3: any;
  let properties: Array<{nftId: number, collectionId: any, key: string, value: string}>;

  before(async () => {
    api = await getApiConnection();
    collections = [
      {
        id: 1,
        metadata: "Metadata#1",
        collectionMax: null,
        symbol: "Col1Sym",
      },
      {
        id: 2,
        metadata: "Metadata#2",
        collectionMax: null,
        symbol: "Col2Sym",
      },
      {
        id: 3,
        metadata: "Metadata#3",
        collectionMax: null,
        symbol: "Col3Sym",
      }
    ];

    properties = [
      {
        nftId: 0, 
        collectionId: collections[0].id,
        key: `nft-${collections[0].id}-0-test-key`,
        value: `nft-${collections[0].id}-0-test-key-value`,
      }, 
      {
        nftId: 1, 
        collectionId: collections[0].id,
        key: `nft-${collections[0].id}-1-test-key`,
        value: `nft-${collections[0].id}-1-test-key-value`,
      }, 
      {
        nftId: 0, 
        collectionId: collections[1].id,
        key: `nft-${collections[1].id}-0-test-key`,
        value: `nft-${collections[1].id}-0-test-key-value`,
      },
      {
        nftId: 0, 
        collectionId: collections[2].id,
        key: `nft-${collections[2].id}-0-test-key`,
        value: `nft-${collections[2].id}-0-test-key-value`,
      }
    ];

    collectionId1 = await createCollection(
      api,
      collections[0].id,
      alice,
      collections[0].metadata,
      collections[0].collectionMax,
      collections[0].symbol
    );

    collectionId2 = await createCollection(
      api,
      collections[1].id,
      alice,
      collections[1].metadata,
      collections[1].collectionMax,
      collections[1].symbol
    );

    collectionId3 = await createCollection(
      api,
      collections[2].id,
      alice,
      collections[2].metadata,
      collections[2].collectionMax,
      collections[2].symbol
    );

    await mintNft(
      api,
      0,
      alice,
      owner,
      collectionId1,
      nftMetadata + "-0",
      recipientUri,
      royalty
    );
    await mintNft(
      api,
      1,
      alice,
      owner,
      collectionId1,
      nftMetadata + "-1",
      recipientUri,
      royalty
    );
    await mintNft(
      api,
      0,
      alice,
      owner,
      collectionId2,
      nftMetadata + "-0",
      recipientUri,
      royalty
    );
    await mintNft(
      api,
      0,
      alice,
      owner,
      collectionId3,
      nftMetadata + "-0",
      recipientUri,
      royalty
    );

    await setNftProperty(
      api,
      alice,
      collectionId1,
      0,
      properties[0].key,
      properties[0].value,
    );
    await setNftProperty(
      api,
      alice,
      collectionId1,
      1,
      properties[1].key,
      properties[1].value,
    );
    await setNftProperty(
      api,
      alice,
      collectionId2,
      0,
      properties[2].key,
      properties[2].value,
    );
    await setNftProperty(
      api,
      alice,
      collectionId3,
      0,
      properties[3].key,
      properties[3].value,
    );
  });

  const alice = "//Alice";
  const owner = alice;
  const recipientUri = null;
  const royalty = null;
  const nftMetadata = "alice-NFT-metadata";

  it("fetch all the properites of the NFTs owned by a user over multiple collections", async () => {
    const ownedNfts = await getPropertiesOfOwnedNfts(api, alice, null, null);

    properties.forEach(({nftId, collectionId, key, value}) => {
      const nft = ownedNfts.find((ownedNft) => {
        return ownedNft[0].toNumber() === collectionId && ownedNft[1].toNumber() === nftId;
      });

      expect(nft !== undefined, `NFT (${collectionId}, ${nftId}) should be owned by ${owner}`).to.be
        .true;
      if(nft) {
        const actualProperties = nft[2];
        actualProperties.forEach((property) => {
          expect(property.key.toUtf8() === key, `The key for (${collectionId}, ${nftId}) is incorrect.`).to.be.true;
          expect(property.value.toUtf8() === value, `The value for (${collectionId}, ${nftId}) is incorrect.`).to.be.true;
        });
      }
    });
  });

  it("fetch all the properites of the NFTs owned by a user over multiple collections with specified start", async () => {
    // We are skipping the first collection by setting the start to "1". So we
    // should only get the properties from the NFTs from the rest of the
    // collections, in this case the NFTs from collection 2 and 3.
    const ownedNfts = await getPropertiesOfOwnedNfts(api, alice, "1", null);

    expect(ownedNfts.length === 2, "Only two NFTs should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() !== collections[0].id, "The returned NFTs shouldn't be from the first collection.").to.be
        .true;
    });
  });

  it("fetch all the properites of the NFTs owned by a user over multiple collections with specified count", async () => {
    // We should only get the properties from the NFTs in the first two
    // collections since we are setting the count to "2".
    const ownedNfts = await getPropertiesOfOwnedNfts(api, alice, null, "2");

    expect(ownedNfts.length === 3, "Only three NFTs should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() !== collections[2].id, "The returned NFTs shouldn't be from the third collection.").to.be
        .true;
    });
  });

  it("fetch all the properites of the NFTs owned by a user over multiple collections with specified start and count", async () => {
    // We are skipping the first collection by setting the start to "1". But
    // because we are setting the count to "1" we are only going to receive the
    // properties from NFTs inside one collection, i.e. the collection following
    // the first one, in this case collection number 2.
    const ownedNfts = await getPropertiesOfOwnedNfts(api, alice, "1", "1");

    expect(ownedNfts.length === 1, "Only one NFT should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[1].id, "The returned NFTs should be from the second collection.").to.be
        .true;
    });
  });

  after(() => {
    api.disconnect();
  });
});
