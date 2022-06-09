// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    miscFrozen: 'u128',
    feeFrozen: 'u128'
  },
  /**
   * Lookup7: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU64: {
    normal: 'u64',
    operational: 'u64',
    mandatory: 'u64'
  },
  /**
   * Lookup11: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup13: sp_runtime::generic::digest::DigestItem
   **/
  SpRuntimeDigestDigestItem: {
    _enum: {
      Other: 'Bytes',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      Consensus: '([u8;4],Bytes)',
      Seal: '([u8;4],Bytes)',
      PreRuntime: '([u8;4],Bytes)',
      __Unused7: 'Null',
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup16: frame_system::EventRecord<rmrk_substrate_runtime::Event, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup18: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
      },
      CodeUpdated: 'Null',
      NewAccount: {
        account: 'AccountId32',
      },
      KilledAccount: {
        account: 'AccountId32',
      },
      Remarked: {
        _alias: {
          hash_: 'hash',
        },
        sender: 'AccountId32',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup19: frame_support::weights::DispatchInfo
   **/
  FrameSupportWeightsDispatchInfo: {
    weight: 'u64',
    class: 'FrameSupportWeightsDispatchClass',
    paysFee: 'FrameSupportWeightsPays'
  },
  /**
   * Lookup20: frame_support::weights::DispatchClass
   **/
  FrameSupportWeightsDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup21: frame_support::weights::Pays
   **/
  FrameSupportWeightsPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup22: sp_runtime::DispatchError
   **/
  SpRuntimeDispatchError: {
    _enum: {
      Other: 'Null',
      CannotLookup: 'Null',
      BadOrigin: 'Null',
      Module: 'SpRuntimeModuleError',
      ConsumerRemaining: 'Null',
      NoProviders: 'Null',
      TooManyConsumers: 'Null',
      Token: 'SpRuntimeTokenError',
      Arithmetic: 'SpRuntimeArithmeticError',
      Transactional: 'SpRuntimeTransactionalError'
    }
  },
  /**
   * Lookup23: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup24: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported']
  },
  /**
   * Lookup25: sp_runtime::ArithmeticError
   **/
  SpRuntimeArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup26: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup27: pallet_grandpa::pallet::Event
   **/
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: 'Vec<(SpFinalityGrandpaAppPublic,u64)>',
      },
      Paused: 'Null',
      Resumed: 'Null'
    }
  },
  /**
   * Lookup30: sp_finality_grandpa::app::Public
   **/
  SpFinalityGrandpaAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup31: sp_core::ed25519::Public
   **/
  SpCoreEd25519Public: '[u8;32]',
  /**
   * Lookup32: pallet_balances::pallet::Event<T, I>
   **/
  PalletBalancesEvent: {
    _enum: {
      Endowed: {
        account: 'AccountId32',
        freeBalance: 'u128',
      },
      DustLost: {
        account: 'AccountId32',
        amount: 'u128',
      },
      Transfer: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      BalanceSet: {
        who: 'AccountId32',
        free: 'u128',
        reserved: 'u128',
      },
      Reserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unreserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      ReserveRepatriated: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
      },
      Deposit: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Withdraw: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Slashed: {
        who: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup33: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup34: pallet_sudo::pallet::Event<T>
   **/
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>',
      },
      KeyChanged: {
        oldSudoer: 'Option<AccountId32>',
      },
      SudoAsDone: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup38: pallet_template::pallet::Event<T>
   **/
  PalletTemplateEvent: {
    _enum: {
      SomethingStored: '(u32,AccountId32)'
    }
  },
  /**
   * Lookup39: pallet_rmrk_equip::pallet::Event<T>
   **/
  PalletRmrkEquipEvent: {
    _enum: {
      BaseCreated: {
        issuer: 'AccountId32',
        baseId: 'u32',
      },
      SlotEquipped: {
        itemCollection: 'u32',
        itemNft: 'u32',
        baseId: 'u32',
        slotId: 'u32',
      },
      SlotUnequipped: {
        itemCollection: 'u32',
        itemNft: 'u32',
        baseId: 'u32',
        slotId: 'u32',
      },
      EquippablesUpdated: {
        baseId: 'u32',
        slotId: 'u32',
      },
      BaseIssuerChanged: {
        oldIssuer: 'AccountId32',
        newIssuer: 'AccountId32',
        baseId: 'u32'
      }
    }
  },
  /**
   * Lookup40: pallet_rmrk_core::pallet::Event<T>
   **/
  PalletRmrkCoreEvent: {
    _enum: {
      CollectionCreated: {
        issuer: 'AccountId32',
        collectionId: 'u32',
      },
      NftMinted: {
        owner: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
      },
      NFTBurned: {
        owner: 'AccountId32',
        nftId: 'u32',
      },
      CollectionDestroyed: {
        issuer: 'AccountId32',
        collectionId: 'u32',
      },
      NFTSent: {
        sender: 'AccountId32',
        recipient: 'RmrkTraitsNftAccountIdOrCollectionNftTuple',
        collectionId: 'u32',
        nftId: 'u32',
        approvalRequired: 'bool',
      },
      NFTAccepted: {
        sender: 'AccountId32',
        recipient: 'RmrkTraitsNftAccountIdOrCollectionNftTuple',
        collectionId: 'u32',
        nftId: 'u32',
      },
      NFTRejected: {
        sender: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
      },
      IssuerChanged: {
        oldIssuer: 'AccountId32',
        newIssuer: 'AccountId32',
        collectionId: 'u32',
      },
      PropertySet: {
        collectionId: 'u32',
        maybeNftId: 'Option<u32>',
        key: 'Bytes',
        value: 'Bytes',
      },
      CollectionLocked: {
        issuer: 'AccountId32',
        collectionId: 'u32',
      },
      ResourceAdded: {
        nftId: 'u32',
        resourceId: 'u32',
      },
      ResourceAccepted: {
        nftId: 'u32',
        resourceId: 'u32',
      },
      ResourceRemoval: {
        nftId: 'u32',
        resourceId: 'u32',
      },
      ResourceRemovalAccepted: {
        nftId: 'u32',
        resourceId: 'u32',
      },
      PrioritySet: {
        collectionId: 'u32',
        nftId: 'u32'
      }
    }
  },
  /**
   * Lookup41: rmrk_traits::nft::AccountIdOrCollectionNftTuple<sp_core::crypto::AccountId32>
   **/
  RmrkTraitsNftAccountIdOrCollectionNftTuple: {
    _enum: {
      AccountId: 'AccountId32',
      CollectionAndNftTuple: '(u32,u32)'
    }
  },
  /**
   * Lookup46: pallet_rmrk_market::pallet::Event<T>
   **/
  PalletRmrkMarketEvent: {
    _enum: {
      TokenPriceUpdated: {
        owner: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
        price: 'Option<u128>',
      },
      TokenSold: {
        owner: 'AccountId32',
        buyer: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
        price: 'u128',
      },
      TokenListed: {
        owner: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
        price: 'u128',
      },
      TokenUnlisted: {
        owner: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
      },
      OfferPlaced: {
        offerer: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
        price: 'u128',
      },
      OfferWithdrawn: {
        sender: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32',
      },
      OfferAccepted: {
        owner: 'AccountId32',
        buyer: 'AccountId32',
        collectionId: 'u32',
        nftId: 'u32'
      }
    }
  },
  /**
   * Lookup48: pallet_uniques::pallet::Event<T, I>
   **/
  PalletUniquesEvent: {
    _enum: {
      Created: {
        class: 'u32',
        creator: 'AccountId32',
        owner: 'AccountId32',
      },
      ForceCreated: {
        class: 'u32',
        owner: 'AccountId32',
      },
      Destroyed: {
        class: 'u32',
      },
      Issued: {
        class: 'u32',
        instance: 'u32',
        owner: 'AccountId32',
      },
      Transferred: {
        class: 'u32',
        instance: 'u32',
        from: 'AccountId32',
        to: 'AccountId32',
      },
      Burned: {
        class: 'u32',
        instance: 'u32',
        owner: 'AccountId32',
      },
      Frozen: {
        class: 'u32',
        instance: 'u32',
      },
      Thawed: {
        class: 'u32',
        instance: 'u32',
      },
      ClassFrozen: {
        class: 'u32',
      },
      ClassThawed: {
        class: 'u32',
      },
      OwnerChanged: {
        class: 'u32',
        newOwner: 'AccountId32',
      },
      TeamChanged: {
        class: 'u32',
        issuer: 'AccountId32',
        admin: 'AccountId32',
        freezer: 'AccountId32',
      },
      ApprovedTransfer: {
        class: 'u32',
        instance: 'u32',
        owner: 'AccountId32',
        delegate: 'AccountId32',
      },
      ApprovalCancelled: {
        class: 'u32',
        instance: 'u32',
        owner: 'AccountId32',
        delegate: 'AccountId32',
      },
      AssetStatusChanged: {
        class: 'u32',
      },
      ClassMetadataSet: {
        class: 'u32',
        data: 'Bytes',
        isFrozen: 'bool',
      },
      ClassMetadataCleared: {
        class: 'u32',
      },
      MetadataSet: {
        class: 'u32',
        instance: 'u32',
        data: 'Bytes',
        isFrozen: 'bool',
      },
      MetadataCleared: {
        class: 'u32',
        instance: 'u32',
      },
      Redeposited: {
        class: 'u32',
        successfulInstances: 'Vec<u32>',
      },
      AttributeSet: {
        class: 'u32',
        maybeInstance: 'Option<u32>',
        key: 'Bytes',
        value: 'Bytes',
      },
      AttributeCleared: {
        class: 'u32',
        maybeInstance: 'Option<u32>',
        key: 'Bytes',
      },
      OwnershipAcceptanceChanged: {
        who: 'AccountId32',
        maybeClass: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup51: pallet_utility::pallet::Event
   **/
  PalletUtilityEvent: {
    _enum: {
      BatchInterrupted: {
        index: 'u32',
        error: 'SpRuntimeDispatchError',
      },
      BatchCompleted: 'Null',
      BatchCompletedWithErrors: 'Null',
      ItemCompleted: 'Null',
      ItemFailed: {
        error: 'SpRuntimeDispatchError',
      },
      DispatchedAs: {
        result: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup52: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup56: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup59: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      fill_block: {
        ratio: 'Perbill',
      },
      remark: {
        remark: 'Bytes',
      },
      set_heap_pages: {
        pages: 'u64',
      },
      set_code: {
        code: 'Bytes',
      },
      set_code_without_checks: {
        code: 'Bytes',
      },
      set_storage: {
        items: 'Vec<(Bytes,Bytes)>',
      },
      kill_storage: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'Vec<Bytes>',
      },
      kill_prefix: {
        prefix: 'Bytes',
        subkeys: 'u32',
      },
      remark_with_event: {
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup64: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'u64',
    maxBlock: 'u64',
    perClass: 'FrameSupportWeightsPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup65: frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup66: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'u64',
    maxExtrinsic: 'Option<u64>',
    maxTotal: 'Option<u64>',
    reserved: 'Option<u64>'
  },
  /**
   * Lookup68: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportWeightsPerDispatchClassU32'
  },
  /**
   * Lookup69: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup70: frame_support::weights::RuntimeDbWeight
   **/
  FrameSupportWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup71: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup77: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup79: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup82: sp_consensus_aura::sr25519::app_sr25519::Public
   **/
  SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
  /**
   * Lookup83: sp_core::sr25519::Public
   **/
  SpCoreSr25519Public: '[u8;32]',
  /**
   * Lookup86: pallet_grandpa::StoredState<N>
   **/
  PalletGrandpaStoredState: {
    _enum: {
      Live: 'Null',
      PendingPause: {
        scheduledAt: 'u32',
        delay: 'u32',
      },
      Paused: 'Null',
      PendingResume: {
        scheduledAt: 'u32',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup87: pallet_grandpa::StoredPendingChange<N, Limit>
   **/
  PalletGrandpaStoredPendingChange: {
    scheduledAt: 'u32',
    delay: 'u32',
    nextAuthorities: 'Vec<(SpFinalityGrandpaAppPublic,u64)>',
    forced: 'Option<u32>'
  },
  /**
   * Lookup89: pallet_grandpa::pallet::Call<T>
   **/
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: 'SpFinalityGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      report_equivocation_unsigned: {
        equivocationProof: 'SpFinalityGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      note_stalled: {
        delay: 'u32',
        bestFinalizedBlockNumber: 'u32'
      }
    }
  },
  /**
   * Lookup90: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N>
   **/
  SpFinalityGrandpaEquivocationProof: {
    setId: 'u64',
    equivocation: 'SpFinalityGrandpaEquivocation'
  },
  /**
   * Lookup91: sp_finality_grandpa::Equivocation<primitive_types::H256, N>
   **/
  SpFinalityGrandpaEquivocation: {
    _enum: {
      Prevote: 'FinalityGrandpaEquivocationPrevote',
      Precommit: 'FinalityGrandpaEquivocationPrecommit'
    }
  },
  /**
   * Lookup92: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: 'u64',
    identity: 'SpFinalityGrandpaAppPublic',
    first: '(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)',
    second: '(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)'
  },
  /**
   * Lookup93: finality_grandpa::Prevote<primitive_types::H256, N>
   **/
  FinalityGrandpaPrevote: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup94: sp_finality_grandpa::app::Signature
   **/
  SpFinalityGrandpaAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup95: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup98: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: 'u64',
    identity: 'SpFinalityGrandpaAppPublic',
    first: '(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)',
    second: '(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)'
  },
  /**
   * Lookup99: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup101: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup102: pallet_grandpa::pallet::Error<T>
   **/
  PalletGrandpaError: {
    _enum: ['PauseFailed', 'ResumeFailed', 'ChangePending', 'TooSoon', 'InvalidKeyOwnershipProof', 'InvalidEquivocationProof', 'DuplicateOffenceReport']
  },
  /**
   * Lookup104: pallet_balances::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup105: pallet_balances::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup108: pallet_balances::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup110: pallet_balances::Releases
   **/
  PalletBalancesReleases: {
    _enum: ['V1_0_0', 'V2_0_0']
  },
  /**
   * Lookup111: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>',
        newReserved: 'Compact<u128>',
      },
      force_transfer: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_keep_alive: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_all: {
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      force_unreserve: {
        who: 'MultiAddress',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup116: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'KeepAlive', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup118: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup120: frame_support::weights::WeightToFeeCoefficient<Balance>
   **/
  FrameSupportWeightsWeightToFeeCoefficient: {
    coeffInteger: 'u128',
    coeffFrac: 'Perbill',
    negative: 'bool',
    degree: 'u8'
  },
  /**
   * Lookup121: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'u64',
      },
      set_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      sudo_as: {
        who: 'MultiAddress',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup123: pallet_template::pallet::Call<T>
   **/
  PalletTemplateCall: {
    _enum: {
      do_something: {
        something: 'u32',
      },
      cause_error: 'Null'
    }
  },
  /**
   * Lookup124: pallet_rmrk_equip::pallet::Call<T>
   **/
  PalletRmrkEquipCall: {
    _enum: {
      change_base_issuer: {
        baseId: 'u32',
        newIssuer: 'MultiAddress',
      },
      equip: {
        item: '(u32,u32)',
        equipper: '(u32,u32)',
        resourceId: 'u32',
        base: 'u32',
        slot: 'u32',
      },
      equippable: {
        baseId: 'u32',
        slotId: 'u32',
        equippables: 'RmrkTraitsPartEquippableList',
      },
      theme_add: {
        baseId: 'u32',
        theme: 'RmrkTraitsTheme',
      },
      create_base: {
        baseType: 'Bytes',
        symbol: 'Bytes',
        parts: 'Vec<RmrkTraitsPartPartType>'
      }
    }
  },
  /**
   * Lookup125: rmrk_traits::part::EquippableList<frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsPartEquippableList: {
    _enum: {
      All: 'Null',
      Empty: 'Null',
      Custom: 'Vec<u32>'
    }
  },
  /**
   * Lookup127: rmrk_traits::theme::Theme<frame_support::storage::bounded_vec::BoundedVec<T, S>, PropertyList>
   **/
  RmrkTraitsTheme: {
    name: 'Bytes',
    properties: 'Vec<RmrkTraitsThemeThemeProperty>',
    inherit: 'bool'
  },
  /**
   * Lookup129: rmrk_traits::theme::ThemeProperty<frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsThemeThemeProperty: {
    key: 'Bytes',
    value: 'Bytes'
  },
  /**
   * Lookup131: rmrk_traits::part::PartType<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsPartPartType: {
    _enum: {
      FixedPart: 'RmrkTraitsPartFixedPart',
      SlotPart: 'RmrkTraitsPartSlotPart'
    }
  },
  /**
   * Lookup132: rmrk_traits::part::FixedPart<frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsPartFixedPart: {
    id: 'u32',
    z: 'u32',
    src: 'Bytes'
  },
  /**
   * Lookup133: rmrk_traits::part::SlotPart<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsPartSlotPart: {
    id: 'u32',
    equippable: 'RmrkTraitsPartEquippableList',
    src: 'Bytes',
    z: 'u32'
  },
  /**
   * Lookup135: pallet_rmrk_core::pallet::Call<T>
   **/
  PalletRmrkCoreCall: {
    _enum: {
      mint_nft: {
        owner: 'AccountId32',
        collectionId: 'u32',
        royaltyRecipient: 'Option<AccountId32>',
        royalty: 'Option<Permill>',
        metadata: 'Bytes',
        transferable: 'bool',
      },
      create_collection: {
        metadata: 'Bytes',
        max: 'Option<u32>',
        symbol: 'Bytes',
      },
      burn_nft: {
        collectionId: 'u32',
        nftId: 'u32',
        maxBurns: 'u32',
      },
      destroy_collection: {
        collectionId: 'u32',
      },
      send: {
        collectionId: 'u32',
        nftId: 'u32',
        newOwner: 'RmrkTraitsNftAccountIdOrCollectionNftTuple',
      },
      accept_nft: {
        collectionId: 'u32',
        nftId: 'u32',
        newOwner: 'RmrkTraitsNftAccountIdOrCollectionNftTuple',
      },
      reject_nft: {
        collectionId: 'u32',
        nftId: 'u32',
        maxBurns: 'u32',
      },
      change_collection_issuer: {
        collectionId: 'u32',
        newIssuer: 'MultiAddress',
      },
      set_property: {
        collectionId: 'Compact<u32>',
        maybeNftId: 'Option<u32>',
        key: 'Bytes',
        value: 'Bytes',
      },
      lock_collection: {
        collectionId: 'u32',
      },
      add_basic_resource: {
        collectionId: 'u32',
        nftId: 'u32',
        resource: 'RmrkTraitsResourceBasicResource',
      },
      add_composable_resource: {
        collectionId: 'u32',
        nftId: 'u32',
        resource: 'RmrkTraitsResourceComposableResource',
      },
      add_slot_resource: {
        collectionId: 'u32',
        nftId: 'u32',
        resource: 'RmrkTraitsResourceSlotResource',
      },
      accept_resource: {
        collectionId: 'u32',
        nftId: 'u32',
        resourceId: 'u32',
      },
      remove_resource: {
        collectionId: 'u32',
        nftId: 'u32',
        resourceId: 'u32',
      },
      accept_resource_removal: {
        collectionId: 'u32',
        nftId: 'u32',
        resourceId: 'u32',
      },
      set_priority: {
        collectionId: 'u32',
        nftId: 'u32',
        priorities: 'Vec<u32>'
      }
    }
  },
  /**
   * Lookup139: rmrk_traits::resource::BasicResource<frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsResourceBasicResource: {
    src: 'Option<Bytes>',
    metadata: 'Option<Bytes>',
    license: 'Option<Bytes>',
    thumb: 'Option<Bytes>'
  },
  /**
   * Lookup141: rmrk_traits::resource::ComposableResource<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsResourceComposableResource: {
    parts: 'Vec<u32>',
    base: 'u32',
    src: 'Option<Bytes>',
    metadata: 'Option<Bytes>',
    license: 'Option<Bytes>',
    thumb: 'Option<Bytes>'
  },
  /**
   * Lookup143: rmrk_traits::resource::SlotResource<frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsResourceSlotResource: {
    base: 'u32',
    src: 'Option<Bytes>',
    metadata: 'Option<Bytes>',
    slot: 'u32',
    license: 'Option<Bytes>',
    thumb: 'Option<Bytes>'
  },
  /**
   * Lookup145: pallet_rmrk_market::pallet::Call<T>
   **/
  PalletRmrkMarketCall: {
    _enum: {
      buy: {
        collectionId: 'u32',
        nftId: 'u32',
        amount: 'Option<u128>',
      },
      list: {
        collectionId: 'u32',
        nftId: 'u32',
        amount: 'u128',
        expires: 'Option<u32>',
      },
      unlist: {
        collectionId: 'u32',
        nftId: 'u32',
      },
      make_offer: {
        collectionId: 'u32',
        nftId: 'u32',
        amount: 'u128',
        expires: 'Option<u32>',
      },
      withdraw_offer: {
        collectionId: 'u32',
        nftId: 'u32',
      },
      accept_offer: {
        collectionId: 'u32',
        nftId: 'u32',
        offerer: 'AccountId32'
      }
    }
  },
  /**
   * Lookup146: pallet_uniques::pallet::Call<T, I>
   **/
  PalletUniquesCall: {
    _enum: {
      create: {
        class: 'u32',
        admin: 'MultiAddress',
      },
      force_create: {
        class: 'u32',
        owner: 'MultiAddress',
        freeHolding: 'bool',
      },
      destroy: {
        class: 'u32',
        witness: 'PalletUniquesDestroyWitness',
      },
      mint: {
        class: 'u32',
        instance: 'u32',
        owner: 'MultiAddress',
      },
      burn: {
        class: 'u32',
        instance: 'u32',
        checkOwner: 'Option<MultiAddress>',
      },
      transfer: {
        class: 'u32',
        instance: 'u32',
        dest: 'MultiAddress',
      },
      redeposit: {
        class: 'u32',
        instances: 'Vec<u32>',
      },
      freeze: {
        class: 'u32',
        instance: 'u32',
      },
      thaw: {
        class: 'u32',
        instance: 'u32',
      },
      freeze_class: {
        class: 'u32',
      },
      thaw_class: {
        class: 'u32',
      },
      transfer_ownership: {
        class: 'u32',
        owner: 'MultiAddress',
      },
      set_team: {
        class: 'u32',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
      },
      approve_transfer: {
        class: 'u32',
        instance: 'u32',
        delegate: 'MultiAddress',
      },
      cancel_approval: {
        class: 'u32',
        instance: 'u32',
        maybeCheckDelegate: 'Option<MultiAddress>',
      },
      force_asset_status: {
        class: 'u32',
        owner: 'MultiAddress',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
        freeHolding: 'bool',
        isFrozen: 'bool',
      },
      set_attribute: {
        class: 'u32',
        maybeInstance: 'Option<u32>',
        key: 'Bytes',
        value: 'Bytes',
      },
      clear_attribute: {
        class: 'u32',
        maybeInstance: 'Option<u32>',
        key: 'Bytes',
      },
      set_metadata: {
        class: 'u32',
        instance: 'u32',
        data: 'Bytes',
        isFrozen: 'bool',
      },
      clear_metadata: {
        class: 'u32',
        instance: 'u32',
      },
      set_class_metadata: {
        class: 'u32',
        data: 'Bytes',
        isFrozen: 'bool',
      },
      clear_class_metadata: {
        class: 'u32',
      },
      set_accept_ownership: {
        maybeClass: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup147: pallet_uniques::types::DestroyWitness
   **/
  PalletUniquesDestroyWitness: {
    instances: 'Compact<u32>',
    instanceMetadatas: 'Compact<u32>',
    attributes: 'Compact<u32>'
  },
  /**
   * Lookup149: pallet_utility::pallet::Call<T>
   **/
  PalletUtilityCall: {
    _enum: {
      batch: {
        calls: 'Vec<Call>',
      },
      as_derivative: {
        index: 'u16',
        call: 'Call',
      },
      batch_all: {
        calls: 'Vec<Call>',
      },
      dispatch_as: {
        asOrigin: 'RmrkSubstrateRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>'
      }
    }
  },
  /**
   * Lookup151: rmrk_substrate_runtime::OriginCaller
   **/
  RmrkSubstrateRuntimeOriginCaller: {
    _enum: {
      system: 'FrameSupportDispatchRawOrigin',
      Void: 'SpCoreVoid'
    }
  },
  /**
   * Lookup152: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup153: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup154: pallet_template::pallet::Error<T>
   **/
  PalletTemplateError: {
    _enum: ['NoneValue', 'StorageOverflow']
  },
  /**
   * Lookup155: rmrk_traits::base::BaseInfo<sp_core::crypto::AccountId32, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsBaseBaseInfo: {
    issuer: 'AccountId32',
    baseType: 'Bytes',
    symbol: 'Bytes'
  },
  /**
   * Lookup158: pallet_rmrk_equip::pallet::Error<T>
   **/
  PalletRmrkEquipError: {
    _enum: ['PermissionError', 'ItemDoesntExist', 'EquipperDoesntExist', 'NoAvailableBaseId', 'NoAvailablePartId', 'MustBeDirectParent', 'PartDoesntExist', 'BaseDoesntExist', 'CantEquipFixedPart', 'NoResourceForThisBaseFoundOnNft', 'CollectionNotEquippable', 'ItemHasNoResourceToEquipThere', 'NoEquippableOnFixedPart', 'NeedsDefaultThemeFirst', 'AlreadyEquipped', 'UnknownError', 'ExceedsMaxPartsPerBase', 'TooManyProperties']
  },
  /**
   * Lookup159: rmrk_traits::collection::CollectionInfo<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>, sp_core::crypto::AccountId32>
   **/
  RmrkTraitsCollectionCollectionInfo: {
    issuer: 'AccountId32',
    metadata: 'Bytes',
    max: 'Option<u32>',
    symbol: 'Bytes',
    nftsCount: 'u32'
  },
  /**
   * Lookup160: rmrk_traits::nft::NftInfo<sp_core::crypto::AccountId32, sp_arithmetic::per_things::Permill, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsNftNftInfo: {
    owner: 'RmrkTraitsNftAccountIdOrCollectionNftTuple',
    royalty: 'Option<RmrkTraitsNftRoyaltyInfo>',
    metadata: 'Bytes',
    equipped: 'bool',
    pending: 'bool',
    transferable: 'bool'
  },
  /**
   * Lookup162: rmrk_traits::nft::RoyaltyInfo<sp_core::crypto::AccountId32, sp_arithmetic::per_things::Permill>
   **/
  RmrkTraitsNftRoyaltyInfo: {
    recipient: 'AccountId32',
    amount: 'Permill'
  },
  /**
   * Lookup165: rmrk_traits::resource::ResourceInfo<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsResourceResourceInfo: {
    id: 'u32',
    resource: 'RmrkTraitsResourceResourceTypes',
    pending: 'bool',
    pendingRemoval: 'bool'
  },
  /**
   * Lookup166: rmrk_traits::resource::ResourceTypes<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsResourceResourceTypes: {
    _enum: {
      Basic: 'RmrkTraitsResourceBasicResource',
      Composable: 'RmrkTraitsResourceComposableResource',
      Slot: 'RmrkTraitsResourceSlotResource'
    }
  },
  /**
   * Lookup170: rmrk_traits::nft::NftChild
   **/
  RmrkTraitsNftNftChild: {
    collectionId: 'u32',
    nftId: 'u32'
  },
  /**
   * Lookup171: PhantomType::up_data_structs<rmrk_traits::property::PropertyInfo<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>>
   **/
  PhantomTypeUpDataStructs: '[Lookup172;0]',
  /**
   * Lookup172: rmrk_traits::property::PropertyInfo<frame_support::storage::bounded_vec::BoundedVec<T, S>, frame_support::storage::bounded_vec::BoundedVec<T, S>>
   **/
  RmrkTraitsPropertyPropertyInfo: {
    key: 'Bytes',
    value: 'Bytes'
  },
  /**
   * Lookup174: pallet_rmrk_core::pallet::Error<T>
   **/
  PalletRmrkCoreError: {
    _enum: ['NoneValue', 'StorageOverflow', 'TooLong', 'NoAvailableCollectionId', 'NoAvailableResourceId', 'MetadataNotSet', 'RecipientNotSet', 'NoAvailableNftId', 'NotInRange', 'RoyaltyNotSet', 'CollectionUnknown', 'NoPermission', 'NoWitness', 'CollectionNotEmpty', 'CollectionFullOrLocked', 'CannotSendToDescendentOrSelf', 'ResourceAlreadyExists', 'EmptyResource', 'TooManyRecursions', 'NftIsLocked', 'CannotAcceptNonOwnedNft', 'CannotRejectNonOwnedNft', 'ResourceDoesntExist', 'ResourceNotPending', 'NonTransferable']
  },
  /**
   * Lookup175: pallet_rmrk_market::types::ListInfo<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  PalletRmrkMarketListInfo: {
    listedBy: 'AccountId32',
    amount: 'u128',
    expires: 'Option<u32>'
  },
  /**
   * Lookup177: pallet_rmrk_market::types::Offer<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  PalletRmrkMarketOffer: {
    maker: 'AccountId32',
    amount: 'u128',
    expires: 'Option<u32>'
  },
  /**
   * Lookup178: pallet_rmrk_market::pallet::Error<T>
   **/
  PalletRmrkMarketError: {
    _enum: ['NoPermission', 'TokenNotForSale', 'CannotWithdrawOffer', 'CannotUnlistToken', 'CannotOfferOnOwnToken', 'CannotBuyOwnToken', 'UnknownOffer', 'CannotListNftOwnedByNft', 'TokenDoesNotExist', 'OfferTooLow', 'AlreadyOffered', 'OfferHasExpired', 'ListingHasExpired', 'PriceDiffersFromExpected', 'NonTransferable']
  },
  /**
   * Lookup179: pallet_uniques::types::ClassDetails<sp_core::crypto::AccountId32, DepositBalance>
   **/
  PalletUniquesClassDetails: {
    owner: 'AccountId32',
    issuer: 'AccountId32',
    admin: 'AccountId32',
    freezer: 'AccountId32',
    totalDeposit: 'u128',
    freeHolding: 'bool',
    instances: 'u32',
    instanceMetadatas: 'u32',
    attributes: 'u32',
    isFrozen: 'bool'
  },
  /**
   * Lookup182: pallet_uniques::types::InstanceDetails<sp_core::crypto::AccountId32, DepositBalance>
   **/
  PalletUniquesInstanceDetails: {
    owner: 'AccountId32',
    approved: 'Option<AccountId32>',
    isFrozen: 'bool',
    deposit: 'u128'
  },
  /**
   * Lookup183: pallet_uniques::types::ClassMetadata<DepositBalance, StringLimit>
   **/
  PalletUniquesClassMetadata: {
    deposit: 'u128',
    data: 'Bytes',
    isFrozen: 'bool'
  },
  /**
   * Lookup184: pallet_uniques::types::InstanceMetadata<DepositBalance, StringLimit>
   **/
  PalletUniquesInstanceMetadata: {
    deposit: 'u128',
    data: 'Bytes',
    isFrozen: 'bool'
  },
  /**
   * Lookup186: pallet_uniques::pallet::Error<T, I>
   **/
  PalletUniquesError: {
    _enum: ['NoPermission', 'UnknownClass', 'AlreadyExists', 'WrongOwner', 'BadWitness', 'InUse', 'Frozen', 'WrongDelegate', 'NoDelegate', 'Unapproved', 'Unaccepted', 'Locked']
  },
  /**
   * Lookup187: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup189: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup190: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup191: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup194: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup195: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup196: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup199: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup200: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup201: pallet_transaction_payment::ChargeTransactionPayment<T>
   **/
  PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
  /**
   * Lookup202: rmrk_substrate_runtime::Runtime
   **/
  RmrkSubstrateRuntimeRuntime: 'Null'
};
