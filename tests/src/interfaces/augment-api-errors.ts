// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';

declare module '@polkadot/api-base/types/errors' {
  export interface AugmentedErrors<ApiType extends ApiTypes> {
    balances: {
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Balance too low to send value
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed MaxReserves
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value
       **/
      VestingBalance: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    grandpa: {
      /**
       * Attempt to signal GRANDPA change with one already pending.
       **/
      ChangePending: AugmentedError<ApiType>;
      /**
       * A given equivocation report is valid but already previously reported.
       **/
      DuplicateOffenceReport: AugmentedError<ApiType>;
      /**
       * An equivocation proof provided as part of an equivocation report is invalid.
       **/
      InvalidEquivocationProof: AugmentedError<ApiType>;
      /**
       * A key ownership proof provided as part of an equivocation report is invalid.
       **/
      InvalidKeyOwnershipProof: AugmentedError<ApiType>;
      /**
       * Attempt to signal GRANDPA pause when the authority set isn't live
       * (either paused or already pending pause).
       **/
      PauseFailed: AugmentedError<ApiType>;
      /**
       * Attempt to signal GRANDPA resume when the authority set isn't paused
       * (either live or already pending resume).
       **/
      ResumeFailed: AugmentedError<ApiType>;
      /**
       * Cannot signal forced change so soon after last.
       **/
      TooSoon: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    rmrkCore: {
      CannotAcceptNonOwnedNft: AugmentedError<ApiType>;
      CannotRejectNonOwnedNft: AugmentedError<ApiType>;
      CannotSendToDescendentOrSelf: AugmentedError<ApiType>;
      CollectionFullOrLocked: AugmentedError<ApiType>;
      CollectionNotEmpty: AugmentedError<ApiType>;
      CollectionUnknown: AugmentedError<ApiType>;
      EmptyResource: AugmentedError<ApiType>;
      MetadataNotSet: AugmentedError<ApiType>;
      NftIsLocked: AugmentedError<ApiType>;
      NoAvailableCollectionId: AugmentedError<ApiType>;
      NoAvailableNftId: AugmentedError<ApiType>;
      /**
       * Error names should be descriptive.
       **/
      NoneValue: AugmentedError<ApiType>;
      NoPermission: AugmentedError<ApiType>;
      NotInRange: AugmentedError<ApiType>;
      NoWitness: AugmentedError<ApiType>;
      RecipientNotSet: AugmentedError<ApiType>;
      ResourceAlreadyExists: AugmentedError<ApiType>;
      ResourceDoesntExist: AugmentedError<ApiType>;
      /**
       * Accepting a resource that is not pending should fail
       **/
      ResourceNotPending: AugmentedError<ApiType>;
      RoyaltyNotSet: AugmentedError<ApiType>;
      /**
       * Errors should have helpful documentation associated with them.
       **/
      StorageOverflow: AugmentedError<ApiType>;
      TooLong: AugmentedError<ApiType>;
      TooManyRecursions: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    rmrkEquip: {
      AlreadyEquipped: AugmentedError<ApiType>;
      BaseDoesntExist: AugmentedError<ApiType>;
      CantEquipFixedPart: AugmentedError<ApiType>;
      CollectionNotEquippable: AugmentedError<ApiType>;
      EquipperDoesntExist: AugmentedError<ApiType>;
      ExceedsMaxPartsPerBase: AugmentedError<ApiType>;
      ItemDoesntExist: AugmentedError<ApiType>;
      ItemHasNoResourceToEquipThere: AugmentedError<ApiType>;
      MustBeDirectParent: AugmentedError<ApiType>;
      NeedsDefaultThemeFirst: AugmentedError<ApiType>;
      NoAvailableBaseId: AugmentedError<ApiType>;
      NoAvailablePartId: AugmentedError<ApiType>;
      NoEquippableOnFixedPart: AugmentedError<ApiType>;
      NoResourceForThisBaseFoundOnNft: AugmentedError<ApiType>;
      PartDoesntExist: AugmentedError<ApiType>;
      PermissionError: AugmentedError<ApiType>;
      TooManyProperties: AugmentedError<ApiType>;
      UnknownError: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    rmrkMarket: {
      /**
       * Account cannot offer on a NFT again with an active offer
       **/
      AlreadyOffered: AugmentedError<ApiType>;
      /**
       * Cannot buy NFT that is already owned
       **/
      CannotBuyOwnToken: AugmentedError<ApiType>;
      /**
       * Cannot list NFT owned by a NFT
       **/
      CannotListNftOwnedByNft: AugmentedError<ApiType>;
      /**
       * Cannot make offer on NFT on own NFT
       **/
      CannotOfferOnOwnToken: AugmentedError<ApiType>;
      /**
       * Cannot unlist NFT as it has already been unlisted or sold
       **/
      CannotUnlistToken: AugmentedError<ApiType>;
      /**
       * Offer already accepted and cannot withdraw
       **/
      CannotWithdrawOffer: AugmentedError<ApiType>;
      /**
       * Listing has expired and cannot be bought
       **/
      ListingHasExpired: AugmentedError<ApiType>;
      /**
       * No permissions for account to interact with NFT
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * Accepted offer has expired and cannot be accepted
       **/
      OfferHasExpired: AugmentedError<ApiType>;
      /**
       * Offer is below the OfferMinimumAmount threshold
       **/
      OfferTooLow: AugmentedError<ApiType>;
      /**
       * Price differs from when `buy` was executed
       **/
      PriceDiffersFromExpected: AugmentedError<ApiType>;
      /**
       * Cannot list a non-existing NFT
       **/
      TokenDoesNotExist: AugmentedError<ApiType>;
      /**
       * Token cannot be bought
       **/
      TokenNotForSale: AugmentedError<ApiType>;
      /**
       * Offer is unknown
       **/
      UnknownOffer: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    sudo: {
      /**
       * Sender must be the Sudo account
       **/
      RequireSudo: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    system: {
      /**
       * The origin filter prevent the call to be dispatched.
       **/
      CallFiltered: AugmentedError<ApiType>;
      /**
       * Failed to extract the runtime version from the new runtime.
       * 
       * Either calling `Core_version` or decoding `RuntimeVersion` failed.
       **/
      FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
      /**
       * The name of specification does not match between the current runtime
       * and the new runtime.
       **/
      InvalidSpecName: AugmentedError<ApiType>;
      /**
       * Suicide called when the account has non-default composite data.
       **/
      NonDefaultComposite: AugmentedError<ApiType>;
      /**
       * There is a non-zero reference count preventing the account from being purged.
       **/
      NonZeroRefCount: AugmentedError<ApiType>;
      /**
       * The specification version is not allowed to decrease between the current runtime
       * and the new runtime.
       **/
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    templateModule: {
      /**
       * Error names should be descriptive.
       **/
      NoneValue: AugmentedError<ApiType>;
      /**
       * Errors should have helpful documentation associated with them.
       **/
      StorageOverflow: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    uniques: {
      /**
       * The asset instance ID has already been used for an asset.
       **/
      AlreadyExists: AugmentedError<ApiType>;
      /**
       * Invalid witness data given.
       **/
      BadWitness: AugmentedError<ApiType>;
      /**
       * The asset instance or class is frozen.
       **/
      Frozen: AugmentedError<ApiType>;
      /**
       * The asset ID is already taken.
       **/
      InUse: AugmentedError<ApiType>;
      /**
       * The asset instance is locked.
       **/
      Locked: AugmentedError<ApiType>;
      /**
       * There is no delegate approved.
       **/
      NoDelegate: AugmentedError<ApiType>;
      /**
       * The signing account has no permission to do the operation.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * The named owner has not signed ownership of the class is acceptable.
       **/
      Unaccepted: AugmentedError<ApiType>;
      /**
       * No approval exists that would allow the transfer.
       **/
      Unapproved: AugmentedError<ApiType>;
      /**
       * The given asset ID is unknown.
       **/
      UnknownClass: AugmentedError<ApiType>;
      /**
       * The delegate turned out to be different to what was expected.
       **/
      WrongDelegate: AugmentedError<ApiType>;
      /**
       * The owner turned out to be different to what was expected.
       **/
      WrongOwner: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    utility: {
      /**
       * Too many calls batched.
       **/
      TooManyCalls: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
  } // AugmentedErrors
} // declare module
