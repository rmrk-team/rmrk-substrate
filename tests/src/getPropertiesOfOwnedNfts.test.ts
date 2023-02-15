import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { getPropertiesOfOwnedNfts } from "./util/fetch";
import { mintNft, createCollection, setNftProperty } from "./util/tx";

describe("integration test: get properties of owned NFTs", () => {
  let api: any;
  let collections: Array<{id: number, metadata: string, collectionMax: any, symbol: string}>;
  let nftProperties: Array<{nftId: number, collectionId: any, key: string, value: string}>;

  before(async () => {
    api = await getApiConnection();

    collections = [
      {
        id: 421,
        metadata: "Metadata#421",
        collectionMax: null,
        symbol: "Sym421",
      },
      {
        id: 422,
        metadata: "Metadata#422",
        collectionMax: null,
        symbol: "Sym422",
      },
      {
        id: 423,
        metadata: "Metadata#423",
        collectionMax: null,
        symbol: "Sym423",
      }
    ];

    nftProperties = [
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

    for(const collection of collections) {
      await createCollection(
        api,
        collection.id,
        dave,
        collection.metadata,
        collection.collectionMax,
        collection.symbol
      );
    }

    for(const nftProps of nftProperties) {
      await mintNft(
        api,
        nftProps.nftId,
        dave,
        owner,
        nftProps.collectionId,
        nftMetadata + `-${nftProps.nftId}`,
        recipientUri,
        royalty
      );

      await setNftProperty(
        api,
        dave,
        nftProps.collectionId,
        nftProps.nftId,
        nftProps.key,
        nftProps.value,
      );
    }
  });

  const dave = "//Dave";
  const owner = dave;
  const recipientUri = null;
  const royalty = null;
  const nftMetadata = "dave-NFT-metadata";

  it("fetch all the properites of the NFTs owned by a user over multiple collections", async () => {
    const ownedNfts = await getPropertiesOfOwnedNfts(api, dave, null, null);

    nftProperties.forEach(({nftId, collectionId, key, value}) => {
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
    // We are skipping the first collection by setting the start index to "1".
    // So we should only get the properties of the NFTs from the collection 192
    // and 193.
    const ownedNfts = await getPropertiesOfOwnedNfts(api, dave, "1", null);

    expect(ownedNfts.length === 2, "Two NFTs should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[1].id || nft[0].toNumber() === collections[2].id,
       "The returned NFTs should be from collection 192 and 193.").to.be.true;
    });
  });

  it("fetch all the properites of the NFTs owned by a user over multiple collections with specified count", async () => {
    // We should only get the properties from the NFTs in collection 191 and 192.
    const ownedNfts = await getPropertiesOfOwnedNfts(api, dave, null, "2");

    expect(ownedNfts.length === 3, "Three NFTs should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[0].id || nft[0].toNumber() === collections[1].id, 
        "The returned NFTs shouldn't be from collection 193.").to.be.true;
    });
  });

  it("fetch all the properites of the NFTs owned by a user over multiple collections with specified start and count", async () => {
    // We are skipping the first collection by setting the start index to "1".
    // But because we are setting the count to "1" we are only going to receive
    // the properties from NFTs inside one collection, i.e. the collection
    // following the first one, in this case collection number 422.
    const ownedNfts = await getPropertiesOfOwnedNfts(api, dave, "1", "1");

    expect(ownedNfts.length === 1, "Only one NFT should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[1].id, "The returned NFTs should be from collection 192").to.be
        .true;
    });
  });

  after(() => {
    api.disconnect();
  });
});
