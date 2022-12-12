// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Compact, Enum, Null, Option, Result, Struct, Text, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H256, MultiAddress, Perbill, Permill } from '@polkadot/types/interfaces/runtime';
import type { Event } from '@polkadot/types/interfaces/system';

/** @name FinalityGrandpaEquivocationPrecommit */
export interface FinalityGrandpaEquivocationPrecommit extends Struct {
  readonly roundNumber: u64;
  readonly identity: SpFinalityGrandpaAppPublic;
  readonly first: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
  readonly second: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
}

/** @name FinalityGrandpaEquivocationPrevote */
export interface FinalityGrandpaEquivocationPrevote extends Struct {
  readonly roundNumber: u64;
  readonly identity: SpFinalityGrandpaAppPublic;
  readonly first: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
  readonly second: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
}

/** @name FinalityGrandpaPrecommit */
export interface FinalityGrandpaPrecommit extends Struct {
  readonly targetHash: H256;
  readonly targetNumber: u32;
}

/** @name FinalityGrandpaPrevote */
export interface FinalityGrandpaPrevote extends Struct {
  readonly targetHash: H256;
  readonly targetNumber: u32;
}

/** @name FrameSupportDispatchDispatchClass */
export interface FrameSupportDispatchDispatchClass extends Enum {
  readonly isNormal: boolean;
  readonly isOperational: boolean;
  readonly isMandatory: boolean;
  readonly type: 'Normal' | 'Operational' | 'Mandatory';
}

/** @name FrameSupportDispatchDispatchInfo */
export interface FrameSupportDispatchDispatchInfo extends Struct {
  readonly weight: SpWeightsWeightV2Weight;
  readonly class: FrameSupportDispatchDispatchClass;
  readonly paysFee: FrameSupportDispatchPays;
}

/** @name FrameSupportDispatchPays */
export interface FrameSupportDispatchPays extends Enum {
  readonly isYes: boolean;
  readonly isNo: boolean;
  readonly type: 'Yes' | 'No';
}

/** @name FrameSupportDispatchPerDispatchClassU32 */
export interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
  readonly normal: u32;
  readonly operational: u32;
  readonly mandatory: u32;
}

/** @name FrameSupportDispatchPerDispatchClassWeight */
export interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
  readonly normal: SpWeightsWeightV2Weight;
  readonly operational: SpWeightsWeightV2Weight;
  readonly mandatory: SpWeightsWeightV2Weight;
}

/** @name FrameSupportDispatchPerDispatchClassWeightsPerClass */
export interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
  readonly normal: FrameSystemLimitsWeightsPerClass;
  readonly operational: FrameSystemLimitsWeightsPerClass;
  readonly mandatory: FrameSystemLimitsWeightsPerClass;
}

/** @name FrameSupportDispatchRawOrigin */
export interface FrameSupportDispatchRawOrigin extends Enum {
  readonly isRoot: boolean;
  readonly isSigned: boolean;
  readonly asSigned: AccountId32;
  readonly isNone: boolean;
  readonly type: 'Root' | 'Signed' | 'None';
}

/** @name FrameSupportTokensMiscBalanceStatus */
export interface FrameSupportTokensMiscBalanceStatus extends Enum {
  readonly isFree: boolean;
  readonly isReserved: boolean;
  readonly type: 'Free' | 'Reserved';
}

/** @name FrameSystemAccountInfo */
export interface FrameSystemAccountInfo extends Struct {
  readonly nonce: u32;
  readonly consumers: u32;
  readonly providers: u32;
  readonly sufficients: u32;
  readonly data: PalletBalancesAccountData;
}

/** @name FrameSystemCall */
export interface FrameSystemCall extends Enum {
  readonly isFillBlock: boolean;
  readonly asFillBlock: {
    readonly ratio: Perbill;
  } & Struct;
  readonly isRemark: boolean;
  readonly asRemark: {
    readonly remark: Bytes;
  } & Struct;
  readonly isSetHeapPages: boolean;
  readonly asSetHeapPages: {
    readonly pages: u64;
  } & Struct;
  readonly isSetCode: boolean;
  readonly asSetCode: {
    readonly code: Bytes;
  } & Struct;
  readonly isSetCodeWithoutChecks: boolean;
  readonly asSetCodeWithoutChecks: {
    readonly code: Bytes;
  } & Struct;
  readonly isSetStorage: boolean;
  readonly asSetStorage: {
    readonly items: Vec<ITuple<[Bytes, Bytes]>>;
  } & Struct;
  readonly isKillStorage: boolean;
  readonly asKillStorage: {
    readonly keys_: Vec<Bytes>;
  } & Struct;
  readonly isKillPrefix: boolean;
  readonly asKillPrefix: {
    readonly prefix: Bytes;
    readonly subkeys: u32;
  } & Struct;
  readonly isRemarkWithEvent: boolean;
  readonly asRemarkWithEvent: {
    readonly remark: Bytes;
  } & Struct;
  readonly type: 'FillBlock' | 'Remark' | 'SetHeapPages' | 'SetCode' | 'SetCodeWithoutChecks' | 'SetStorage' | 'KillStorage' | 'KillPrefix' | 'RemarkWithEvent';
}

/** @name FrameSystemError */
export interface FrameSystemError extends Enum {
  readonly isInvalidSpecName: boolean;
  readonly isSpecVersionNeedsToIncrease: boolean;
  readonly isFailedToExtractRuntimeVersion: boolean;
  readonly isNonDefaultComposite: boolean;
  readonly isNonZeroRefCount: boolean;
  readonly isCallFiltered: boolean;
  readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
}

/** @name FrameSystemEvent */
export interface FrameSystemEvent extends Enum {
  readonly isExtrinsicSuccess: boolean;
  readonly asExtrinsicSuccess: {
    readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
  } & Struct;
  readonly isExtrinsicFailed: boolean;
  readonly asExtrinsicFailed: {
    readonly dispatchError: SpRuntimeDispatchError;
    readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
  } & Struct;
  readonly isCodeUpdated: boolean;
  readonly isNewAccount: boolean;
  readonly asNewAccount: {
    readonly account: AccountId32;
  } & Struct;
  readonly isKilledAccount: boolean;
  readonly asKilledAccount: {
    readonly account: AccountId32;
  } & Struct;
  readonly isRemarked: boolean;
  readonly asRemarked: {
    readonly sender: AccountId32;
    readonly hash_: H256;
  } & Struct;
  readonly type: 'ExtrinsicSuccess' | 'ExtrinsicFailed' | 'CodeUpdated' | 'NewAccount' | 'KilledAccount' | 'Remarked';
}

/** @name FrameSystemEventRecord */
export interface FrameSystemEventRecord extends Struct {
  readonly phase: FrameSystemPhase;
  readonly event: Event;
  readonly topics: Vec<H256>;
}

/** @name FrameSystemExtensionsCheckGenesis */
export interface FrameSystemExtensionsCheckGenesis extends Null {}

/** @name FrameSystemExtensionsCheckNonce */
export interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

/** @name FrameSystemExtensionsCheckSpecVersion */
export interface FrameSystemExtensionsCheckSpecVersion extends Null {}

/** @name FrameSystemExtensionsCheckTxVersion */
export interface FrameSystemExtensionsCheckTxVersion extends Null {}

/** @name FrameSystemExtensionsCheckWeight */
export interface FrameSystemExtensionsCheckWeight extends Null {}

/** @name FrameSystemLastRuntimeUpgradeInfo */
export interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
  readonly specVersion: Compact<u32>;
  readonly specName: Text;
}

/** @name FrameSystemLimitsBlockLength */
export interface FrameSystemLimitsBlockLength extends Struct {
  readonly max: FrameSupportDispatchPerDispatchClassU32;
}

/** @name FrameSystemLimitsBlockWeights */
export interface FrameSystemLimitsBlockWeights extends Struct {
  readonly baseBlock: SpWeightsWeightV2Weight;
  readonly maxBlock: SpWeightsWeightV2Weight;
  readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
}

/** @name FrameSystemLimitsWeightsPerClass */
export interface FrameSystemLimitsWeightsPerClass extends Struct {
  readonly baseExtrinsic: SpWeightsWeightV2Weight;
  readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
  readonly maxTotal: Option<SpWeightsWeightV2Weight>;
  readonly reserved: Option<SpWeightsWeightV2Weight>;
}

