// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Result, Vec, bool, u128, u32, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportDispatchDispatchInfo, FrameSupportTokensMiscBalanceStatus, RmrkTraitsNftAccountIdOrCollectionNftTuple, SpFinalityGrandpaAppPublic, SpRuntimeDispatchError } from '@polkadot/types/lookup';

declare module '@polkadot/api-base/types/events' {
  export interface AugmentedEvents<ApiType extends ApiTypes> {
    balances: {
      /**
       * A balance was set by root.
       **/
      BalanceSet: AugmentedEvent<ApiType, [who: AccountId32, free: u128, reserved: u128], { who: AccountId32, free: u128, reserved: u128 }>;
      /**
       * Some amount was deposited (e.g. for transaction fees).
       **/
      Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * An account was removed whose balance was non-zero but below ExistentialDeposit,
       * resulting in an outright loss.
       **/
      DustLost: AugmentedEvent<ApiType, [account: AccountId32, amount: u128], { account: AccountId32, amount: u128 }>;
      /**
       * An account was created with some free balance.
       **/
      Endowed: AugmentedEvent<ApiType, [account: AccountId32, freeBalance: u128], { account: AccountId32, freeBalance: u128 }>;
      /**
       * Some balance was reserved (moved from free to reserved).
       **/
      Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was moved from the reserve of the first account to the second account.
       * Final argument indicates the destination balance type.
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus], { from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus }>;
      /**
       * Some amount was removed from the account (e.g. for misbehavior).
       **/
      Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Transfer succeeded.
       **/
      Transfer: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128], { from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * Some balance was unreserved (moved from reserved to free).
       **/
      Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was withdrawn from the account (e.g. for transaction fees).
       **/
      Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    grandpa: {
      /**
       * New authority set has been applied.
       **/
      NewAuthorities: AugmentedEvent<ApiType, [authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>], { authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>> }>;
      /**
       * Current authority set has been paused.
       **/
      Paused: AugmentedEvent<ApiType, []>;
      /**
       * Current authority set has been resumed.
       **/
      Resumed: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    rmrkCore: {
      CollectionCreated: AugmentedEvent<ApiType, [issuer: AccountId32, collectionId: u32], { issuer: AccountId32, collectionId: u32 }>;
      CollectionDestroyed: AugmentedEvent<ApiType, [issuer: AccountId32, collectionId: u32], { issuer: AccountId32, collectionId: u32 }>;
      CollectionLocked: AugmentedEvent<ApiType, [issuer: AccountId32, collectionId: u32], { issuer: AccountId32, collectionId: u32 }>;
      IssuerChanged: AugmentedEvent<ApiType, [oldIssuer: AccountId32, newIssuer: AccountId32, collectionId: u32], { oldIssuer: AccountId32, newIssuer: AccountId32, collectionId: u32 }>;
      NFTAccepted: AugmentedEvent<ApiType, [sender: AccountId32, recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32], { sender: AccountId32, recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32 }>;
      NFTBurned: AugmentedEvent<ApiType, [owner: AccountId32, collectionId: u32, nftId: u32], { owner: AccountId32, collectionId: u32, nftId: u32 }>;
      NftMinted: AugmentedEvent<ApiType, [owner: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32], { owner: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32 }>;
      NFTRejected: AugmentedEvent<ApiType, [sender: AccountId32, collectionId: u32, nftId: u32], { sender: AccountId32, collectionId: u32, nftId: u32 }>;
      NFTSent: AugmentedEvent<ApiType, [sender: AccountId32, recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32, approvalRequired: bool], { sender: AccountId32, recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32, approvalRequired: bool }>;
      PrioritySet: AugmentedEvent<ApiType, [collectionId: u32, nftId: u32], { collectionId: u32, nftId: u32 }>;
      PropertyRemoved: AugmentedEvent<ApiType, [collectionId: u32, maybeNftId: Option<u32>, key: Bytes], { collectionId: u32, maybeNftId: Option<u32>, key: Bytes }>;
      PropertySet: AugmentedEvent<ApiType, [collectionId: u32, maybeNftId: Option<u32>, key: Bytes, value: Bytes], { collectionId: u32, maybeNftId: Option<u32>, key: Bytes, value: Bytes }>;
      ResourceAccepted: AugmentedEvent<ApiType, [nftId: u32, resourceId: u32, collectionId: u32], { nftId: u32, resourceId: u32, collectionId: u32 }>;
      ResourceAdded: AugmentedEvent<ApiType, [nftId: u32, resourceId: u32, collectionId: u32], { nftId: u32, resourceId: u32, collectionId: u32 }>;
      ResourceRemoval: AugmentedEvent<ApiType, [nftId: u32, resourceId: u32, collectionId: u32], { nftId: u32, resourceId: u32, collectionId: u32 }>;
      ResourceRemovalAccepted: AugmentedEvent<ApiType, [nftId: u32, resourceId: u32, collectionId: u32], { nftId: u32, resourceId: u32, collectionId: u32 }>;
      ResourceReplaced: AugmentedEvent<ApiType, [nftId: u32, resourceId: u32, collectionId: u32], { nftId: u32, resourceId: u32, collectionId: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    rmrkEquip: {
      BaseCreated: AugmentedEvent<ApiType, [issuer: AccountId32, baseId: u32], { issuer: AccountId32, baseId: u32 }>;
      BaseIssuerChanged: AugmentedEvent<ApiType, [oldIssuer: AccountId32, newIssuer: AccountId32, baseId: u32], { oldIssuer: AccountId32, newIssuer: AccountId32, baseId: u32 }>;
      EquippablesUpdated: AugmentedEvent<ApiType, [baseId: u32, slotId: u32], { baseId: u32, slotId: u32 }>;
      SlotEquipped: AugmentedEvent<ApiType, [itemCollection: u32, itemNft: u32, baseId: u32, slotId: u32], { itemCollection: u32, itemNft: u32, baseId: u32, slotId: u32 }>;
      SlotUnequipped: AugmentedEvent<ApiType, [itemCollection: u32, itemNft: u32, baseId: u32, slotId: u32], { itemCollection: u32, itemNft: u32, baseId: u32, slotId: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    rmrkMarket: {
      /**
       * Offer was accepted
       **/
      OfferAccepted: AugmentedEvent<ApiType, [owner: AccountId32, buyer: AccountId32, collectionId: u32, nftId: u32], { owner: AccountId32, buyer: AccountId32, collectionId: u32, nftId: u32 }>;
      /**
       * Offer was placed on a token
       **/
      OfferPlaced: AugmentedEvent<ApiType, [offerer: AccountId32, collectionId: u32, nftId: u32, price: u128], { offerer: AccountId32, collectionId: u32, nftId: u32, price: u128 }>;
      /**
       * Offer was withdrawn
       **/
      OfferWithdrawn: AugmentedEvent<ApiType, [sender: AccountId32, collectionId: u32, nftId: u32], { sender: AccountId32, collectionId: u32, nftId: u32 }>;
      /**
       * Token listed on Marketplace
       **/
      TokenListed: AugmentedEvent<ApiType, [owner: AccountId32, collectionId: u32, nftId: u32, price: u128], { owner: AccountId32, collectionId: u32, nftId: u32, price: u128 }>;
      /**
       * The price for a token was updated
       **/
      TokenPriceUpdated: AugmentedEvent<ApiType, [owner: AccountId32, collectionId: u32, nftId: u32, price: Option<u128>], { owner: AccountId32, collectionId: u32, nftId: u32, price: Option<u128> }>;
      /**
       * Token was sold to a new owner
       **/
      TokenSold: AugmentedEvent<ApiType, [owner: AccountId32, buyer: AccountId32, collectionId: u32, nftId: u32, price: u128], { owner: AccountId32, buyer: AccountId32, collectionId: u32, nftId: u32, price: u128 }>;
      /**
       * Token unlisted on Marketplace
       **/
      TokenUnlisted: AugmentedEvent<ApiType, [owner: AccountId32, collectionId: u32, nftId: u32], { owner: AccountId32, collectionId: u32, nftId: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /**
       * The \[sudoer\] just switched identity; the old key is supplied if one existed.
       **/
      KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      Sudid: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      SudoAsDone: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /**
       * `:code` was updated.
       **/
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /**
       * An extrinsic failed.
       **/
      ExtrinsicFailed: AugmentedEvent<ApiType, [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo }>;
      /**
       * An extrinsic completed successfully.
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchInfo: FrameSupportDispatchDispatchInfo }>;
      /**
       * An account was reaped.
       **/
      KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * A new account was created.
       **/
      NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * On on-chain remark happened.
       **/
      Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32, hash_: H256 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    templateModule: {
      /**
       * Event documentation should end with an array that provides descriptive names for event
       * parameters. [something, who]
       **/
      SomethingStored: AugmentedEvent<ApiType, [u32, AccountId32]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    transactionPayment: {
      /**
       * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
       * has been paid by `who`.
       **/
      TransactionFeePaid: AugmentedEvent<ApiType, [who: AccountId32, actualFee: u128, tip: u128], { who: AccountId32, actualFee: u128, tip: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    uniques: {
      /**
       * An approval for a `delegate` account to transfer the `item` of an item
       * `collection` was cancelled by its `owner`.
       **/
      ApprovalCancelled: AugmentedEvent<ApiType, [collection: u32, item: u32, owner: AccountId32, delegate: AccountId32], { collection: u32, item: u32, owner: AccountId32, delegate: AccountId32 }>;
      /**
       * An `item` of a `collection` has been approved by the `owner` for transfer by
       * a `delegate`.
       **/
      ApprovedTransfer: AugmentedEvent<ApiType, [collection: u32, item: u32, owner: AccountId32, delegate: AccountId32], { collection: u32, item: u32, owner: AccountId32, delegate: AccountId32 }>;
      /**
       * Attribute metadata has been cleared for a `collection` or `item`.
       **/
      AttributeCleared: AugmentedEvent<ApiType, [collection: u32, maybeItem: Option<u32>, key: Bytes], { collection: u32, maybeItem: Option<u32>, key: Bytes }>;
      /**
       * New attribute metadata has been set for a `collection` or `item`.
       **/
      AttributeSet: AugmentedEvent<ApiType, [collection: u32, maybeItem: Option<u32>, key: Bytes, value: Bytes], { collection: u32, maybeItem: Option<u32>, key: Bytes, value: Bytes }>;
      /**
       * An `item` was destroyed.
       **/
      Burned: AugmentedEvent<ApiType, [collection: u32, item: u32, owner: AccountId32], { collection: u32, item: u32, owner: AccountId32 }>;
      /**
       * Some `collection` was frozen.
       **/
      CollectionFrozen: AugmentedEvent<ApiType, [collection: u32], { collection: u32 }>;
      /**
       * Max supply has been set for a collection.
       **/
      CollectionMaxSupplySet: AugmentedEvent<ApiType, [collection: u32, maxSupply: u32], { collection: u32, maxSupply: u32 }>;
      /**
       * Metadata has been cleared for a `collection`.
       **/
      CollectionMetadataCleared: AugmentedEvent<ApiType, [collection: u32], { collection: u32 }>;
      /**
       * New metadata has been set for a `collection`.
       **/
      CollectionMetadataSet: AugmentedEvent<ApiType, [collection: u32, data: Bytes, isFrozen: bool], { collection: u32, data: Bytes, isFrozen: bool }>;
      /**
       * Some `collection` was thawed.
       **/
      CollectionThawed: AugmentedEvent<ApiType, [collection: u32], { collection: u32 }>;
      /**
       * A `collection` was created.
       **/
      Created: AugmentedEvent<ApiType, [collection: u32, creator: AccountId32, owner: AccountId32], { collection: u32, creator: AccountId32, owner: AccountId32 }>;
      /**
       * A `collection` was destroyed.
       **/
      Destroyed: AugmentedEvent<ApiType, [collection: u32], { collection: u32 }>;
      /**
       * A `collection` was force-created.
       **/
      ForceCreated: AugmentedEvent<ApiType, [collection: u32, owner: AccountId32], { collection: u32, owner: AccountId32 }>;
      /**
       * Some `item` was frozen.
       **/
      Frozen: AugmentedEvent<ApiType, [collection: u32, item: u32], { collection: u32, item: u32 }>;
      /**
       * An `item` was issued.
       **/
      Issued: AugmentedEvent<ApiType, [collection: u32, item: u32, owner: AccountId32], { collection: u32, item: u32, owner: AccountId32 }>;
      /**
       * An item was bought.
       **/
      ItemBought: AugmentedEvent<ApiType, [collection: u32, item: u32, price: u128, seller: AccountId32, buyer: AccountId32], { collection: u32, item: u32, price: u128, seller: AccountId32, buyer: AccountId32 }>;
      /**
       * The price for the instance was removed.
       **/
      ItemPriceRemoved: AugmentedEvent<ApiType, [collection: u32, item: u32], { collection: u32, item: u32 }>;
      /**
       * The price was set for the instance.
       **/
      ItemPriceSet: AugmentedEvent<ApiType, [collection: u32, item: u32, price: u128, whitelistedBuyer: Option<AccountId32>], { collection: u32, item: u32, price: u128, whitelistedBuyer: Option<AccountId32> }>;
      /**
       * A `collection` has had its attributes changed by the `Force` origin.
       **/
      ItemStatusChanged: AugmentedEvent<ApiType, [collection: u32], { collection: u32 }>;
      /**
       * Metadata has been cleared for an item.
       **/
      MetadataCleared: AugmentedEvent<ApiType, [collection: u32, item: u32], { collection: u32, item: u32 }>;
      /**
       * New metadata has been set for an item.
       **/
      MetadataSet: AugmentedEvent<ApiType, [collection: u32, item: u32, data: Bytes, isFrozen: bool], { collection: u32, item: u32, data: Bytes, isFrozen: bool }>;
      /**
       * The owner changed.
       **/
      OwnerChanged: AugmentedEvent<ApiType, [collection: u32, newOwner: AccountId32], { collection: u32, newOwner: AccountId32 }>;
      /**
       * Ownership acceptance has changed for an account.
       **/
      OwnershipAcceptanceChanged: AugmentedEvent<ApiType, [who: AccountId32, maybeCollection: Option<u32>], { who: AccountId32, maybeCollection: Option<u32> }>;
      /**
       * Metadata has been cleared for an item.
       **/
      Redeposited: AugmentedEvent<ApiType, [collection: u32, successfulItems: Vec<u32>], { collection: u32, successfulItems: Vec<u32> }>;
      /**
       * The management team changed.
       **/
      TeamChanged: AugmentedEvent<ApiType, [collection: u32, issuer: AccountId32, admin: AccountId32, freezer: AccountId32], { collection: u32, issuer: AccountId32, admin: AccountId32, freezer: AccountId32 }>;
      /**
       * Some `item` was thawed.
       **/
      Thawed: AugmentedEvent<ApiType, [collection: u32, item: u32], { collection: u32, item: u32 }>;
      /**
       * An `item` was transferred.
       **/
      Transferred: AugmentedEvent<ApiType, [collection: u32, item: u32, from: AccountId32, to: AccountId32], { collection: u32, item: u32, from: AccountId32, to: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /**
       * Batch of dispatches completed fully with no error.
       **/
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches completed but has errors.
       **/
      BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
       * well as the error.
       **/
      BatchInterrupted: AugmentedEvent<ApiType, [index: u32, error: SpRuntimeDispatchError], { index: u32, error: SpRuntimeDispatchError }>;
      /**
       * A call was dispatched.
       **/
      DispatchedAs: AugmentedEvent<ApiType, [result: Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single item within a Batch of dispatches has completed with no error.
       **/
      ItemCompleted: AugmentedEvent<ApiType, []>;
      /**
       * A single item within a Batch of dispatches has completed with error.
       **/
      ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
