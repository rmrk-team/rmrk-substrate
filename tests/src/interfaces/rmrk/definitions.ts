import types from '../lookup';

type RpcParam = {
  name: string;
  type: string;
  isOptional?: true;
};

const atParam = {name: 'at', type: 'Hash', isOptional: true};
const fn = (description: string, params: RpcParam[], type: string) => ({
    description,
    params: [...params, atParam],
    type,
});

export default {
    types,
    rpc: {
        lastCollectionIdx: fn('Get the latest created collection id', [], 'u32'),
        collectionById: fn('Get collection by id', [{name: 'id', type: 'u32'}], 'Option<UpDataStructsRmrkCollectionInfo>'),
        nftById: fn(
            'Get NFT by collection id and NFT id',
            [
                {name: 'collectionId', type: 'u32'},
                {name: 'nftId', type: 'u32'},
            ],
            'Option<UpDataStructsRmrkNftInfo>'
        ),
        accountTokens: fn(
            'Get tokens owned by an account in a collection',
            [
                {name: 'accountId', type: 'AccountId32'},
                {name: 'collectionId', type: 'u32'}
            ],
            'Vec<u32>'
        ),
        nftChildren: fn(
            'Get NFT children',
            [
                {name: 'collectionId', type: 'u32'},
                {name: 'nftId', type: 'u32'},
            ],
            'Vec<UpDataStructsRmrkNftChild>'
        ),
        collectionProperties: fn(
            'Get collection properties',
            [{name: 'collectionId', type: 'u32'}],
            'Vec<UpDataStructsRmrkPropertyInfo>'
        ),
        nftProperties: fn(
            'Get NFT properties',
            [
                {name: 'collectionId', type: 'u32'},
                {name: 'nftId', type: 'u32'}
            ],
            'Vec<UpDataStructsRmrkPropertyInfo>'
        ),
        nftResources: fn(
            'Get NFT resources',
            [
                {name: 'collectionId', type: 'u32'},
                {name: 'nftId', type: 'u32'}
            ],
            'Vec<UpDataStructsRmrkResourceInfo>'
        ),
        nftResourcePriorities: fn(
            'Get NFT resource priorities',
            [
                {name: 'collectionId', type: 'u32'},
                {name: 'nftId', type: 'u32'}
            ],
            'Vec<u32>'
        ),
        base: fn(
            'Get base info',
            [{name: 'baseId', type: 'u32'}],
            'Option<UpDataStructsRmrkBaseInfo>'
        ),
        baseParts: fn(
            'Get all Base\'s parts',
            [{name: 'baseId', type: 'u32'}],
            'Vec<UpDataStructsRmrkPartType>'
        ),
        themeNames: fn(
            'Get Base\'s theme names',
            [{name: 'baseId', type: 'u32'}],
            'Vec<Bytes>'
        ),
        themes: fn(
            'Get Theme\'s keys values',
            [
                {name: 'baseId', type: 'u32'},
                {name: 'themeName', type: 'String'},
                {name: 'keys', type: 'Option<Vec<String>>'}
            ],
            'Option<UpDataStructsRmrkTheme>'
        )
    }
};