/** @name FrameSystemPhase */
export interface FrameSystemPhase extends Enum {
  readonly isApplyExtrinsic: boolean;
  readonly asApplyExtrinsic: u32;
  readonly isFinalization: boolean;
  readonly isInitialization: boolean;
  readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
}

/** @name PalletBalancesAccountData */
export interface PalletBalancesAccountData extends Struct {
  readonly free: u128;
  readonly reserved: u128;
  readonly miscFrozen: u128;
  readonly feeFrozen: u128;
}

/** @name PalletBalancesBalanceLock */
export interface PalletBalancesBalanceLock extends Struct {
  readonly id: U8aFixed;
  readonly amount: u128;
  readonly reasons: PalletBalancesReasons;
}

/** @name PalletBalancesCall */
export interface PalletBalancesCall extends Enum {
  readonly isTransfer: boolean;
  readonly asTransfer: {
    readonly dest: MultiAddress;
    readonly value: Compact<u128>;
  } & Struct;
  readonly isSetBalance: boolean;
  readonly asSetBalance: {
    readonly who: MultiAddress;
    readonly newFree: Compact<u128>;
    readonly newReserved: Compact<u128>;
  } & Struct;
  readonly isForceTransfer: boolean;
  readonly asForceTransfer: {
    readonly source: MultiAddress;
    readonly dest: MultiAddress;
    readonly value: Compact<u128>;
  } & Struct;
  readonly isTransferKeepAlive: boolean;
  readonly asTransferKeepAlive: {
    readonly dest: MultiAddress;
    readonly value: Compact<u128>;
  } & Struct;
  readonly isTransferAll: boolean;
  readonly asTransferAll: {
    readonly dest: MultiAddress;
    readonly keepAlive: bool;
  } & Struct;
  readonly isForceUnreserve: boolean;
  readonly asForceUnreserve: {
    readonly who: MultiAddress;
    readonly amount: u128;
  } & Struct;
  readonly type: 'Transfer' | 'SetBalance' | 'ForceTransfer' | 'TransferKeepAlive' | 'TransferAll' | 'ForceUnreserve';
}

/** @name PalletBalancesError */
export interface PalletBalancesError extends Enum {
  readonly isVestingBalance: boolean;
  readonly isLiquidityRestrictions: boolean;
  readonly isInsufficientBalance: boolean;
  readonly isExistentialDeposit: boolean;
  readonly isKeepAlive: boolean;
  readonly isExistingVestingSchedule: boolean;
  readonly isDeadAccount: boolean;
  readonly isTooManyReserves: boolean;
  readonly type: 'VestingBalance' | 'LiquidityRestrictions' | 'InsufficientBalance' | 'ExistentialDeposit' | 'KeepAlive' | 'ExistingVestingSchedule' | 'DeadAccount' | 'TooManyReserves';
}

/** @name PalletBalancesEvent */
export interface PalletBalancesEvent extends Enum {
  readonly isEndowed: boolean;
  readonly asEndowed: {
    readonly account: AccountId32;
    readonly freeBalance: u128;
  } & Struct;
  readonly isDustLost: boolean;
  readonly asDustLost: {
    readonly account: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly isTransfer: boolean;
  readonly asTransfer: {
    readonly from: AccountId32;
    readonly to: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly isBalanceSet: boolean;
  readonly asBalanceSet: {
    readonly who: AccountId32;
    readonly free: u128;
    readonly reserved: u128;
  } & Struct;
  readonly isReserved: boolean;
  readonly asReserved: {
    readonly who: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly isUnreserved: boolean;
  readonly asUnreserved: {
    readonly who: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly isReserveRepatriated: boolean;
  readonly asReserveRepatriated: {
    readonly from: AccountId32;
    readonly to: AccountId32;
    readonly amount: u128;
    readonly destinationStatus: FrameSupportTokensMiscBalanceStatus;
  } & Struct;
  readonly isDeposit: boolean;
  readonly asDeposit: {
    readonly who: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly isWithdraw: boolean;
  readonly asWithdraw: {
    readonly who: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly isSlashed: boolean;
  readonly asSlashed: {
    readonly who: AccountId32;
    readonly amount: u128;
  } & Struct;
  readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'BalanceSet' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'Deposit' | 'Withdraw' | 'Slashed';
}

/** @name PalletBalancesReasons */
export interface PalletBalancesReasons extends Enum {
  readonly isFee: boolean;
  readonly isMisc: boolean;
  readonly isAll: boolean;
  readonly type: 'Fee' | 'Misc' | 'All';
}

/** @name PalletBalancesReleases */
export interface PalletBalancesReleases extends Enum {
  readonly isV100: boolean;
  readonly isV200: boolean;
  readonly type: 'V100' | 'V200';
}

/** @name PalletBalancesReserveData */
export interface PalletBalancesReserveData extends Struct {
  readonly id: U8aFixed;
  readonly amount: u128;
}

/** @name PalletGrandpaCall */
export interface PalletGrandpaCall extends Enum {
  readonly isReportEquivocation: boolean;
  readonly asReportEquivocation: {
    readonly equivocationProof: SpFinalityGrandpaEquivocationProof;
    readonly keyOwnerProof: SpCoreVoid;
  } & Struct;
  readonly isReportEquivocationUnsigned: boolean;
  readonly asReportEquivocationUnsigned: {
    readonly equivocationProof: SpFinalityGrandpaEquivocationProof;
    readonly keyOwnerProof: SpCoreVoid;
  } & Struct;
  readonly isNoteStalled: boolean;
  readonly asNoteStalled: {
    readonly delay: u32;
    readonly bestFinalizedBlockNumber: u32;
  } & Struct;
  readonly type: 'ReportEquivocation' | 'ReportEquivocationUnsigned' | 'NoteStalled';
}

/** @name PalletGrandpaError */
export interface PalletGrandpaError extends Enum {
  readonly isPauseFailed: boolean;
  readonly isResumeFailed: boolean;
  readonly isChangePending: boolean;
  readonly isTooSoon: boolean;
  readonly isInvalidKeyOwnershipProof: boolean;
  readonly isInvalidEquivocationProof: boolean;
  readonly isDuplicateOffenceReport: boolean;
  readonly type: 'PauseFailed' | 'ResumeFailed' | 'ChangePending' | 'TooSoon' | 'InvalidKeyOwnershipProof' | 'InvalidEquivocationProof' | 'DuplicateOffenceReport';
}

/** @name PalletGrandpaEvent */
export interface PalletGrandpaEvent extends Enum {
  readonly isNewAuthorities: boolean;
  readonly asNewAuthorities: {
    readonly authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
  } & Struct;
  readonly isPaused: boolean;
  readonly isResumed: boolean;
  readonly type: 'NewAuthorities' | 'Paused' | 'Resumed';
}

/** @name PalletGrandpaStoredPendingChange */
export interface PalletGrandpaStoredPendingChange extends Struct {
  readonly scheduledAt: u32;
  readonly delay: u32;
  readonly nextAuthorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
  readonly forced: Option<u32>;
}

/** @name PalletGrandpaStoredState */
export interface PalletGrandpaStoredState extends Enum {
  readonly isLive: boolean;
  readonly isPendingPause: boolean;
  readonly asPendingPause: {
    readonly scheduledAt: u32;
    readonly delay: u32;
  } & Struct;
  readonly isPaused: boolean;
  readonly isPendingResume: boolean;
  readonly asPendingResume: {
    readonly scheduledAt: u32;
    readonly delay: u32;
  } & Struct;
  readonly type: 'Live' | 'PendingPause' | 'Paused' | 'PendingResume';
}

/** @name PalletRmrkCoreCall */
export interface PalletRmrkCoreCall extends Enum {
  readonly isMintNft: boolean;
  readonly asMintNft: {
    readonly owner: Option<AccountId32>;
    readonly nftId: u32;
    readonly collectionId: u32;
    readonly royaltyRecipient: Option<AccountId32>;
    readonly royalty: Option<Permill>;
    readonly metadata: Bytes;
    readonly transferable: bool;
    readonly resources: Option<Vec<RmrkTraitsResourceResourceInfoMin>>;
  } & Struct;
  readonly isMintNftDirectlyToNft: boolean;
  readonly asMintNftDirectlyToNft: {
    readonly owner: ITuple<[u32, u32]>;
    readonly nftId: u32;
    readonly collectionId: u32;
    readonly royaltyRecipient: Option<AccountId32>;
    readonly royalty: Option<Permill>;
    readonly metadata: Bytes;
    readonly transferable: bool;
    readonly resources: Option<Vec<RmrkTraitsResourceResourceInfoMin>>;
  } & Struct;
  readonly isCreateCollection: boolean;
  readonly asCreateCollection: {
    readonly collectionId: u32;
    readonly metadata: Bytes;
    readonly max: Option<u32>;
    readonly symbol: Bytes;
  } & Struct;
  readonly isBurnNft: boolean;
  readonly asBurnNft: {
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isDestroyCollection: boolean;
  readonly asDestroyCollection: {
    readonly collectionId: u32;
  } & Struct;
  readonly isSend: boolean;
  readonly asSend: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly newOwner: RmrkTraitsNftAccountIdOrCollectionNftTuple;
  } & Struct;
  readonly isAcceptNft: boolean;
  readonly asAcceptNft: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly newOwner: RmrkTraitsNftAccountIdOrCollectionNftTuple;
  } & Struct;
  readonly isRejectNft: boolean;
  readonly asRejectNft: {
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isChangeCollectionIssuer: boolean;
  readonly asChangeCollectionIssuer: {
    readonly collectionId: u32;
    readonly newIssuer: MultiAddress;
  } & Struct;
  readonly isSetProperty: boolean;
  readonly asSetProperty: {
    readonly collectionId: u32;
    readonly maybeNftId: Option<u32>;
    readonly key: Bytes;
    readonly value: Bytes;
  } & Struct;
  readonly isLockCollection: boolean;
  readonly asLockCollection: {
    readonly collectionId: u32;
  } & Struct;
  readonly isAddBasicResource: boolean;
  readonly asAddBasicResource: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resource: RmrkTraitsResourceBasicResource;
    readonly resourceId: u32;
  } & Struct;
  readonly isAddComposableResource: boolean;
  readonly asAddComposableResource: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resource: RmrkTraitsResourceComposableResource;
    readonly resourceId: u32;
  } & Struct;
  readonly isAddSlotResource: boolean;
  readonly asAddSlotResource: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resource: RmrkTraitsResourceSlotResource;
    readonly resourceId: u32;
  } & Struct;
  readonly isReplaceResource: boolean;
  readonly asReplaceResource: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resource: RmrkTraitsResourceResourceTypes;
    readonly resourceId: u32;
  } & Struct;
  readonly isAcceptResource: boolean;
  readonly asAcceptResource: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resourceId: u32;
  } & Struct;
  readonly isRemoveResource: boolean;
  readonly asRemoveResource: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resourceId: u32;
  } & Struct;
  readonly isAcceptResourceRemoval: boolean;
  readonly asAcceptResourceRemoval: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly resourceId: u32;
  } & Struct;
  readonly isSetPriority: boolean;
  readonly asSetPriority: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly priorities: Vec<u32>;
  } & Struct;
  readonly type: 'MintNft' | 'MintNftDirectlyToNft' | 'CreateCollection' | 'BurnNft' | 'DestroyCollection' | 'Send' | 'AcceptNft' | 'RejectNft' | 'ChangeCollectionIssuer' | 'SetProperty' | 'LockCollection' | 'AddBasicResource' | 'AddComposableResource' | 'AddSlotResource' | 'ReplaceResource' | 'AcceptResource' | 'RemoveResource' | 'AcceptResourceRemoval' | 'SetPriority';
}

