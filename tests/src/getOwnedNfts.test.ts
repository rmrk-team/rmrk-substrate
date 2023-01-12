import { expect } from "chai";
import { getApiConnection } from "./substrate/substrate-api";
import { getOwnedNfts } from "./util/fetch";
import { mintNft, createCollection } from "./util/tx";

describe("integration test: get owned NFTs", () => {
  let api: any;
  before(async () => {
    api = await getApiConnection();
  });

  const alice = "//Alice";

  it("fetch all NFTs owned by a user over multiple collections", async () => {
    const owner = alice;

    const collections = [
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
      }
    ];
    const recipientUri = null;
    const royalty = null;
    const nftMetadata = "alice-NFT-metadata";

    let collectionId1 = await createCollection(
      api,
      collections[0].id,
      alice,
      collections[0].metadata,
      collections[0].collectionMax,
      collections[0].symbol
    );

    let collectionId2 = await createCollection(
      api,
      collections[1].id,
      alice,
      collections[1].metadata,
      collections[1].collectionMax,
      collections[1].symbol
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
    ),
    await mintNft(
      api,
      1,
      alice,
      owner,
      collectionId1,
      nftMetadata + "-1",
      recipientUri,
      royalty
    ),
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

    const ids = [
      {nftId: 0, collectionId: collections[0].id}, 
      {nftId: 1, collectionId: collections[0].id}, 
      {nftId: 0, collectionId: collections[1].id}
    ];

    const ownedNfts = await getOwnedNfts(api, alice, null, null);

    ids.forEach(({nftId, collectionId}) => {
      const nft = ownedNfts.find((ownedNft) => {
        return ownedNft[0].toNumber() === collectionId && ownedNft[1].toNumber() === nftId;
      });

      expect(nft !== undefined, `NFT (${collectionId}, ${nftId}) should be owned by ${alice}`).to.be
        .true;

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
    });
  });

  after(() => {
    api.disconnect();
  });
});
