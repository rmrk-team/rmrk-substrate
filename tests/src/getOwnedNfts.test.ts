import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { getOwnedNfts } from "./util/fetch";
import { mintNft, createCollection } from "./util/tx";

function checkMetadata(nft: any, nftMetadata: string, nftId: number) {
  if(nft) {
    expect(nft[2].transferable.isTrue, `The nft should be transferable`).to.be
      .true;
    expect(nft[2].metadata.toUtf8() === (nftMetadata + `-${nftId}`), `The nft metadata should be correct`).to.be
      .true;
    expect(nft[2].royalty.isNone, `The royalty should be None.`).to.be
      .true;
    expect(nft[2].equipped.isEmpty, `The nft shouldn't be equipped.`).to.be
      .true;
    expect(nft[2].pending.isFalse, `The nft shouldn't be pending.`).to.be
      .true;
  }
}

describe("integration test: get owned NFTs", () => {
  let api: any;
  let collections: any;
  let collectionId1: any;
  let collectionId2: any;
  let collectionId3: any;
  let ids: Array<{nftId: number, collectionId: any}>;

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

    ids = [
      {nftId: 0, collectionId: collections[0].id}, 
      {nftId: 1, collectionId: collections[0].id}, 
      {nftId: 0, collectionId: collections[1].id},
      {nftId: 0, collectionId: collections[2].id}
    ];
  });

  const alice = "//Alice";
  const owner = alice;
  const recipientUri = null;
  const royalty = null;
  const nftMetadata = "alice-NFT-metadata";

  it("fetch all NFTs owned by a user over multiple collections", async () => {
    const ownedNfts = await getOwnedNfts(api, alice, null, null);

    ids.forEach(({nftId, collectionId}) => {
      const nft = ownedNfts.find((ownedNft) => {
        return ownedNft[0].toNumber() === collectionId && ownedNft[1].toNumber() === nftId;
      });

      expect(nft !== undefined, `NFT (${collectionId}, ${nftId}) should be owned by ${owner}`).to.be
        .true;

      checkMetadata(nft, nftMetadata, nftId);
    });
  });

  it("fetch all NFTs owned by a user over multiple collections providing start", async () => {
    // We are skipping the first collection by setting the start to "1". So we
    // should only get the NFTs from the rest of the collections, in this case
    // the NFTs from collection 2 and 3.
    const ownedNfts = await getOwnedNfts(api, alice, "1", null);
    console.log(ownedNfts);
    expect(ownedNfts.length === 2, "Only two NFTs should be returned since we skipped the first collection.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === 2 || nft[0].toNumber() === 3, "The NFTs we received should be from the second or third collection.").to.be
        .true;
    })
  });

  it("fetch all NFTs owned by a user over multiple collections providing count", async () => {
    // We should only get the NFTs from the first two collections since we are
    // setting the count to "2". In this case we are getting two NFTs from the
    // first and one from the second collection.
    const ownedNfts = await getOwnedNfts(api, alice, null, "2");
    console.log(ownedNfts);
    expect(ownedNfts.length === 3, "Three NFTs should be returned from the first and second collection.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === 1 || nft[0].toNumber() === 2, "The NFT we received should be from the first and second collection.").to.be
        .true;
    })
  });

  it("fetch all NFTs owned by a user over multiple collections providing start and count", async () => {
    // We are skipping the first collection by setting the start to "1". But
    // because we are setting the count to "1" we are only going to receive NFTs
    // from one collection, i.e. the collection following the first one, in this
    // case collection number 2.
    const ownedNfts = await getOwnedNfts(api, alice, "1", "1");
    console.log(ownedNfts);
    expect(ownedNfts.length === 1, "Only one NFT should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === 2, "The NFT we received should be from the second collection.").to.be
        .true;
    })
  });

  after(() => {
    api.disconnect();
  });
});