/** @name PalletRmrkCoreError */
export interface PalletRmrkCoreError extends Enum {
  readonly isNoneValue: boolean;
  readonly isStorageOverflow: boolean;
  readonly isTooLong: boolean;
  readonly isNoAvailableCollectionId: boolean;
  readonly isNoAvailableResourceId: boolean;
  readonly isMetadataNotSet: boolean;
  readonly isRecipientNotSet: boolean;
  readonly isNoAvailableNftId: boolean;
  readonly isNotInRange: boolean;
  readonly isRoyaltyNotSet: boolean;
  readonly isCollectionUnknown: boolean;
  readonly isNoPermission: boolean;
  readonly isNoWitness: boolean;
  readonly isCollectionNotEmpty: boolean;
  readonly isCollectionFullOrLocked: boolean;
  readonly isCannotSendToDescendentOrSelf: boolean;
  readonly isResourceAlreadyExists: boolean;
  readonly isNftAlreadyExists: boolean;
  readonly isEmptyResource: boolean;
  readonly isTooManyRecursions: boolean;
  readonly isNftIsLocked: boolean;
  readonly isCannotAcceptNonOwnedNft: boolean;
  readonly isCannotRejectNonOwnedNft: boolean;
  readonly isCannotRejectNonPendingNft: boolean;
  readonly isResourceDoesntExist: boolean;
  readonly isResourceNotPending: boolean;
  readonly isNonTransferable: boolean;
  readonly isCannotSendEquippedItem: boolean;
  readonly isCannotAcceptToNewOwner: boolean;
  readonly isFailedTransferHooksPreCheck: boolean;
  readonly isFailedTransferHooksPostTransfer: boolean;
  readonly type: 'NoneValue' | 'StorageOverflow' | 'TooLong' | 'NoAvailableCollectionId' | 'NoAvailableResourceId' | 'MetadataNotSet' | 'RecipientNotSet' | 'NoAvailableNftId' | 'NotInRange' | 'RoyaltyNotSet' | 'CollectionUnknown' | 'NoPermission' | 'NoWitness' | 'CollectionNotEmpty' | 'CollectionFullOrLocked' | 'CannotSendToDescendentOrSelf' | 'ResourceAlreadyExists' | 'NftAlreadyExists' | 'EmptyResource' | 'TooManyRecursions' | 'NftIsLocked' | 'CannotAcceptNonOwnedNft' | 'CannotRejectNonOwnedNft' | 'CannotRejectNonPendingNft' | 'ResourceDoesntExist' | 'ResourceNotPending' | 'NonTransferable' | 'CannotSendEquippedItem' | 'CannotAcceptToNewOwner' | 'FailedTransferHooksPreCheck' | 'FailedTransferHooksPostTransfer';
}

