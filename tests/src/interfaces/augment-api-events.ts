// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Result, Vec, bool, u128, u32, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportTokensMiscBalanceStatus, FrameSupportWeightsDispatchInfo, RmrkTraitsNftAccountIdOrCollectionNftTuple, SpFinalityGrandpaAppPublic, SpRuntimeDispatchError } from '@polkadot/types/lookup';

declare module '@polkadot/api-base/types/events' {
  export interface AugmentedEvents<ApiType extends ApiTypes> {
    balances: {
      /**
       * A balance was set by root.
       **/
      BalanceSet: AugmentedEvent<ApiType, [AccountId32, u128, u128], { who: AccountId32, free: u128, reserved: u128 }>;
      /**
       * Some amount was deposited (e.g. for transaction fees).
       **/
      Deposit: AugmentedEvent<ApiType, [AccountId32, u128], { who: AccountId32, amount: u128 }>;
      /**
       * An account was removed whose balance was non-zero but below ExistentialDeposit,
       * resulting in an outright loss.
       **/
      DustLost: AugmentedEvent<ApiType, [AccountId32, u128], { account: AccountId32, amount: u128 }>;
      /**
       * An account was created with some free balance.
       **/
      Endowed: AugmentedEvent<ApiType, [AccountId32, u128], { account: AccountId32, freeBalance: u128 }>;
      /**
       * Some balance was reserved (moved from free to reserved).
       **/
      Reserved: AugmentedEvent<ApiType, [AccountId32, u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was moved from the reserve of the first account to the second account.
       * Final argument indicates the destination balance type.
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128, FrameSupportTokensMiscBalanceStatus], { from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus }>;
      /**
       * Some amount was removed from the account (e.g. for misbehavior).
       **/
      Slashed: AugmentedEvent<ApiType, [AccountId32, u128], { who: AccountId32, amount: u128 }>;
      /**
       * Transfer succeeded.
       **/
      Transfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128], { from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * Some balance was unreserved (moved from reserved to free).
       **/
      Unreserved: AugmentedEvent<ApiType, [AccountId32, u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was withdrawn from the account (e.g. for transaction fees).
       **/
      Withdraw: AugmentedEvent<ApiType, [AccountId32, u128], { who: AccountId32, amount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    grandpa: {
      /**
       * New authority set has been applied.
       **/
      NewAuthorities: AugmentedEvent<ApiType, [Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>], { authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>> }>;
      /**
       * Current authority set has been paused.
       **/
      Paused: AugmentedEvent<ApiType, [], {  }>;
      /**
       * Current authority set has been resumed.
       **/
      Resumed: AugmentedEvent<ApiType, [], {  }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    rmrkCore: {
      CollectionCreated: AugmentedEvent<ApiType, [AccountId32, u32], { issuer: AccountId32, collectionId: u32 }>;
      CollectionDestroyed: AugmentedEvent<ApiType, [AccountId32, u32], { issuer: AccountId32, collectionId: u32 }>;
      CollectionLocked: AugmentedEvent<ApiType, [AccountId32, u32], { issuer: AccountId32, collectionId: u32 }>;
      IssuerChanged: AugmentedEvent<ApiType, [AccountId32, AccountId32, u32], { oldIssuer: AccountId32, newIssuer: AccountId32, collectionId: u32 }>;
      NFTAccepted: AugmentedEvent<ApiType, [AccountId32, RmrkTraitsNftAccountIdOrCollectionNftTuple, u32, u32], { sender: AccountId32, recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32 }>;
      NFTBurned: AugmentedEvent<ApiType, [AccountId32, u32], { owner: AccountId32, nftId: u32 }>;
      NftMinted: AugmentedEvent<ApiType, [AccountId32, u32, u32], { owner: AccountId32, collectionId: u32, nftId: u32 }>;
      NFTRejected: AugmentedEvent<ApiType, [AccountId32, u32, u32], { sender: AccountId32, collectionId: u32, nftId: u32 }>;
      NFTSent: AugmentedEvent<ApiType, [AccountId32, RmrkTraitsNftAccountIdOrCollectionNftTuple, u32, u32, bool], { sender: AccountId32, recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple, collectionId: u32, nftId: u32, approvalRequired: bool }>;
      PrioritySet: AugmentedEvent<ApiType, [u32, u32], { collectionId: u32, nftId: u32 }>;
      PropertySet: AugmentedEvent<ApiType, [u32, Option<u32>, Bytes, Bytes], { collectionId: u32, maybeNftId: Option<u32>, key: Bytes, value: Bytes }>;
      ResourceAccepted: AugmentedEvent<ApiType, [u32, u32], { nftId: u32, resourceId: u32 }>;
      ResourceAdded: AugmentedEvent<ApiType, [u32, u32], { nftId: u32, resourceId: u32 }>;
      ResourceRemoval: AugmentedEvent<ApiType, [u32, u32], { nftId: u32, resourceId: u32 }>;
      ResourceRemovalAccepted: AugmentedEvent<ApiType, [u32, u32], { nftId: u32, resourceId: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    rmrkEquip: {
      BaseCreated: AugmentedEvent<ApiType, [AccountId32, u32], { issuer: AccountId32, baseId: u32 }>;
      BaseIssuerChanged: AugmentedEvent<ApiType, [AccountId32, AccountId32, u32], { oldIssuer: AccountId32, newIssuer: AccountId32, baseId: u32 }>;
      EquippablesUpdated: AugmentedEvent<ApiType, [u32, u32], { baseId: u32, slotId: u32 }>;
      SlotEquipped: AugmentedEvent<ApiType, [u32, u32, u32, u32], { itemCollection: u32, itemNft: u32, baseId: u32, slotId: u32 }>;
      SlotUnequipped: AugmentedEvent<ApiType, [u32, u32, u32, u32], { itemCollection: u32, itemNft: u32, baseId: u32, slotId: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    rmrkMarket: {
      /**
       * Offer was accepted
       **/
      OfferAccepted: AugmentedEvent<ApiType, [AccountId32, AccountId32, u32, u32], { owner: AccountId32, buyer: AccountId32, collectionId: u32, nftId: u32 }>;
      /**
       * Offer was placed on a token
       **/
      OfferPlaced: AugmentedEvent<ApiType, [AccountId32, u32, u32, u128], { offerer: AccountId32, collectionId: u32, nftId: u32, price: u128 }>;
      /**
       * Offer was withdrawn
       **/
      OfferWithdrawn: AugmentedEvent<ApiType, [AccountId32, u32, u32], { sender: AccountId32, collectionId: u32, nftId: u32 }>;
      /**
       * Token listed on Marketplace
       **/
      TokenListed: AugmentedEvent<ApiType, [AccountId32, u32, u32, u128], { owner: AccountId32, collectionId: u32, nftId: u32, price: u128 }>;
      /**
       * The price for a token was updated
       **/
      TokenPriceUpdated: AugmentedEvent<ApiType, [AccountId32, u32, u32, Option<u128>], { owner: AccountId32, collectionId: u32, nftId: u32, price: Option<u128> }>;
      /**
       * Token was sold to a new owner
       **/
      TokenSold: AugmentedEvent<ApiType, [AccountId32, AccountId32, u32, u32, u128], { owner: AccountId32, buyer: AccountId32, collectionId: u32, nftId: u32, price: u128 }>;
      /**
       * Token unlisted on Marketplace
       **/
      TokenUnlisted: AugmentedEvent<ApiType, [AccountId32, u32, u32], { owner: AccountId32, collectionId: u32, nftId: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /**
       * The \[sudoer\] just switched identity; the old key is supplied if one existed.
       **/
      KeyChanged: AugmentedEvent<ApiType, [Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      Sudid: AugmentedEvent<ApiType, [Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      SudoAsDone: AugmentedEvent<ApiType, [Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /**
       * `:code` was updated.
       **/
      CodeUpdated: AugmentedEvent<ApiType, [], {  }>;
      /**
       * An extrinsic failed.
       **/
      ExtrinsicFailed: AugmentedEvent<ApiType, [SpRuntimeDispatchError, FrameSupportWeightsDispatchInfo], { dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportWeightsDispatchInfo }>;
      /**
       * An extrinsic completed successfully.
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [FrameSupportWeightsDispatchInfo], { dispatchInfo: FrameSupportWeightsDispatchInfo }>;
      /**
       * An account was reaped.
       **/
      KilledAccount: AugmentedEvent<ApiType, [AccountId32], { account: AccountId32 }>;
      /**
       * A new account was created.
       **/
      NewAccount: AugmentedEvent<ApiType, [AccountId32], { account: AccountId32 }>;
      /**
       * On on-chain remark happened.
       **/
      Remarked: AugmentedEvent<ApiType, [AccountId32, H256], { sender: AccountId32, hash_: H256 }>;
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
      SomethingStored: AugmentedEvent<ApiType, [u32, AccountId32], {}>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    uniques: {
      /**
       * An approval for a `delegate` account to transfer the `instance` of an asset `class` was
       * cancelled by its `owner`.
       **/
      ApprovalCancelled: AugmentedEvent<ApiType, [u32, u32, AccountId32, AccountId32], { class: u32, instance: u32, owner: AccountId32, delegate: AccountId32 }>;
      /**
       * An `instance` of an asset `class` has been approved by the `owner` for transfer by a
       * `delegate`.
       **/
      ApprovedTransfer: AugmentedEvent<ApiType, [u32, u32, AccountId32, AccountId32], { class: u32, instance: u32, owner: AccountId32, delegate: AccountId32 }>;
      /**
       * An asset `class` has had its attributes changed by the `Force` origin.
       **/
      AssetStatusChanged: AugmentedEvent<ApiType, [u32], { class: u32 }>;
      /**
       * Attribute metadata has been cleared for an asset class or instance.
       **/
      AttributeCleared: AugmentedEvent<ApiType, [u32, Option<u32>, Bytes], { class: u32, maybeInstance: Option<u32>, key: Bytes }>;
      /**
       * New attribute metadata has been set for an asset class or instance.
       **/
      AttributeSet: AugmentedEvent<ApiType, [u32, Option<u32>, Bytes, Bytes], { class: u32, maybeInstance: Option<u32>, key: Bytes, value: Bytes }>;
      /**
       * An asset `instance` was destroyed.
       **/
      Burned: AugmentedEvent<ApiType, [u32, u32, AccountId32], { class: u32, instance: u32, owner: AccountId32 }>;
      /**
       * Some asset `class` was frozen.
       **/
      ClassFrozen: AugmentedEvent<ApiType, [u32], { class: u32 }>;
      /**
       * Metadata has been cleared for an asset class.
       **/
      ClassMetadataCleared: AugmentedEvent<ApiType, [u32], { class: u32 }>;
      /**
       * New metadata has been set for an asset class.
       **/
      ClassMetadataSet: AugmentedEvent<ApiType, [u32, Bytes, bool], { class: u32, data: Bytes, isFrozen: bool }>;
      /**
       * Some asset `class` was thawed.
       **/
      ClassThawed: AugmentedEvent<ApiType, [u32], { class: u32 }>;
      /**
       * An asset class was created.
       **/
      Created: AugmentedEvent<ApiType, [u32, AccountId32, AccountId32], { class: u32, creator: AccountId32, owner: AccountId32 }>;
      /**
       * An asset `class` was destroyed.
       **/
      Destroyed: AugmentedEvent<ApiType, [u32], { class: u32 }>;
      /**
       * An asset class was force-created.
       **/
      ForceCreated: AugmentedEvent<ApiType, [u32, AccountId32], { class: u32, owner: AccountId32 }>;
      /**
       * Some asset `instance` was frozen.
       **/
      Frozen: AugmentedEvent<ApiType, [u32, u32], { class: u32, instance: u32 }>;
      /**
       * An asset `instance` was issued.
       **/
      Issued: AugmentedEvent<ApiType, [u32, u32, AccountId32], { class: u32, instance: u32, owner: AccountId32 }>;
      /**
       * Metadata has been cleared for an asset instance.
       **/
      MetadataCleared: AugmentedEvent<ApiType, [u32, u32], { class: u32, instance: u32 }>;
      /**
       * New metadata has been set for an asset instance.
       **/
      MetadataSet: AugmentedEvent<ApiType, [u32, u32, Bytes, bool], { class: u32, instance: u32, data: Bytes, isFrozen: bool }>;
      /**
       * The owner changed.
       **/
      OwnerChanged: AugmentedEvent<ApiType, [u32, AccountId32], { class: u32, newOwner: AccountId32 }>;
      /**
       * Ownership acceptance has changed for an account.
       **/
      OwnershipAcceptanceChanged: AugmentedEvent<ApiType, [AccountId32, Option<u32>], { who: AccountId32, maybeClass: Option<u32> }>;
      /**
       * Metadata has been cleared for an asset instance.
       **/
      Redeposited: AugmentedEvent<ApiType, [u32, Vec<u32>], { class: u32, successfulInstances: Vec<u32> }>;
      /**
       * The management team changed.
       **/
      TeamChanged: AugmentedEvent<ApiType, [u32, AccountId32, AccountId32, AccountId32], { class: u32, issuer: AccountId32, admin: AccountId32, freezer: AccountId32 }>;
      /**
       * Some asset `instance` was thawed.
       **/
      Thawed: AugmentedEvent<ApiType, [u32, u32], { class: u32, instance: u32 }>;
      /**
       * An asset `instance` was transferred.
       **/
      Transferred: AugmentedEvent<ApiType, [u32, u32, AccountId32, AccountId32], { class: u32, instance: u32, from: AccountId32, to: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /**
       * Batch of dispatches completed fully with no error.
       **/
      BatchCompleted: AugmentedEvent<ApiType, [], {  }>;
      /**
       * Batch of dispatches completed but has errors.
       **/
      BatchCompletedWithErrors: AugmentedEvent<ApiType, [], {  }>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
       * well as the error.
       **/
      BatchInterrupted: AugmentedEvent<ApiType, [u32, SpRuntimeDispatchError], { index: u32, error: SpRuntimeDispatchError }>;
      /**
       * A call was dispatched.
       **/
      DispatchedAs: AugmentedEvent<ApiType, [Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single item within a Batch of dispatches has completed with no error.
       **/
      ItemCompleted: AugmentedEvent<ApiType, [], {  }>;
      /**
       * A single item within a Batch of dispatches has completed with error.
       **/
      ItemFailed: AugmentedEvent<ApiType, [SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
