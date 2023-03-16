import { ApiPromise } from "@polkadot/api";
import { Option, Vec, u32 } from "@polkadot/types-codec";
import { ITuple } from "@polkadot/types-codec/types";
import type {
  RmrkTraitsCollectionCollectionInfo as Collection,
  RmrkTraitsNftNftInfo as Nft,
  RmrkTraitsResourceResourceInfo as Resource,
  RmrkTraitsBaseBaseInfo as Base,
  RmrkTraitsPartPartType as PartType,
  RmrkTraitsNftNftChild as NftChild,
  RmrkTraitsTheme as Theme,
  RmrkTraitsPropertyPropertyInfo as Property,
} from "../interfaces/rmrk/types"; // '@polkadot/types/lookup';
import "../interfaces/augment-api";
import "../interfaces/augment-api-query";
import privateKey from "../substrate/privateKey";

export type NftIdTuple = [number, number];

export async function getCollection(
  api: ApiPromise,
  id: number
): Promise<Option<Collection>> {
  return api.rpc.rmrk.collectionById(id);
}

export async function getOwnedNftsInCollection(
  api: ApiPromise,
  ownerUri: string,
  collectionId: number
): Promise<number[]> {
  const ss58Format = api.registry.getChainProperties()!.toJSON().ss58Format;
  const owner = privateKey(ownerUri, Number(ss58Format));

  return (await api.rpc.rmrk.accountTokens(owner.address, collectionId)).map(
    (value) => value.toNumber()
  );
}

export async function getNft(
  api: ApiPromise,
  collectionId: number,
  nftId: number
): Promise<Option<Nft>> {
  return api.rpc.rmrk.nftById(collectionId, nftId);
}

export async function getOwnedNfts(
  api: ApiPromise,
  ownerUri: string,
  startIndex: string | null,
  count: string | null,
): Promise<Vec<ITuple<[u32, u32, Nft]>>> {
  const ss58Format = api.registry.getChainProperties()!.toJSON().ss58Format;
  const owner = privateKey(ownerUri, Number(ss58Format));

  return api.rpc.rmrk.nftsOwnedBy(owner.address, startIndex, count);
}

export async function getPropertiesOfOwnedNfts(
  api: ApiPromise,
  ownerUri: string,
  startIndex: string | null = null,
  count: string | null = null,
): Promise<Vec<ITuple<[u32, u32, Vec<Property>]>>> {
  const ss58Format = api.registry.getChainProperties()!.toJSON().ss58Format;
  const owner = privateKey(ownerUri, Number(ss58Format));

  return api.rpc.rmrk.propertiesOfNftsOwnedBy(owner.address, startIndex, count);
}

export async function getCollectionProperties(
  api: ApiPromise,
  collectionId: number
): Promise<Property[]> {
  return (await api.rpc.rmrk.collectionProperties(collectionId)).toArray();
}

export async function getNftProperties(
  api: ApiPromise,
  collectionId: number,
  nftId: number
): Promise<Property[]> {
  return (await api.rpc.rmrk.nftProperties(collectionId, nftId)).toArray();
}

export async function getChildren(
  api: ApiPromise,
  collectionId: number,
  nftId: number
): Promise<NftChild[]> {
  return (await api.rpc.rmrk.nftChildren(collectionId, nftId)).toArray();
}

export async function getBase(
  api: ApiPromise,
  baseId: number
): Promise<Option<Base>> {
  return api.rpc.rmrk.base(baseId);
}

export async function getParts(
  api: ApiPromise,
  baseId: number
): Promise<PartType[]> {
  return (await api.rpc.rmrk.baseParts(baseId)).toArray();
}

export async function getEquippableList(
  api: ApiPromise,
  baseId: number,
  slotId: number
): Promise<"All" | "Empty" | { Custom: number[] } | null> {
  const parts = await getParts(api, baseId);

  const part = parts.find((part) => {
    if (part.isSlotPart) {
      return part.asSlotPart.id.toNumber() === slotId;
    } else {
      return false;
    }
  });

  if (part) {
    const slot = part.asSlotPart;
    if (slot.equippable.isCustom) {
      return {
        Custom: slot.equippable.asCustom
          .toArray()
          .map((collectionId) => collectionId.toNumber()),
      };
    } else if (slot.equippable.isAll) {
      return "All";
    } else {
      return "Empty";
    }
  } else {
    return null;
  }
}

export async function getResourcePriority(
  api: ApiPromise,
  collectionId: number,
  nftId: number,
  resourceId: number
): Promise<number> {
  return (
    await api.rpc.rmrk.nftResourcePriority(collectionId, nftId, resourceId)
  )
    .unwrap()
    .toNumber();
}

export async function getThemeNames(
  api: ApiPromise,
  baseId: number
): Promise<string[]> {
  return (await api.rpc.rmrk.themeNames(baseId)).map((name) => name.toUtf8());
}

export async function getTheme(
  api: ApiPromise,
  baseId: number,
  themeName: string,
  keys: string[] | null = null
): Promise<Option<Theme>> {
  return api.rpc.rmrk.themes(baseId, themeName, keys);
}

export async function getResources(
  api: ApiPromise,
  collectionId: number,
  nftId: number
): Promise<Resource[]> {
  return (await api.rpc.rmrk.nftResources(collectionId, nftId)).toArray();
}
