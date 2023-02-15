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
  let collections: Array<{id: number, metadata: string, collectionMax: any, symbol: string}>;
  let nfts: Array<{nftId: number, collectionId: any}>;

  const eve = "//Eve";
  const owner = eve;
  const recipientUri = null;
  const royalty = null;
  const nftMetadata = "eve-NFT-metadata";

  before(async () => {
    api = await getApiConnection();

    collections = [
      {
        id: 291,
        metadata: "Metadata#291",
        collectionMax: null,
        symbol: "Sym291",
      },
      {
        id: 292,
        metadata: "Metadata#292",
        collectionMax: null,
        symbol: "Sym292",
      },
      {
        id: 293,
        metadata: "Metadata#293",
        collectionMax: null,
        symbol: "Sym293",
      }
    ];
    nfts = [
      {nftId: 0, collectionId: collections[0].id}, 
      {nftId: 1, collectionId: collections[0].id}, 
      {nftId: 0, collectionId: collections[1].id},
      {nftId: 0, collectionId: collections[2].id}
    ];

    for(const collection of collections) {
      await createCollection(
        api,
        collection.id,
        eve,
        collection.metadata,
        collection.collectionMax,
        collection.symbol
      );
    }

    for(const nft of nfts) {
      await mintNft(
        api,
        nft.nftId,
        eve,
        owner,
        nft.collectionId,
        nftMetadata + `-${nft.nftId}`,
        recipientUri,
        royalty
      );
    }
  });

  it("fetch all NFTs owned by a user over multiple collections", async () => {
    const ownedNfts = await getOwnedNfts(api, eve, null, null);

    nfts.forEach(({nftId, collectionId}) => {
      const nft = ownedNfts.find((ownedNft) => {
        return ownedNft[0].toNumber() === collectionId && ownedNft[1].toNumber() === nftId;
      });

      expect(nft !== undefined, `NFT (${collectionId}, ${nftId}) should be owned by ${owner}`).to.be
        .true;

      checkMetadata(nft, nftMetadata, nftId);
    });
  });

  it("fetch all NFTs owned by a user over multiple collections providing start", async () => {
    // We are skipping the first collection by setting the start index to "1". So the
    // collection we are skipping here is 291.
    const ownedNfts = await getOwnedNfts(api, eve, "1", null);
    expect(ownedNfts.length === 2, "Two NFTs should be returned since we skipped the first collection.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[1].id || nft[0].toNumber() === collections[2].id, 
        "The NFTs we received should be from collection 292 and 293.").to.be.true;
    })
  });

  it("fetch all NFTs owned by a user over multiple collections providing count", async () => {
    // We should get the NFTs from collection 291 and 292.
    const ownedNfts = await getOwnedNfts(api, eve, null, "2");
    expect(ownedNfts.length === 3, "Three NFTs should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[0].id || nft[0].toNumber() === collections[1].id, 
        "The NFT we received should be from collection 291 and 292.").to.be.true;
    })
  });

  it("fetch all NFTs owned by a user over multiple collections providing start and count", async () => {
    // We are skipping the first collection by setting the start index to "1". But
    // because we are setting the count to "1" we are only going to receive NFTs
    // from one collection.
    const ownedNfts = await getOwnedNfts(api, eve, "1", "1");
    expect(ownedNfts.length === 1, "Only one NFT should be returned.").to.be
      .true;

    ownedNfts.forEach((nft) => {
      expect(nft[0].toNumber() === collections[1].id, "The NFT we received should be from collection 292.").to.be
        .true;
    })
  });

  after(() => {
    api.disconnect();
  });
});