/** @name PalletRmrkCoreEvent */
export interface PalletRmrkCoreEvent extends Enum {
  readonly isCollectionCreated: boolean;
  readonly asCollectionCreated: {
    readonly issuer: AccountId32;
    readonly collectionId: u32;
  } & Struct;
  readonly isNftMinted: boolean;
  readonly asNftMinted: {
    readonly owner: RmrkTraitsNftAccountIdOrCollectionNftTuple;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isNftBurned: boolean;
  readonly asNftBurned: {
    readonly owner: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isCollectionDestroyed: boolean;
  readonly asCollectionDestroyed: {
    readonly issuer: AccountId32;
    readonly collectionId: u32;
  } & Struct;
  readonly isNftSent: boolean;
  readonly asNftSent: {
    readonly sender: AccountId32;
    readonly recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple;
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly approvalRequired: bool;
  } & Struct;
  readonly isNftAccepted: boolean;
  readonly asNftAccepted: {
    readonly sender: AccountId32;
    readonly recipient: RmrkTraitsNftAccountIdOrCollectionNftTuple;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isNftRejected: boolean;
  readonly asNftRejected: {
    readonly sender: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isIssuerChanged: boolean;
  readonly asIssuerChanged: {
    readonly oldIssuer: AccountId32;
    readonly newIssuer: AccountId32;
    readonly collectionId: u32;
  } & Struct;
  readonly isPropertySet: boolean;
  readonly asPropertySet: {
    readonly collectionId: u32;
    readonly maybeNftId: Option<u32>;
    readonly key: Bytes;
    readonly value: Bytes;
  } & Struct;
  readonly isPropertyRemoved: boolean;
  readonly asPropertyRemoved: {
    readonly collectionId: u32;
    readonly maybeNftId: Option<u32>;
    readonly key: Bytes;
  } & Struct;
  readonly isCollectionLocked: boolean;
  readonly asCollectionLocked: {
    readonly issuer: AccountId32;
    readonly collectionId: u32;
  } & Struct;
  readonly isResourceAdded: boolean;
  readonly asResourceAdded: {
    readonly nftId: u32;
    readonly resourceId: u32;
    readonly collectionId: u32;
  } & Struct;
  readonly isResourceReplaced: boolean;
  readonly asResourceReplaced: {
    readonly nftId: u32;
    readonly resourceId: u32;
    readonly collectionId: u32;
  } & Struct;
  readonly isResourceAccepted: boolean;
  readonly asResourceAccepted: {
    readonly nftId: u32;
    readonly resourceId: u32;
    readonly collectionId: u32;
  } & Struct;
  readonly isResourceRemoval: boolean;
  readonly asResourceRemoval: {
    readonly nftId: u32;
    readonly resourceId: u32;
    readonly collectionId: u32;
  } & Struct;
  readonly isResourceRemovalAccepted: boolean;
  readonly asResourceRemovalAccepted: {
    readonly nftId: u32;
    readonly resourceId: u32;
    readonly collectionId: u32;
  } & Struct;
  readonly isPrioritySet: boolean;
  readonly asPrioritySet: {
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly type: 'CollectionCreated' | 'NftMinted' | 'NftBurned' | 'CollectionDestroyed' | 'NftSent' | 'NftAccepted' | 'NftRejected' | 'IssuerChanged' | 'PropertySet' | 'PropertyRemoved' | 'CollectionLocked' | 'ResourceAdded' | 'ResourceReplaced' | 'ResourceAccepted' | 'ResourceRemoval' | 'ResourceRemovalAccepted' | 'PrioritySet';
}

/** @name PalletRmrkEquipCall */
export interface PalletRmrkEquipCall extends Enum {
  readonly isChangeBaseIssuer: boolean;
  readonly asChangeBaseIssuer: {
    readonly baseId: u32;
    readonly newIssuer: MultiAddress;
  } & Struct;
  readonly isEquip: boolean;
  readonly asEquip: {
    readonly item: ITuple<[u32, u32]>;
    readonly equipper: ITuple<[u32, u32]>;
    readonly resourceId: u32;
    readonly base: u32;
    readonly slot: u32;
  } & Struct;
  readonly isUnequip: boolean;
  readonly asUnequip: {
    readonly item: ITuple<[u32, u32]>;
    readonly unequipper: ITuple<[u32, u32]>;
    readonly base: u32;
    readonly slot: u32;
  } & Struct;
  readonly isEquippable: boolean;
  readonly asEquippable: {
    readonly baseId: u32;
    readonly slotId: u32;
    readonly equippables: RmrkTraitsPartEquippableList;
  } & Struct;
  readonly isEquippableAdd: boolean;
  readonly asEquippableAdd: {
    readonly baseId: u32;
    readonly slotId: u32;
    readonly equippable: u32;
  } & Struct;
  readonly isEquippableRemove: boolean;
  readonly asEquippableRemove: {
    readonly baseId: u32;
    readonly slotId: u32;
    readonly equippable: u32;
  } & Struct;
  readonly isThemeAdd: boolean;
  readonly asThemeAdd: {
    readonly baseId: u32;
    readonly theme: RmrkTraitsTheme;
  } & Struct;
  readonly isCreateBase: boolean;
  readonly asCreateBase: {
    readonly baseType: Bytes;
    readonly symbol: Bytes;
    readonly parts: Vec<RmrkTraitsPartPartType>;
  } & Struct;
  readonly type: 'ChangeBaseIssuer' | 'Equip' | 'Unequip' | 'Equippable' | 'EquippableAdd' | 'EquippableRemove' | 'ThemeAdd' | 'CreateBase';
}

/** @name PalletRmrkEquipError */
export interface PalletRmrkEquipError extends Enum {
  readonly isPermissionError: boolean;
  readonly isItemDoesntExist: boolean;
  readonly isEquipperDoesntExist: boolean;
  readonly isNoAvailableBaseId: boolean;
  readonly isTooManyEquippables: boolean;
  readonly isNoAvailablePartId: boolean;
  readonly isMustBeDirectParent: boolean;
  readonly isPartDoesntExist: boolean;
  readonly isBaseDoesntExist: boolean;
  readonly isCantEquipFixedPart: boolean;
  readonly isNoResourceForThisBaseFoundOnNft: boolean;
  readonly isCollectionNotEquippable: boolean;
  readonly isItemHasNoResourceToEquipThere: boolean;
  readonly isNoEquippableOnFixedPart: boolean;
  readonly isNeedsDefaultThemeFirst: boolean;
  readonly isItemAlreadyEquipped: boolean;
  readonly isSlotAlreadyEquipped: boolean;
  readonly isSlotNotEquipped: boolean;
  readonly isUnknownError: boolean;
  readonly isExceedsMaxPartsPerBase: boolean;
  readonly isTooManyProperties: boolean;
  readonly isItemNotEquipped: boolean;
  readonly isUnequipperMustOwnEitherItemOrEquipper: boolean;
  readonly isUnexpectedTryFromIntError: boolean;
  readonly isUnexpectedVecConversionError: boolean;
  readonly type: 'PermissionError' | 'ItemDoesntExist' | 'EquipperDoesntExist' | 'NoAvailableBaseId' | 'TooManyEquippables' | 'NoAvailablePartId' | 'MustBeDirectParent' | 'PartDoesntExist' | 'BaseDoesntExist' | 'CantEquipFixedPart' | 'NoResourceForThisBaseFoundOnNft' | 'CollectionNotEquippable' | 'ItemHasNoResourceToEquipThere' | 'NoEquippableOnFixedPart' | 'NeedsDefaultThemeFirst' | 'ItemAlreadyEquipped' | 'SlotAlreadyEquipped' | 'SlotNotEquipped' | 'UnknownError' | 'ExceedsMaxPartsPerBase' | 'TooManyProperties' | 'ItemNotEquipped' | 'UnequipperMustOwnEitherItemOrEquipper' | 'UnexpectedTryFromIntError' | 'UnexpectedVecConversionError';
}

/** @name PalletRmrkEquipEvent */
export interface PalletRmrkEquipEvent extends Enum {
  readonly isBaseCreated: boolean;
  readonly asBaseCreated: {
    readonly issuer: AccountId32;
    readonly baseId: u32;
  } & Struct;
  readonly isSlotEquipped: boolean;
  readonly asSlotEquipped: {
    readonly itemCollection: u32;
    readonly itemNft: u32;
    readonly baseId: u32;
    readonly slotId: u32;
  } & Struct;
  readonly isSlotUnequipped: boolean;
  readonly asSlotUnequipped: {
    readonly itemCollection: u32;
    readonly itemNft: u32;
    readonly baseId: u32;
    readonly slotId: u32;
  } & Struct;
  readonly isEquippablesUpdated: boolean;
  readonly asEquippablesUpdated: {
    readonly baseId: u32;
    readonly slotId: u32;
  } & Struct;
  readonly isBaseIssuerChanged: boolean;
  readonly asBaseIssuerChanged: {
    readonly oldIssuer: AccountId32;
    readonly newIssuer: AccountId32;
    readonly baseId: u32;
  } & Struct;
  readonly type: 'BaseCreated' | 'SlotEquipped' | 'SlotUnequipped' | 'EquippablesUpdated' | 'BaseIssuerChanged';
}

/** @name PalletRmrkMarketCall */
export interface PalletRmrkMarketCall extends Enum {
  readonly isBuy: boolean;
  readonly asBuy: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly amount: Option<u128>;
  } & Struct;
  readonly isList: boolean;
  readonly asList: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly amount: u128;
    readonly expires: Option<u32>;
  } & Struct;
  readonly isUnlist: boolean;
  readonly asUnlist: {
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isMakeOffer: boolean;
  readonly asMakeOffer: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly amount: u128;
    readonly expires: Option<u32>;
  } & Struct;
  readonly isWithdrawOffer: boolean;
  readonly asWithdrawOffer: {
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isAcceptOffer: boolean;
  readonly asAcceptOffer: {
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly offerer: AccountId32;
  } & Struct;
  readonly type: 'Buy' | 'List' | 'Unlist' | 'MakeOffer' | 'WithdrawOffer' | 'AcceptOffer';
}

/** @name PalletRmrkMarketError */
export interface PalletRmrkMarketError extends Enum {
  readonly isNoPermission: boolean;
  readonly isTokenNotForSale: boolean;
  readonly isCannotWithdrawOffer: boolean;
  readonly isCannotUnlistToken: boolean;
  readonly isCannotOfferOnOwnToken: boolean;
  readonly isCannotBuyOwnToken: boolean;
  readonly isUnknownOffer: boolean;
  readonly isCannotListNftOwnedByNft: boolean;
  readonly isTokenDoesNotExist: boolean;
  readonly isOfferTooLow: boolean;
  readonly isAlreadyOffered: boolean;
  readonly isOfferHasExpired: boolean;
  readonly isListingHasExpired: boolean;
  readonly isPriceDiffersFromExpected: boolean;
  readonly isNonTransferable: boolean;
  readonly type: 'NoPermission' | 'TokenNotForSale' | 'CannotWithdrawOffer' | 'CannotUnlistToken' | 'CannotOfferOnOwnToken' | 'CannotBuyOwnToken' | 'UnknownOffer' | 'CannotListNftOwnedByNft' | 'TokenDoesNotExist' | 'OfferTooLow' | 'AlreadyOffered' | 'OfferHasExpired' | 'ListingHasExpired' | 'PriceDiffersFromExpected' | 'NonTransferable';
}

/** @name PalletRmrkMarketEvent */
export interface PalletRmrkMarketEvent extends Enum {
  readonly isTokenPriceUpdated: boolean;
  readonly asTokenPriceUpdated: {
    readonly owner: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly price: Option<u128>;
  } & Struct;
  readonly isTokenSold: boolean;
  readonly asTokenSold: {
    readonly owner: AccountId32;
    readonly buyer: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly price: u128;
  } & Struct;
  readonly isTokenListed: boolean;
  readonly asTokenListed: {
    readonly owner: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly price: u128;
  } & Struct;
  readonly isTokenUnlisted: boolean;
  readonly asTokenUnlisted: {
    readonly owner: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isOfferPlaced: boolean;
  readonly asOfferPlaced: {
    readonly offerer: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
    readonly price: u128;
  } & Struct;
  readonly isOfferWithdrawn: boolean;
  readonly asOfferWithdrawn: {
    readonly sender: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly isOfferAccepted: boolean;
  readonly asOfferAccepted: {
    readonly owner: AccountId32;
    readonly buyer: AccountId32;
    readonly collectionId: u32;
    readonly nftId: u32;
  } & Struct;
  readonly type: 'TokenPriceUpdated' | 'TokenSold' | 'TokenListed' | 'TokenUnlisted' | 'OfferPlaced' | 'OfferWithdrawn' | 'OfferAccepted';
}

/** @name PalletRmrkMarketListInfo */
export interface PalletRmrkMarketListInfo extends Struct {
  readonly listedBy: AccountId32;
  readonly amount: u128;
  readonly expires: Option<u32>;
}

/** @name PalletRmrkMarketOffer */
export interface PalletRmrkMarketOffer extends Struct {
  readonly maker: AccountId32;
  readonly amount: u128;
  readonly expires: Option<u32>;
}

/** @name PalletSudoCall */
export interface PalletSudoCall extends Enum {
  readonly isSudo: boolean;
  readonly asSudo: {
    readonly call: Call;
  } & Struct;
  readonly isSudoUncheckedWeight: boolean;
  readonly asSudoUncheckedWeight: {
    readonly call: Call;
    readonly weight: SpWeightsWeightV2Weight;
  } & Struct;
  readonly isSetKey: boolean;
  readonly asSetKey: {
    readonly new_: MultiAddress;
  } & Struct;
  readonly isSudoAs: boolean;
  readonly asSudoAs: {
    readonly who: MultiAddress;
    readonly call: Call;
  } & Struct;
  readonly type: 'Sudo' | 'SudoUncheckedWeight' | 'SetKey' | 'SudoAs';
}

/** @name PalletSudoError */
export interface PalletSudoError extends Enum {
  readonly isRequireSudo: boolean;
  readonly type: 'RequireSudo';
}

/** @name PalletSudoEvent */
export interface PalletSudoEvent extends Enum {
  readonly isSudid: boolean;
  readonly asSudid: {
    readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
  } & Struct;
  readonly isKeyChanged: boolean;
  readonly asKeyChanged: {
    readonly oldSudoer: Option<AccountId32>;
  } & Struct;
  readonly isSudoAsDone: boolean;
  readonly asSudoAsDone: {
    readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
  } & Struct;
  readonly type: 'Sudid' | 'KeyChanged' | 'SudoAsDone';
}

/** @name PalletTemplateCall */
export interface PalletTemplateCall extends Enum {
  readonly isDoSomething: boolean;
  readonly asDoSomething: {
    readonly something: u32;
  } & Struct;
  readonly isCauseError: boolean;
  readonly type: 'DoSomething' | 'CauseError';
}

/** @name PalletTemplateError */
export interface PalletTemplateError extends Enum {
  readonly isNoneValue: boolean;
  readonly isStorageOverflow: boolean;
  readonly type: 'NoneValue' | 'StorageOverflow';
}

/** @name PalletTemplateEvent */
export interface PalletTemplateEvent extends Enum {
  readonly isSomethingStored: boolean;
  readonly asSomethingStored: ITuple<[u32, AccountId32]>;
  readonly type: 'SomethingStored';
}

/** @name PalletTimestampCall */
export interface PalletTimestampCall extends Enum {
  readonly isSet: boolean;
  readonly asSet: {
    readonly now: Compact<u64>;
  } & Struct;
  readonly type: 'Set';
}

/** @name PalletTransactionPaymentChargeTransactionPayment */
export interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

/** @name PalletTransactionPaymentEvent */
export interface PalletTransactionPaymentEvent extends Enum {
  readonly isTransactionFeePaid: boolean;
  readonly asTransactionFeePaid: {
    readonly who: AccountId32;
    readonly actualFee: u128;
    readonly tip: u128;
  } & Struct;
  readonly type: 'TransactionFeePaid';
}

/** @name PalletTransactionPaymentReleases */
export interface PalletTransactionPaymentReleases extends Enum {
  readonly isV1Ancient: boolean;
  readonly isV2: boolean;
  readonly type: 'V1Ancient' | 'V2';
}

/** @name PalletUniquesCall */
export interface PalletUniquesCall extends Enum {
  readonly isCreate: boolean;
  readonly asCreate: {
    readonly collection: u32;
    readonly admin: MultiAddress;
  } & Struct;
  readonly isForceCreate: boolean;
  readonly asForceCreate: {
    readonly collection: u32;
    readonly owner: MultiAddress;
    readonly freeHolding: bool;
  } & Struct;
  readonly isDestroy: boolean;
  readonly asDestroy: {
    readonly collection: u32;
    readonly witness: PalletUniquesDestroyWitness;
  } & Struct;
  readonly isMint: boolean;
  readonly asMint: {
    readonly collection: u32;
    readonly item: u32;
    readonly owner: MultiAddress;
  } & Struct;
  readonly isBurn: boolean;
  readonly asBurn: {
    readonly collection: u32;
    readonly item: u32;
    readonly checkOwner: Option<MultiAddress>;
  } & Struct;
  readonly isTransfer: boolean;
  readonly asTransfer: {
    readonly collection: u32;
    readonly item: u32;
    readonly dest: MultiAddress;
  } & Struct;
  readonly isRedeposit: boolean;
  readonly asRedeposit: {
    readonly collection: u32;
    readonly items: Vec<u32>;
  } & Struct;
  readonly isFreeze: boolean;
  readonly asFreeze: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isThaw: boolean;
  readonly asThaw: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isFreezeCollection: boolean;
  readonly asFreezeCollection: {
    readonly collection: u32;
  } & Struct;
  readonly isThawCollection: boolean;
  readonly asThawCollection: {
    readonly collection: u32;
  } & Struct;
  readonly isTransferOwnership: boolean;
  readonly asTransferOwnership: {
    readonly collection: u32;
    readonly owner: MultiAddress;
  } & Struct;
  readonly isSetTeam: boolean;
  readonly asSetTeam: {
    readonly collection: u32;
    readonly issuer: MultiAddress;
    readonly admin: MultiAddress;
    readonly freezer: MultiAddress;
  } & Struct;
  readonly isApproveTransfer: boolean;
  readonly asApproveTransfer: {
    readonly collection: u32;
    readonly item: u32;
    readonly delegate: MultiAddress;
  } & Struct;
  readonly isCancelApproval: boolean;
  readonly asCancelApproval: {
    readonly collection: u32;
    readonly item: u32;
    readonly maybeCheckDelegate: Option<MultiAddress>;
  } & Struct;
  readonly isForceItemStatus: boolean;
  readonly asForceItemStatus: {
    readonly collection: u32;
    readonly owner: MultiAddress;
    readonly issuer: MultiAddress;
    readonly admin: MultiAddress;
    readonly freezer: MultiAddress;
    readonly freeHolding: bool;
    readonly isFrozen: bool;
  } & Struct;
  readonly isSetAttribute: boolean;
  readonly asSetAttribute: {
    readonly collection: u32;
    readonly maybeItem: Option<u32>;
    readonly key: Bytes;
    readonly value: Bytes;
  } & Struct;
  readonly isClearAttribute: boolean;
  readonly asClearAttribute: {
    readonly collection: u32;
    readonly maybeItem: Option<u32>;
    readonly key: Bytes;
  } & Struct;
  readonly isSetMetadata: boolean;
  readonly asSetMetadata: {
    readonly collection: u32;
    readonly item: u32;
    readonly data: Bytes;
    readonly isFrozen: bool;
  } & Struct;
  readonly isClearMetadata: boolean;
  readonly asClearMetadata: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isSetCollectionMetadata: boolean;
  readonly asSetCollectionMetadata: {
    readonly collection: u32;
    readonly data: Bytes;
    readonly isFrozen: bool;
  } & Struct;
  readonly isClearCollectionMetadata: boolean;
  readonly asClearCollectionMetadata: {
    readonly collection: u32;
  } & Struct;
  readonly isSetAcceptOwnership: boolean;
  readonly asSetAcceptOwnership: {
    readonly maybeCollection: Option<u32>;
  } & Struct;
  readonly isSetCollectionMaxSupply: boolean;
  readonly asSetCollectionMaxSupply: {
    readonly collection: u32;
    readonly maxSupply: u32;
  } & Struct;
  readonly isSetPrice: boolean;
  readonly asSetPrice: {
    readonly collection: u32;
    readonly item: u32;
    readonly price: Option<u128>;
    readonly whitelistedBuyer: Option<MultiAddress>;
  } & Struct;
  readonly isBuyItem: boolean;
  readonly asBuyItem: {
    readonly collection: u32;
    readonly item: u32;
    readonly bidPrice: u128;
  } & Struct;
  readonly type: 'Create' | 'ForceCreate' | 'Destroy' | 'Mint' | 'Burn' | 'Transfer' | 'Redeposit' | 'Freeze' | 'Thaw' | 'FreezeCollection' | 'ThawCollection' | 'TransferOwnership' | 'SetTeam' | 'ApproveTransfer' | 'CancelApproval' | 'ForceItemStatus' | 'SetAttribute' | 'ClearAttribute' | 'SetMetadata' | 'ClearMetadata' | 'SetCollectionMetadata' | 'ClearCollectionMetadata' | 'SetAcceptOwnership' | 'SetCollectionMaxSupply' | 'SetPrice' | 'BuyItem';
}

/** @name PalletUniquesCollectionDetails */
export interface PalletUniquesCollectionDetails extends Struct {
  readonly owner: AccountId32;
  readonly issuer: AccountId32;
  readonly admin: AccountId32;
  readonly freezer: AccountId32;
  readonly totalDeposit: u128;
  readonly freeHolding: bool;
  readonly items: u32;
  readonly itemMetadatas: u32;
  readonly attributes: u32;
  readonly isFrozen: bool;
}

/** @name PalletUniquesCollectionMetadata */
export interface PalletUniquesCollectionMetadata extends Struct {
  readonly deposit: u128;
  readonly data: Bytes;
  readonly isFrozen: bool;
}

/** @name PalletUniquesDestroyWitness */
export interface PalletUniquesDestroyWitness extends Struct {
  readonly items: Compact<u32>;
  readonly itemMetadatas: Compact<u32>;
  readonly attributes: Compact<u32>;
}

/** @name PalletUniquesError */
export interface PalletUniquesError extends Enum {
  readonly isNoPermission: boolean;
  readonly isUnknownCollection: boolean;
  readonly isAlreadyExists: boolean;
  readonly isWrongOwner: boolean;
  readonly isBadWitness: boolean;
  readonly isInUse: boolean;
  readonly isFrozen: boolean;
  readonly isWrongDelegate: boolean;
  readonly isNoDelegate: boolean;
  readonly isUnapproved: boolean;
  readonly isUnaccepted: boolean;
  readonly isLocked: boolean;
  readonly isMaxSupplyReached: boolean;
  readonly isMaxSupplyAlreadySet: boolean;
  readonly isMaxSupplyTooSmall: boolean;
  readonly isUnknownItem: boolean;
  readonly isNotForSale: boolean;
  readonly isBidTooLow: boolean;
  readonly type: 'NoPermission' | 'UnknownCollection' | 'AlreadyExists' | 'WrongOwner' | 'BadWitness' | 'InUse' | 'Frozen' | 'WrongDelegate' | 'NoDelegate' | 'Unapproved' | 'Unaccepted' | 'Locked' | 'MaxSupplyReached' | 'MaxSupplyAlreadySet' | 'MaxSupplyTooSmall' | 'UnknownItem' | 'NotForSale' | 'BidTooLow';
}

/** @name PalletUniquesEvent */
export interface PalletUniquesEvent extends Enum {
  readonly isCreated: boolean;
  readonly asCreated: {
    readonly collection: u32;
    readonly creator: AccountId32;
    readonly owner: AccountId32;
  } & Struct;
  readonly isForceCreated: boolean;
  readonly asForceCreated: {
    readonly collection: u32;
    readonly owner: AccountId32;
  } & Struct;
  readonly isDestroyed: boolean;
  readonly asDestroyed: {
    readonly collection: u32;
  } & Struct;
  readonly isIssued: boolean;
  readonly asIssued: {
    readonly collection: u32;
    readonly item: u32;
    readonly owner: AccountId32;
  } & Struct;
  readonly isTransferred: boolean;
  readonly asTransferred: {
    readonly collection: u32;
    readonly item: u32;
    readonly from: AccountId32;
    readonly to: AccountId32;
  } & Struct;
  readonly isBurned: boolean;
  readonly asBurned: {
    readonly collection: u32;
    readonly item: u32;
    readonly owner: AccountId32;
  } & Struct;
  readonly isFrozen: boolean;
  readonly asFrozen: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isThawed: boolean;
  readonly asThawed: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isCollectionFrozen: boolean;
  readonly asCollectionFrozen: {
    readonly collection: u32;
  } & Struct;
  readonly isCollectionThawed: boolean;
  readonly asCollectionThawed: {
    readonly collection: u32;
  } & Struct;
  readonly isOwnerChanged: boolean;
  readonly asOwnerChanged: {
    readonly collection: u32;
    readonly newOwner: AccountId32;
  } & Struct;
  readonly isTeamChanged: boolean;
  readonly asTeamChanged: {
    readonly collection: u32;
    readonly issuer: AccountId32;
    readonly admin: AccountId32;
    readonly freezer: AccountId32;
  } & Struct;
  readonly isApprovedTransfer: boolean;
  readonly asApprovedTransfer: {
    readonly collection: u32;
    readonly item: u32;
    readonly owner: AccountId32;
    readonly delegate: AccountId32;
  } & Struct;
  readonly isApprovalCancelled: boolean;
  readonly asApprovalCancelled: {
    readonly collection: u32;
    readonly item: u32;
    readonly owner: AccountId32;
    readonly delegate: AccountId32;
  } & Struct;
  readonly isItemStatusChanged: boolean;
  readonly asItemStatusChanged: {
    readonly collection: u32;
  } & Struct;
  readonly isCollectionMetadataSet: boolean;
  readonly asCollectionMetadataSet: {
    readonly collection: u32;
    readonly data: Bytes;
    readonly isFrozen: bool;
  } & Struct;
  readonly isCollectionMetadataCleared: boolean;
  readonly asCollectionMetadataCleared: {
    readonly collection: u32;
  } & Struct;
  readonly isMetadataSet: boolean;
  readonly asMetadataSet: {
    readonly collection: u32;
    readonly item: u32;
    readonly data: Bytes;
    readonly isFrozen: bool;
  } & Struct;
  readonly isMetadataCleared: boolean;
  readonly asMetadataCleared: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isRedeposited: boolean;
  readonly asRedeposited: {
    readonly collection: u32;
    readonly successfulItems: Vec<u32>;
  } & Struct;
  readonly isAttributeSet: boolean;
  readonly asAttributeSet: {
    readonly collection: u32;
    readonly maybeItem: Option<u32>;
    readonly key: Bytes;
    readonly value: Bytes;
  } & Struct;
  readonly isAttributeCleared: boolean;
  readonly asAttributeCleared: {
    readonly collection: u32;
    readonly maybeItem: Option<u32>;
    readonly key: Bytes;
  } & Struct;
  readonly isOwnershipAcceptanceChanged: boolean;
  readonly asOwnershipAcceptanceChanged: {
    readonly who: AccountId32;
    readonly maybeCollection: Option<u32>;
  } & Struct;
  readonly isCollectionMaxSupplySet: boolean;
  readonly asCollectionMaxSupplySet: {
    readonly collection: u32;
    readonly maxSupply: u32;
  } & Struct;
  readonly isItemPriceSet: boolean;
  readonly asItemPriceSet: {
    readonly collection: u32;
    readonly item: u32;
    readonly price: u128;
    readonly whitelistedBuyer: Option<AccountId32>;
  } & Struct;
  readonly isItemPriceRemoved: boolean;
  readonly asItemPriceRemoved: {
    readonly collection: u32;
    readonly item: u32;
  } & Struct;
  readonly isItemBought: boolean;
  readonly asItemBought: {
    readonly collection: u32;
    readonly item: u32;
    readonly price: u128;
    readonly seller: AccountId32;
    readonly buyer: AccountId32;
  } & Struct;
  readonly type: 'Created' | 'ForceCreated' | 'Destroyed' | 'Issued' | 'Transferred' | 'Burned' | 'Frozen' | 'Thawed' | 'CollectionFrozen' | 'CollectionThawed' | 'OwnerChanged' | 'TeamChanged' | 'ApprovedTransfer' | 'ApprovalCancelled' | 'ItemStatusChanged' | 'CollectionMetadataSet' | 'CollectionMetadataCleared' | 'MetadataSet' | 'MetadataCleared' | 'Redeposited' | 'AttributeSet' | 'AttributeCleared' | 'OwnershipAcceptanceChanged' | 'CollectionMaxSupplySet' | 'ItemPriceSet' | 'ItemPriceRemoved' | 'ItemBought';
}

/** @name PalletUniquesItemDetails */
export interface PalletUniquesItemDetails extends Struct {
  readonly owner: AccountId32;
  readonly approved: Option<AccountId32>;
  readonly isFrozen: bool;
  readonly deposit: u128;
}

/** @name PalletUniquesItemMetadata */
export interface PalletUniquesItemMetadata extends Struct {
  readonly deposit: u128;
  readonly data: Bytes;
  readonly isFrozen: bool;
}

/** @name PalletUtilityCall */
export interface PalletUtilityCall extends Enum {
  readonly isBatch: boolean;
  readonly asBatch: {
    readonly calls: Vec<Call>;
  } & Struct;
  readonly isAsDerivative: boolean;
  readonly asAsDerivative: {
    readonly index: u16;
    readonly call: Call;
  } & Struct;
  readonly isBatchAll: boolean;
  readonly asBatchAll: {
    readonly calls: Vec<Call>;
  } & Struct;
  readonly isDispatchAs: boolean;
  readonly asDispatchAs: {
    readonly asOrigin: RmrkSubstrateRuntimeOriginCaller;
    readonly call: Call;
  } & Struct;
  readonly isForceBatch: boolean;
  readonly asForceBatch: {
    readonly calls: Vec<Call>;
  } & Struct;
  readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch';
}

/** @name PalletUtilityError */
export interface PalletUtilityError extends Enum {
  readonly isTooManyCalls: boolean;
  readonly type: 'TooManyCalls';
}

/** @name PalletUtilityEvent */
export interface PalletUtilityEvent extends Enum {
  readonly isBatchInterrupted: boolean;
  readonly asBatchInterrupted: {
    readonly index: u32;
    readonly error: SpRuntimeDispatchError;
  } & Struct;
  readonly isBatchCompleted: boolean;
  readonly isBatchCompletedWithErrors: boolean;
  readonly isItemCompleted: boolean;
  readonly isItemFailed: boolean;
  readonly asItemFailed: {
    readonly error: SpRuntimeDispatchError;
  } & Struct;
  readonly isDispatchedAs: boolean;
  readonly asDispatchedAs: {
    readonly result: Result<Null, SpRuntimeDispatchError>;
  } & Struct;
  readonly type: 'BatchInterrupted' | 'BatchCompleted' | 'BatchCompletedWithErrors' | 'ItemCompleted' | 'ItemFailed' | 'DispatchedAs';
}

/** @name PhantomTypePhantomType */
export interface PhantomTypePhantomType extends Vec<Lookup178> {}

/** @name RmrkSubstrateRuntimeOriginCaller */
export interface RmrkSubstrateRuntimeOriginCaller extends Enum {
  readonly isSystem: boolean;
  readonly asSystem: FrameSupportDispatchRawOrigin;
  readonly isVoid: boolean;
  readonly asVoid: SpCoreVoid;
  readonly type: 'System' | 'Void';
}

/** @name RmrkSubstrateRuntimeRuntime */
export interface RmrkSubstrateRuntimeRuntime extends Null {}

/** @name RmrkTraitsBaseBaseInfo */
export interface RmrkTraitsBaseBaseInfo extends Struct {
  readonly issuer: AccountId32;
  readonly baseType: Bytes;
  readonly symbol: Bytes;
}

/** @name RmrkTraitsCollectionCollectionInfo */
export interface RmrkTraitsCollectionCollectionInfo extends Struct {
  readonly issuer: AccountId32;
  readonly metadata: Bytes;
  readonly max: Option<u32>;
  readonly symbol: Bytes;
  readonly nftsCount: u32;
}

/** @name RmrkTraitsNftAccountIdOrCollectionNftTuple */
export interface RmrkTraitsNftAccountIdOrCollectionNftTuple extends Enum {
  readonly isAccountId: boolean;
  readonly asAccountId: AccountId32;
  readonly isCollectionAndNftTuple: boolean;
  readonly asCollectionAndNftTuple: ITuple<[u32, u32]>;
  readonly type: 'AccountId' | 'CollectionAndNftTuple';
}

/** @name RmrkTraitsNftNftChild */
export interface RmrkTraitsNftNftChild extends Struct {
  readonly collectionId: u32;
  readonly nftId: u32;
}

/** @name RmrkTraitsNftNftInfo */
export interface RmrkTraitsNftNftInfo extends Struct {
  readonly owner: RmrkTraitsNftAccountIdOrCollectionNftTuple;
  readonly royalty: Option<RmrkTraitsNftRoyaltyInfo>;
  readonly metadata: Bytes;
  readonly equipped: Option<ITuple<[u32, u32]>>;
  readonly pending: bool;
  readonly transferable: bool;
}

/** @name RmrkTraitsNftRoyaltyInfo */
export interface RmrkTraitsNftRoyaltyInfo extends Struct {
  readonly recipient: AccountId32;
  readonly amount: Permill;
}

/** @name RmrkTraitsPartEquippableList */
export interface RmrkTraitsPartEquippableList extends Enum {
  readonly isAll: boolean;
  readonly isEmpty: boolean;
  readonly isCustom: boolean;
  readonly asCustom: Vec<u32>;
  readonly type: 'All' | 'Empty' | 'Custom';
}

/** @name RmrkTraitsPartFixedPart */
export interface RmrkTraitsPartFixedPart extends Struct {
  readonly id: u32;
  readonly z: u32;
  readonly src: Bytes;
}

/** @name RmrkTraitsPartPartType */
export interface RmrkTraitsPartPartType extends Enum {
  readonly isFixedPart: boolean;
  readonly asFixedPart: RmrkTraitsPartFixedPart;
  readonly isSlotPart: boolean;
  readonly asSlotPart: RmrkTraitsPartSlotPart;
  readonly type: 'FixedPart' | 'SlotPart';
}

/** @name RmrkTraitsPartSlotPart */
export interface RmrkTraitsPartSlotPart extends Struct {
  readonly id: u32;
  readonly equippable: RmrkTraitsPartEquippableList;
  readonly src: Option<Bytes>;
  readonly z: u32;
}

/** @name RmrkTraitsPropertyPropertyInfo */
export interface RmrkTraitsPropertyPropertyInfo extends Struct {
  readonly key: Bytes;
  readonly value: Bytes;
}

/** @name RmrkTraitsResourceBasicResource */
export interface RmrkTraitsResourceBasicResource extends Struct {
  readonly metadata: Bytes;
}

/** @name RmrkTraitsResourceComposableResource */
export interface RmrkTraitsResourceComposableResource extends Struct {
  readonly parts: Vec<u32>;
  readonly base: u32;
  readonly metadata: Option<Bytes>;
  readonly slot: Option<ITuple<[u32, u32]>>;
}

/** @name RmrkTraitsResourceResourceInfo */
export interface RmrkTraitsResourceResourceInfo extends Struct {
  readonly id: u32;
  readonly resource: RmrkTraitsResourceResourceTypes;
  readonly pending: bool;
  readonly pendingRemoval: bool;
}

/** @name RmrkTraitsResourceResourceInfoMin */
export interface RmrkTraitsResourceResourceInfoMin extends Struct {
  readonly id: u32;
  readonly resource: RmrkTraitsResourceResourceTypes;
}

/** @name RmrkTraitsResourceResourceTypes */
export interface RmrkTraitsResourceResourceTypes extends Enum {
  readonly isBasic: boolean;
  readonly asBasic: RmrkTraitsResourceBasicResource;
  readonly isComposable: boolean;
  readonly asComposable: RmrkTraitsResourceComposableResource;
  readonly isSlot: boolean;
  readonly asSlot: RmrkTraitsResourceSlotResource;
  readonly type: 'Basic' | 'Composable' | 'Slot';
}

/** @name RmrkTraitsResourceSlotResource */
export interface RmrkTraitsResourceSlotResource extends Struct {
  readonly base: u32;
  readonly metadata: Option<Bytes>;
  readonly slot: u32;
}

/** @name RmrkTraitsTheme */
export interface RmrkTraitsTheme extends Struct {
  readonly name: Bytes;
  readonly properties: Vec<RmrkTraitsThemeThemeProperty>;
  readonly inherit: bool;
}

/** @name RmrkTraitsThemeThemeProperty */
export interface RmrkTraitsThemeThemeProperty extends Struct {
  readonly key: Bytes;
  readonly value: Bytes;
}

/** @name SpConsensusAuraSr25519AppSr25519Public */
export interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

/** @name SpCoreEcdsaSignature */
export interface SpCoreEcdsaSignature extends U8aFixed {}

/** @name SpCoreEd25519Public */
export interface SpCoreEd25519Public extends U8aFixed {}

/** @name SpCoreEd25519Signature */
export interface SpCoreEd25519Signature extends U8aFixed {}

/** @name SpCoreSr25519Public */
export interface SpCoreSr25519Public extends U8aFixed {}

/** @name SpCoreSr25519Signature */
export interface SpCoreSr25519Signature extends U8aFixed {}

/** @name SpCoreVoid */
export interface SpCoreVoid extends Null {}

/** @name SpFinalityGrandpaAppPublic */
export interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

/** @name SpFinalityGrandpaAppSignature */
export interface SpFinalityGrandpaAppSignature extends SpCoreEd25519Signature {}

/** @name SpFinalityGrandpaEquivocation */
export interface SpFinalityGrandpaEquivocation extends Enum {
  readonly isPrevote: boolean;
  readonly asPrevote: FinalityGrandpaEquivocationPrevote;
  readonly isPrecommit: boolean;
  readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
  readonly type: 'Prevote' | 'Precommit';
}

/** @name SpFinalityGrandpaEquivocationProof */
export interface SpFinalityGrandpaEquivocationProof extends Struct {
  readonly setId: u64;
  readonly equivocation: SpFinalityGrandpaEquivocation;
}

/** @name SpRuntimeArithmeticError */
export interface SpRuntimeArithmeticError extends Enum {
  readonly isUnderflow: boolean;
  readonly isOverflow: boolean;
  readonly isDivisionByZero: boolean;
  readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
}

/** @name SpRuntimeDigest */
export interface SpRuntimeDigest extends Struct {
  readonly logs: Vec<SpRuntimeDigestDigestItem>;
}

/** @name SpRuntimeDigestDigestItem */
export interface SpRuntimeDigestDigestItem extends Enum {
  readonly isOther: boolean;
  readonly asOther: Bytes;
  readonly isConsensus: boolean;
  readonly asConsensus: ITuple<[U8aFixed, Bytes]>;
  readonly isSeal: boolean;
  readonly asSeal: ITuple<[U8aFixed, Bytes]>;
  readonly isPreRuntime: boolean;
  readonly asPreRuntime: ITuple<[U8aFixed, Bytes]>;
  readonly isRuntimeEnvironmentUpdated: boolean;
  readonly type: 'Other' | 'Consensus' | 'Seal' | 'PreRuntime' | 'RuntimeEnvironmentUpdated';
}

/** @name SpRuntimeDispatchError */
export interface SpRuntimeDispatchError extends Enum {
  readonly isOther: boolean;
  readonly isCannotLookup: boolean;
  readonly isBadOrigin: boolean;
  readonly isModule: boolean;
  readonly asModule: SpRuntimeModuleError;
  readonly isConsumerRemaining: boolean;
  readonly isNoProviders: boolean;
  readonly isTooManyConsumers: boolean;
  readonly isToken: boolean;
  readonly asToken: SpRuntimeTokenError;
  readonly isArithmetic: boolean;
  readonly asArithmetic: SpRuntimeArithmeticError;
  readonly isTransactional: boolean;
  readonly asTransactional: SpRuntimeTransactionalError;
  readonly isExhausted: boolean;
  readonly isCorruption: boolean;
  readonly isUnavailable: boolean;
  readonly type: 'Other' | 'CannotLookup' | 'BadOrigin' | 'Module' | 'ConsumerRemaining' | 'NoProviders' | 'TooManyConsumers' | 'Token' | 'Arithmetic' | 'Transactional' | 'Exhausted' | 'Corruption' | 'Unavailable';
}

/** @name SpRuntimeModuleError */
export interface SpRuntimeModuleError extends Struct {
  readonly index: u8;
  readonly error: U8aFixed;
}

/** @name SpRuntimeMultiSignature */
export interface SpRuntimeMultiSignature extends Enum {
  readonly isEd25519: boolean;
  readonly asEd25519: SpCoreEd25519Signature;
  readonly isSr25519: boolean;
  readonly asSr25519: SpCoreSr25519Signature;
  readonly isEcdsa: boolean;
  readonly asEcdsa: SpCoreEcdsaSignature;
  readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
}

/** @name SpRuntimeTokenError */
export interface SpRuntimeTokenError extends Enum {
  readonly isNoFunds: boolean;
  readonly isWouldDie: boolean;
  readonly isBelowMinimum: boolean;
  readonly isCannotCreate: boolean;
  readonly isUnknownAsset: boolean;
  readonly isFrozen: boolean;
  readonly isUnsupported: boolean;
  readonly type: 'NoFunds' | 'WouldDie' | 'BelowMinimum' | 'CannotCreate' | 'UnknownAsset' | 'Frozen' | 'Unsupported';
}

/** @name SpRuntimeTransactionalError */
export interface SpRuntimeTransactionalError extends Enum {
  readonly isLimitReached: boolean;
  readonly isNoLayer: boolean;
  readonly type: 'LimitReached' | 'NoLayer';
}

/** @name SpVersionRuntimeVersion */
export interface SpVersionRuntimeVersion extends Struct {
  readonly specName: Text;
  readonly implName: Text;
  readonly authoringVersion: u32;
  readonly specVersion: u32;
  readonly implVersion: u32;
  readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
  readonly transactionVersion: u32;
  readonly stateVersion: u8;
}

/** @name SpWeightsRuntimeDbWeight */
export interface SpWeightsRuntimeDbWeight extends Struct {
  readonly read: u64;
  readonly write: u64;
}

/** @name SpWeightsWeightV2Weight */
export interface SpWeightsWeightV2Weight extends Struct {
  readonly refTime: Compact<u64>;
  readonly proofSize: Compact<u64>;
}

export type PHANTOM_RMRK = 'rmrk';
