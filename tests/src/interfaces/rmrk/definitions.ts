import types from "../lookup";

type RpcParam = {
  name: string;
  type: string;
  isOptional?: true;
};

const atParam = { name: "at", type: "Hash", isOptional: true };
const fn = (description: string, params: RpcParam[], type: string) => ({
  description,
  params: [...params, atParam],
  type,
});

export default {
  types,
  rpc: {
    // lastCollectionIdx: fn('Get the latest created collection id', [], 'u32'),
    collectionById: fn(
      "Get collection by id",
      [{ name: "id", type: "u32" }],
      "Option<RmrkTraitsCollectionCollectionInfo>"
    ),
    nftById: fn(
      "Get NFT by collection id and NFT id",
      [
        { name: "collectionId", type: "u32" },
        { name: "nftId", type: "u32" },
      ],
      "Option<RmrkTraitsNftNftInfo>"
    ),
    nftsOwnedBy: fn(
      "Get all the nfts owned by a user",
      [
        { name: "accountId", type: "AccountId32" },
        { name: "startIndex", type: "Option<u32>" },
        { name: "count", type: "Option<u32>" },
      ],
      "Vec<(u32, u32, RmrkTraitsNftNftInfo)>"
    ),
    propertiesOfNftsOwnedBy: fn(
      "Get the properties of all the nfts owned by a user",
      [
        { name: "accountId", type: "AccountId32" },
        { name: "startIndex", type: "Option<u32>" },
        { name: "count", type: "Option<u32>" },
      ],
      "Vec<(u32, u32, Vec<RmrkTraitsPropertyPropertyInfo>)>"
    ),
    accountTokens: fn(
      "Get tokens owned by an account in a collection",
      [
        { name: "accountId", type: "AccountId32" },
        { name: "collectionId", type: "u32" },
      ],
      "Vec<u32>"
    ),
    nftChildren: fn(
      "Get NFT children",
      [
        { name: "collectionId", type: "u32" },
        { name: "nftId", type: "u32" },
      ],
      "Vec<RmrkTraitsNftNftChild>"
    ),
    collectionProperties: fn(
      "Get collection properties",
      [{ name: "collectionId", type: "u32" }],
      "Vec<RmrkTraitsPropertyPropertyInfo>"
    ),
    nftProperties: fn(
      "Get NFT properties",
      [
        { name: "collectionId", type: "u32" },
        { name: "nftId", type: "u32" },
      ],
      "Vec<RmrkTraitsPropertyPropertyInfo>"
    ),
    nftResources: fn(
      "Get NFT resources",
      [
        { name: "collectionId", type: "u32" },
        { name: "nftId", type: "u32" },
      ],
      "Vec<RmrkTraitsResourceResourceInfo>"
    ),
    nftResourcePriority: fn(
      "Get NFT resource priority",
      [
        { name: "collectionId", type: "u32" },
        { name: "nftId", type: "u32" },
        { name: "resourceId", type: "u32" },
      ],
      "Option<u32>"
    ),
    base: fn(
      "Get base info",
      [{ name: "baseId", type: "u32" }],
      "Option<RmrkTraitsBaseBaseInfo>"
    ),
    baseParts: fn(
      "Get all Base's parts",
      [{ name: "baseId", type: "u32" }],
      "Vec<RmrkTraitsPartPartType>"
    ),
    themeNames: fn(
      "Get Base's theme names",
      [{ name: "baseId", type: "u32" }],
      "Vec<Bytes>"
    ),
    themes: fn(
      "Get Theme info -- name, properties, and inherit flag",
      [
        { name: "baseId", type: "u32" },
        { name: "themeName", type: "String" },
        { name: "keys", type: "Option<Vec<String>>" },
      ],
      "Option<RmrkTraitsTheme>"
    ),
  },
};
