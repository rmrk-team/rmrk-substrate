// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Bytes, Compact, Option, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, MultiAddress, Perbill, Permill } from '@polkadot/types/interfaces/runtime';
import type { PalletUniquesDestroyWitness, RmrkSubstrateRuntimeOriginCaller, RmrkTraitsNftAccountIdOrCollectionNftTuple, RmrkTraitsPartEquippableList, RmrkTraitsPartPartType, RmrkTraitsResourceBasicResource, RmrkTraitsResourceComposableResource, RmrkTraitsResourceResourceInfoMin, RmrkTraitsResourceResourceTypes, RmrkTraitsResourceSlotResource, RmrkTraitsTheme, SpCoreVoid, SpFinalityGrandpaEquivocationProof, SpWeightsWeightV2Weight } from '@polkadot/types/lookup';

declare module '@polkadot/api-base/types/submittable' {
  export interface AugmentedSubmittables<ApiType extends ApiTypes> {
    balances: {
      /**
       * Exactly as `transfer`, except the origin must be root and the source account may be
       * specified.
       * # <weight>
       * - Same as transfer, but additional read and write because the source account is not
       * assumed to be in the overlay.
       * # </weight>
       **/
      forceTransfer: AugmentedSubmittable<(source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * Unreserve some balance from a user by force.
       * 
       * Can only be called by ROOT.
       **/
      forceUnreserve: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, u128]>;
      /**
       * Set the balances of a given account.
       * 
       * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
       * also alter the total issuance of the system (`TotalIssuance`) appropriately.
       * If the new free or reserved balance is below the existential deposit,
       * it will reset the account nonce (`frame_system::AccountNonce`).
       * 
       * The dispatch origin for this call is `root`.
       **/
      setBalance: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array, newReserved: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>, Compact<u128>]>;
      /**
       * Transfer some liquid free balance to another account.
       * 
       * `transfer` will set the `FreeBalance` of the sender and receiver.
       * If the sender's account is below the existential deposit as a result
       * of the transfer, the account will be reaped.
       * 
       * The dispatch origin for this call must be `Signed` by the transactor.
       * 
       * # <weight>
       * - Dependent on arguments but not critical, given proper implementations for input config
       * types. See related functions below.
       * - It contains a limited number of reads and writes internally and no complex
       * computation.
       * 
       * Related functions:
       * 
       * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
       * - Transferring balances to accounts that did not exist before will cause
       * `T::OnNewAccount::on_new_account` to be called.
       * - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
       * - `transfer_keep_alive` works the same way as `transfer`, but has an additional check
       * that the transfer will not kill the origin account.
       * ---------------------------------
       * - Origin account is already in memory, so no DB operations for them.
       * # </weight>
       **/
      transfer: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * Transfer the entire transferable balance from the caller account.
       * 
       * NOTE: This function only attempts to transfer _transferable_ balances. This means that
       * any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be
       * transferred by this function. To ensure that this function results in a killed account,
       * you might need to prepare the account by removing any reference counters, storage
       * deposits, etc...
       * 
       * The dispatch origin of this call must be Signed.
       * 
       * - `dest`: The recipient of the transfer.
       * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all
       * of the funds the account has, causing the sender account to be killed (false), or
       * transfer everything except at least the existential deposit, which will guarantee to
       * keep the sender account alive (true). # <weight>
       * - O(1). Just like transfer, but reading the user's transferable balance first.
       * #</weight>
       **/
      transferAll: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, bool]>;
      /**
       * Same as the [`transfer`] call, but with a check that the transfer will not kill the
       * origin account.
       * 
       * 99% of the time you want [`transfer`] instead.
       * 
       * [`transfer`]: struct.Pallet.html#method.transfer
       **/
      transferKeepAlive: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    grandpa: {
      /**
       * Note that the current authority set of the GRANDPA finality gadget has stalled.
       * 
       * This will trigger a forced authority set change at the beginning of the next session, to
       * be enacted `delay` blocks after that. The `delay` should be high enough to safely assume
       * that the block signalling the forced change will not be re-orged e.g. 1000 blocks.
       * The block production rate (which may be slowed down because of finality lagging) should
       * be taken into account when choosing the `delay`. The GRANDPA voters based on the new
       * authority will start voting on top of `best_finalized_block_number` for new finalized
       * blocks. `best_finalized_block_number` should be the highest of the latest finalized
       * block of all validators of the new authority set.
       * 
       * Only callable by root.
       **/
      noteStalled: AugmentedSubmittable<(delay: u32 | AnyNumber | Uint8Array, bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence
       * will be reported.
       **/
      reportEquivocation: AugmentedSubmittable<(equivocationProof: SpFinalityGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpCoreVoid | null) => SubmittableExtrinsic<ApiType>, [SpFinalityGrandpaEquivocationProof, SpCoreVoid]>;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence
       * will be reported.
       * 
       * This extrinsic must be called unsigned and it is expected that only
       * block authors will call it (validated in `ValidateUnsigned`), as such
       * if the block author is defined it will be defined as the equivocation
       * reporter.
       **/
      reportEquivocationUnsigned: AugmentedSubmittable<(equivocationProof: SpFinalityGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpCoreVoid | null) => SubmittableExtrinsic<ApiType>, [SpFinalityGrandpaEquivocationProof, SpCoreVoid]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rmrkCore: {
      /**
       * Accepts an NFT sent from another account to self or owned NFT
       * 
       * Parameters:
       * - `origin`: sender of the transaction
       * - `collection_id`: collection id of the nft to be accepted
       * - `nft_id`: nft id of the nft to be accepted
       * - `new_owner`: either origin's account ID or origin-owned NFT, whichever the NFT was
       * sent to
       **/
      acceptNft: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, newOwner: RmrkTraitsNftAccountIdOrCollectionNftTuple | { AccountId: any } | { CollectionAndNftTuple: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsNftAccountIdOrCollectionNftTuple]>;
      /**
       * accept the addition of a new resource to an existing NFT
       **/
      acceptResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * accept the removal of a resource of an existing NFT
       **/
      acceptResourceRemoval: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * Create basic resource
       **/
      addBasicResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceBasicResource | { metadata?: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceBasicResource, u32]>;
      /**
       * Create composable resource
       **/
      addComposableResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceComposableResource | { parts?: any; base?: any; metadata?: any; slot?: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceComposableResource, u32]>;
      /**
       * Create slot resource
       **/
      addSlotResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceSlotResource | { base?: any; metadata?: any; slot?: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceSlotResource, u32]>;
      /**
       * burn nft
       **/
      burnNft: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Change the issuer of a collection
       * 
       * Parameters:
       * - `origin`: sender of the transaction
       * - `collection_id`: collection id of the nft to change issuer of
       * - `new_issuer`: Collection's new issuer
       **/
      changeCollectionIssuer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, newIssuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * Create a collection
       **/
      createCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, metadata: Bytes | string | Uint8Array, max: Option<u32> | null | object | string | Uint8Array, symbol: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, Option<u32>, Bytes]>;
      /**
       * destroy collection
       **/
      destroyCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * lock collection
       **/
      lockCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Mints an NFT in the specified collection
       * Sets metadata and the royalty attribute
       * 
       * Parameters:
       * - `collection_id`: The collection of the asset to be minted.
       * - `nft_id`: The nft value of the asset to be minted.
       * - `recipient`: Receiver of the royalty
       * - `royalty`: Permillage reward from each trade for the Recipient
       * - `metadata`: Arbitrary data about an nft, e.g. IPFS hash
       **/
      mintNft: AugmentedSubmittable<(owner: Option<AccountId32> | null | object | string | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, royaltyRecipient: Option<AccountId32> | null | object | string | Uint8Array, royalty: Option<Permill> | null | object | string | Uint8Array, metadata: Bytes | string | Uint8Array, transferable: bool | boolean | Uint8Array, resources: Option<Vec<RmrkTraitsResourceResourceInfoMin>> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<AccountId32>, u32, u32, Option<AccountId32>, Option<Permill>, Bytes, bool, Option<Vec<RmrkTraitsResourceResourceInfoMin>>]>;
      /**
       * Mints an NFT in the specified collection directly to another NFT
       * Sets metadata and the royalty attribute
       * 
       * Parameters:
       * - `collection_id`: The class of the asset to be minted.
       * - `nft_id`: The nft value of the asset to be minted.
       * - `recipient`: Receiver of the royalty
       * - `royalty`: Permillage reward from each trade for the Recipient
       * - `metadata`: Arbitrary data about an nft, e.g. IPFS hash
       **/
      mintNftDirectlyToNft: AugmentedSubmittable<(owner: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], nftId: u32 | AnyNumber | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, royaltyRecipient: Option<AccountId32> | null | object | string | Uint8Array, royalty: Option<Permill> | null | object | string | Uint8Array, metadata: Bytes | string | Uint8Array, transferable: bool | boolean | Uint8Array, resources: Option<Vec<RmrkTraitsResourceResourceInfoMin>> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [ITuple<[u32, u32]>, u32, u32, Option<AccountId32>, Option<Permill>, Bytes, bool, Option<Vec<RmrkTraitsResourceResourceInfoMin>>]>;
      /**
       * Rejects an NFT sent from another account to self or owned NFT
       * 
       * Parameters:
       * - `origin`: sender of the transaction
       * - `collection_id`: collection id of the nft to be accepted
       * - `nft_id`: nft id of the nft to be accepted
       **/
      rejectNft: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * remove resource
       **/
      removeResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * Replace resource by id
       **/
      replaceResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceResourceTypes | { Basic: any } | { Composable: any } | { Slot: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceResourceTypes, u32]>;
      /**
       * Transfers a NFT from an Account or NFT A to another Account or NFT B
       * 
       * Parameters:
       * - `origin`: sender of the transaction
       * - `collection_id`: collection id of the nft to be transferred
       * - `nft_id`: nft id of the nft to be transferred
       * - `new_owner`: new owner of the nft which can be either an account or a NFT
       **/
      send: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, newOwner: RmrkTraitsNftAccountIdOrCollectionNftTuple | { AccountId: any } | { CollectionAndNftTuple: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsNftAccountIdOrCollectionNftTuple]>;
      /**
       * set a different order of resource priority
       **/
      setPriority: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, priorities: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [u32, u32, Vec<u32>]>;
      /**
       * set a custom value on an NFT
       **/
      setProperty: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, maybeNftId: Option<u32> | null | object | string | Uint8Array, key: Bytes | string | Uint8Array, value: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<u32>, Bytes, Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rmrkEquip: {
      /**
       * Change the issuer of a Base
       * 
       * Parameters:
       * - `origin`: sender of the transaction
       * - `base_id`: base_id to change issuer of
       * - `new_issuer`: Base's new issuer
       **/
      changeBaseIssuer: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, newIssuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * Creates a new Base.
       * Modeled after [base interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/base.md)
       * 
       * Parameters:
       * - origin: Caller, will be assigned as the issuer of the Base
       * - base_type: media type, e.g. "svg"
       * - symbol: arbitrary client-chosen symbol, e.g. "kanaria_superbird"
       * - parts: array of Fixed and Slot parts composing the base, confined in length by
       * PartsLimit
       **/
      createBase: AugmentedSubmittable<(baseType: Bytes | string | Uint8Array, symbol: Bytes | string | Uint8Array, parts: Vec<RmrkTraitsPartPartType> | (RmrkTraitsPartPartType | { FixedPart: any } | { SlotPart: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Bytes, Bytes, Vec<RmrkTraitsPartPartType>]>;
      /**
       * Equips a child NFT's resource to a parent's slot, if all are available.
       * Equipping operations are maintained inside the Equippings storage.
       * Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
       * 
       * Parameters:
       * - origin: The caller of the function, not necessarily anything else
       * - item: Child NFT being equipped
       * - equipper: Parent NFT which will equip the item
       * - base: ID of the base which the item and equipper must each have a resource referencing
       * - slot: ID of the slot which the item and equipper must each have a resource referencing
       **/
      equip: AugmentedSubmittable<(item: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], equipper: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], resourceId: u32 | AnyNumber | Uint8Array, base: u32 | AnyNumber | Uint8Array, slot: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [ITuple<[u32, u32]>, ITuple<[u32, u32]>, u32, u32, u32]>;
      /**
       * Updates the array of Collections allowed to be equipped to a Base's specified Slot Part.
       * Modeled after [equippable interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equippable.md)
       * 
       * Parameters:
       * - origin: The caller of the function, must be issuer of the base
       * - base_id: The Base containing the Slot Part to be updated
       * - part_id: The Slot Part whose Equippable List is being updated
       * - equippables: The list of equippables that will override the current Equippaables list
       **/
      equippable: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, slotId: u32 | AnyNumber | Uint8Array, equippables: RmrkTraitsPartEquippableList | { All: any } | { Empty: any } | { Custom: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsPartEquippableList]>;
      /**
       * Adds a new collection that is allowed to be equipped to a Base's specified Slot Part.
       * 
       * Parameters:
       * - origin: The caller of the function, must be issuer of the base
       * - base_id: The Base containing the Slot Part to be updated
       * - part_id: The Slot Part whose Equippable List is being updated
       * - equippable: The equippable that will be added to the current Equippaables list
       **/
      equippableAdd: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, slotId: u32 | AnyNumber | Uint8Array, equippable: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * Remove a collection from the equippables list.
       * 
       * Parameters:
       * - origin: The caller of the function, must be issuer of the base
       * - base_id: The Base containing the Slot Part to be updated
       * - part_id: The Slot Part whose Equippable List is being updated
       * - equippable: The equippable that will be removed from the current Equippaables list
       **/
      equippableRemove: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, slotId: u32 | AnyNumber | Uint8Array, equippable: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * Adds a Theme to a Base.
       * Modeled after [themeadd interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/themeadd.md)
       * Themes are stored in the Themes storage
       * A Theme named "default" is required prior to adding other Themes.
       * 
       * Parameters:
       * - origin: The caller of the function, must be issuer of the base
       * - base_id: The Base containing the Theme to be updated
       * - theme: The Theme to add to the Base.  A Theme has a name and properties, which are an
       * array of [key, value, inherit].  This array is bounded by MaxPropertiesPerTheme.
       * - key: arbitrary BoundedString, defined by client
       * - value: arbitrary BoundedString, defined by client
       * - inherit: optional bool
       **/
      themeAdd: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, theme: RmrkTraitsTheme | { name?: any; properties?: any; inherit?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, RmrkTraitsTheme]>;
      /**
       * Unequips a child NFT's resource from its parent's slot.
       * Can be successful if
       * - Item has beeen burned
       * - Item is equipped and extrinsic called by equipping item owner
       * - Item is equipped and extrinsic called by equipper NFT owner
       * Equipping operations are maintained inside the Equippings storage.
       * Modeled after [equip interaction](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md)
       * 
       * Parameters:
       * - origin: The caller of the function, not necessarily anything else
       * - item: Child NFT being unequipped
       * - unequipper: Parent NFT which will unequip the item
       * - base: ID of the base which the item and equipper must each have a resource referencing
       * - slot: ID of the slot which the item and equipper must each have a resource referencing
       **/
      unequip: AugmentedSubmittable<(item: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], unequipper: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], base: u32 | AnyNumber | Uint8Array, slot: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [ITuple<[u32, u32]>, ITuple<[u32, u32]>, u32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rmrkMarket: {
      acceptOffer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, offerer: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, AccountId32]>;
      /**
       * Buy a listed NFT. Ensure that the NFT is available for purchase and has not recently
       * been purchased, sent, or burned.
       * 
       * Parameters:
       * - `origin` - Account of the potential buyer
       * - `collection_id` - Collection id of the RMRK NFT
       * - `nft_id` - NFT id of the RMRK NFT
       * - `amount` - Optional price at which buyer purchased at
       **/
      buy: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, amount: Option<u128> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<u128>]>;
      /**
       * List a RMRK NFT on the Marketplace for purchase. A listing can be cancelled, and is
       * automatically considered cancelled when a `buy` is executed on top of a given listing.
       * An NFT that has another NFT as its owner CANNOT be listed. An NFT owned by a NFT must
       * first be sent to an account before being listed.
       * 
       * Parameters:
       * - `origin` - Account of owner of the RMRK NFT to be listed
       * - `collection_id` - Collection id of the RMRK NFT
       * - `nft_id` - NFT id of the RMRK NFT
       * - `amount` - Price of the RMRK NFT
       * - `expires` - Optional BlockNumber for when the listing expires
       **/
      list: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, amount: u128 | AnyNumber | Uint8Array, expires: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128, Option<u32>]>;
      /**
       * Make an offer on a RMRK NFT for purchase. An offer can be set with an expiration where
       * the offer can no longer be accepted by the RMRK NFT owner
       * 
       * Parameters:
       * - `origin` - Account of the potential buyer
       * - `collection_id` - Collection id of the RMRK NFT
       * - `nft_id` - NFT id of the RMRK NFT
       * - `amount` - Price of the RMRK NFT
       * - `expiration` - Expiration of the offer
       **/
      makeOffer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, amount: u128 | AnyNumber | Uint8Array, expires: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128, Option<u32>]>;
      /**
       * Unlist a RMRK NFT on the Marketplace and remove from storage in `Listings`.
       * 
       * Parameters:
       * - `origin` - Account owner of the listed RMRK NFT
       * - `collection_id` - Collection id of the RMRK NFT
       * - `nft_id` - NFT id of the RMRK NFT
       **/
      unlist: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Withdraw an offer on a RMRK NFT, such that it is no longer available to be accepted by
       * the NFT owner
       * 
       * Parameters:
       * - `origin` - Account that wants to withdraw their offer
       * - `collection_id` - Collection id of the RMRK NFT
       * - `nft_id` - NFT id of the RMRK NFT
       **/
      withdrawOffer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
       * key.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB change.
       * # </weight>
       **/
      setKey: AugmentedSubmittable<(updated: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       * # </weight>
       **/
      sudo: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Signed` origin from
       * a given account.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       * # </weight>
       **/
      sudoAs: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Call]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       * This function does not check the weight of the call, and instead allows the
       * Sudo user to specify the weight of the call.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - The weight of this call is defined by the caller.
       * # </weight>
       **/
      sudoUncheckedWeight: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array, weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, SpWeightsWeightV2Weight]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    system: {
      /**
       * A dispatch that will fill the block weight up to the given ratio.
       **/
      fillBlock: AugmentedSubmittable<(ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Perbill]>;
      /**
       * Kill all storage items with a key that starts with the given prefix.
       * 
       * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
       * the prefix we are removing to accurately calculate the weight of this function.
       **/
      killPrefix: AugmentedSubmittable<(prefix: Bytes | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, u32]>;
      /**
       * Kill some items from storage.
       **/
      killStorage: AugmentedSubmittable<(keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Bytes>]>;
      /**
       * Make some on-chain remark.
       * 
       * # <weight>
       * - `O(1)`
       * # </weight>
       **/
      remark: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Make some on-chain remark and emit event.
       **/
      remarkWithEvent: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new runtime code.
       * 
       * # <weight>
       * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
       * - 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is
       * expensive).
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event.
       * The weight of this function is dependent on the runtime, but generally this is very
       * expensive. We will treat this as a full block.
       * # </weight>
       **/
      setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new runtime code without doing any checks of the given `code`.
       * 
       * # <weight>
       * - `O(C)` where `C` length of `code`
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event.
       * The weight of this function is dependent on the runtime. We will treat this as a full
       * block. # </weight>
       **/
      setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the number of pages in the WebAssembly environment's heap.
       **/
      setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * Set some items of storage.
       **/
      setStorage: AugmentedSubmittable<(items: Vec<ITuple<[Bytes, Bytes]>> | ([Bytes | string | Uint8Array, Bytes | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[Bytes, Bytes]>>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    templateModule: {
      /**
       * An example dispatchable that may throw a custom error.
       **/
      causeError: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * An example dispatchable that takes a singles value as a parameter, writes the value to
       * storage and emits an event. This function must be dispatched by a signed extrinsic.
       **/
      doSomething: AugmentedSubmittable<(something: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    timestamp: {
      /**
       * Set the current time.
       * 
       * This call should be invoked exactly once per block. It will panic at the finalization
       * phase, if this call hasn't been invoked by that time.
       * 
       * The timestamp should be greater than the previous one by the amount specified by
       * `MinimumPeriod`.
       * 
       * The dispatch origin for this call must be `Inherent`.
       * 
       * # <weight>
       * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
       * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in
       * `on_finalize`)
       * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
       * # </weight>
       **/
      set: AugmentedSubmittable<(now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u64>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    uniques: {
      /**
       * Approve an item to be transferred by a delegated third-party account.
       * 
       * The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be
       * either the owner of the `item` or the admin of the collection.
       * 
       * - `collection`: The collection of the item to be approved for delegated transfer.
       * - `item`: The item of the item to be approved for delegated transfer.
       * - `delegate`: The account to delegate permission to transfer the item.
       * 
       * Important NOTE: The `approved` account gets reset after each transfer.
       * 
       * Emits `ApprovedTransfer` on success.
       * 
       * Weight: `O(1)`
       **/
      approveTransfer: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, delegate: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, MultiAddress]>;
      /**
       * Destroy a single item.
       * 
       * Origin must be Signed and the sender should be the Admin of the `collection`.
       * 
       * - `collection`: The collection of the item to be burned.
       * - `item`: The item of the item to be burned.
       * - `check_owner`: If `Some` then the operation will fail with `WrongOwner` unless the
       * item is owned by this value.
       * 
       * Emits `Burned` with the actual amount burned.
       * 
       * Weight: `O(1)`
       * Modes: `check_owner.is_some()`.
       **/
      burn: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, checkOwner: Option<MultiAddress> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<MultiAddress>]>;
      /**
       * Allows to buy an item if it's up for sale.
       * 
       * Origin must be Signed and must not be the owner of the `item`.
       * 
       * - `collection`: The collection of the item.
       * - `item`: The item the sender wants to buy.
       * - `bid_price`: The price the sender is willing to pay.
       * 
       * Emits `ItemBought` on success.
       **/
      buyItem: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, bidPrice: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128]>;
      /**
       * Cancel the prior approval for the transfer of an item by a delegate.
       * 
       * Origin must be either:
       * - the `Force` origin;
       * - `Signed` with the signer being the Admin of the `collection`;
       * - `Signed` with the signer being the Owner of the `item`;
       * 
       * Arguments:
       * - `collection`: The collection of the item of whose approval will be cancelled.
       * - `item`: The item of the item of whose approval will be cancelled.
       * - `maybe_check_delegate`: If `Some` will ensure that the given account is the one to
       * which permission of transfer is delegated.
       * 
       * Emits `ApprovalCancelled` on success.
       * 
       * Weight: `O(1)`
       **/
      cancelApproval: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, maybeCheckDelegate: Option<MultiAddress> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<MultiAddress>]>;
      /**
       * Clear an attribute for a collection or item.
       * 
       * Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
       * `collection`.
       * 
       * Any deposit is freed for the collection's owner.
       * 
       * - `collection`: The identifier of the collection whose item's metadata to clear.
       * - `maybe_item`: The identifier of the item whose metadata to clear.
       * - `key`: The key of the attribute.
       * 
       * Emits `AttributeCleared`.
       * 
       * Weight: `O(1)`
       **/
      clearAttribute: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, maybeItem: Option<u32> | null | object | string | Uint8Array, key: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<u32>, Bytes]>;
      /**
       * Clear the metadata for a collection.
       * 
       * Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
       * the `collection`.
       * 
       * Any deposit is freed for the collection's owner.
       * 
       * - `collection`: The identifier of the collection whose metadata to clear.
       * 
       * Emits `CollectionMetadataCleared`.
       * 
       * Weight: `O(1)`
       **/
      clearCollectionMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Clear the metadata for an item.
       * 
       * Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
       * `item`.
       * 
       * Any deposit is freed for the collection's owner.
       * 
       * - `collection`: The identifier of the collection whose item's metadata to clear.
       * - `item`: The identifier of the item whose metadata to clear.
       * 
       * Emits `MetadataCleared`.
       * 
       * Weight: `O(1)`
       **/
      clearMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Issue a new collection of non-fungible items from a public origin.
       * 
       * This new collection has no items initially and its owner is the origin.
       * 
       * The origin must conform to the configured `CreateOrigin` and have sufficient funds free.
       * 
       * `ItemDeposit` funds of sender are reserved.
       * 
       * Parameters:
       * - `collection`: The identifier of the new collection. This must not be currently in use.
       * - `admin`: The admin of this collection. The admin is the initial address of each
       * member of the collection's admin team.
       * 
       * Emits `Created` event when successful.
       * 
       * Weight: `O(1)`
       **/
      create: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * Destroy a collection of fungible items.
       * 
       * The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the
       * owner of the `collection`.
       * 
       * - `collection`: The identifier of the collection to be destroyed.
       * - `witness`: Information on the items minted in the collection. This must be
       * correct.
       * 
       * Emits `Destroyed` event when successful.
       * 
       * Weight: `O(n + m)` where:
       * - `n = witness.items`
       * - `m = witness.item_metadatas`
       * - `a = witness.attributes`
       **/
      destroy: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, witness: PalletUniquesDestroyWitness | { items?: any; itemMetadatas?: any; attributes?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletUniquesDestroyWitness]>;
      /**
       * Issue a new collection of non-fungible items from a privileged origin.
       * 
       * This new collection has no items initially.
       * 
       * The origin must conform to `ForceOrigin`.
       * 
       * Unlike `create`, no funds are reserved.
       * 
       * - `collection`: The identifier of the new item. This must not be currently in use.
       * - `owner`: The owner of this collection of items. The owner has full superuser
       * permissions
       * over this item, but may later change and configure the permissions using
       * `transfer_ownership` and `set_team`.
       * 
       * Emits `ForceCreated` event when successful.
       * 
       * Weight: `O(1)`
       **/
      forceCreate: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freeHolding: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, bool]>;
      /**
       * Alter the attributes of a given item.
       * 
       * Origin must be `ForceOrigin`.
       * 
       * - `collection`: The identifier of the item.
       * - `owner`: The new Owner of this item.
       * - `issuer`: The new Issuer of this item.
       * - `admin`: The new Admin of this item.
       * - `freezer`: The new Freezer of this item.
       * - `free_holding`: Whether a deposit is taken for holding an item of this collection.
       * - `is_frozen`: Whether this collection is frozen except for permissioned/admin
       * instructions.
       * 
       * Emits `ItemStatusChanged` with the identity of the item.
       * 
       * Weight: `O(1)`
       **/
      forceItemStatus: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, issuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freezer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freeHolding: bool | boolean | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, MultiAddress, MultiAddress, bool, bool]>;
      /**
       * Disallow further unprivileged transfer of an item.
       * 
       * Origin must be Signed and the sender should be the Freezer of the `collection`.
       * 
       * - `collection`: The collection of the item to be frozen.
       * - `item`: The item of the item to be frozen.
       * 
       * Emits `Frozen`.
       * 
       * Weight: `O(1)`
       **/
      freeze: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Disallow further unprivileged transfers for a whole collection.
       * 
       * Origin must be Signed and the sender should be the Freezer of the `collection`.
       * 
       * - `collection`: The collection to be frozen.
       * 
       * Emits `CollectionFrozen`.
       * 
       * Weight: `O(1)`
       **/
      freezeCollection: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Mint an item of a particular collection.
       * 
       * The origin must be Signed and the sender must be the Issuer of the `collection`.
       * 
       * - `collection`: The collection of the item to be minted.
       * - `item`: The item value of the item to be minted.
       * - `beneficiary`: The initial owner of the minted item.
       * 
       * Emits `Issued` event when successful.
       * 
       * Weight: `O(1)`
       **/
      mint: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, MultiAddress]>;
      /**
       * Reevaluate the deposits on some items.
       * 
       * Origin must be Signed and the sender should be the Owner of the `collection`.
       * 
       * - `collection`: The collection to be frozen.
       * - `items`: The items of the collection whose deposits will be reevaluated.
       * 
       * NOTE: This exists as a best-effort function. Any items which are unknown or
       * in the case that the owner account does not have reservable funds to pay for a
       * deposit increase are ignored. Generally the owner isn't going to call this on items
       * whose existing deposit is less than the refreshed deposit as it would only cost them,
       * so it's of little consequence.
       * 
       * It will still return an error in the case that the collection is unknown of the signer
       * is not permitted to call it.
       * 
       * Weight: `O(items.len())`
       **/
      redeposit: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, items: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [u32, Vec<u32>]>;
      /**
       * Set (or reset) the acceptance of ownership for a particular account.
       * 
       * Origin must be `Signed` and if `maybe_collection` is `Some`, then the signer must have a
       * provider reference.
       * 
       * - `maybe_collection`: The identifier of the collection whose ownership the signer is
       * willing to accept, or if `None`, an indication that the signer is willing to accept no
       * ownership transferal.
       * 
       * Emits `OwnershipAcceptanceChanged`.
       **/
      setAcceptOwnership: AugmentedSubmittable<(maybeCollection: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * Set an attribute for a collection or item.
       * 
       * Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
       * `collection`.
       * 
       * If the origin is Signed, then funds of signer are reserved according to the formula:
       * `MetadataDepositBase + DepositPerByte * (key.len + value.len)` taking into
       * account any already reserved funds.
       * 
       * - `collection`: The identifier of the collection whose item's metadata to set.
       * - `maybe_item`: The identifier of the item whose metadata to set.
       * - `key`: The key of the attribute.
       * - `value`: The value to which to set the attribute.
       * 
       * Emits `AttributeSet`.
       * 
       * Weight: `O(1)`
       **/
      setAttribute: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, maybeItem: Option<u32> | null | object | string | Uint8Array, key: Bytes | string | Uint8Array, value: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<u32>, Bytes, Bytes]>;
      /**
       * Set the maximum amount of items a collection could have.
       * 
       * Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
       * the `collection`.
       * 
       * Note: This function can only succeed once per collection.
       * 
       * - `collection`: The identifier of the collection to change.
       * - `max_supply`: The maximum amount of items a collection could have.
       * 
       * Emits `CollectionMaxSupplySet` event when successful.
       **/
      setCollectionMaxSupply: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, maxSupply: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Set the metadata for a collection.
       * 
       * Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
       * the `collection`.
       * 
       * If the origin is `Signed`, then funds of signer are reserved according to the formula:
       * `MetadataDepositBase + DepositPerByte * data.len` taking into
       * account any already reserved funds.
       * 
       * - `collection`: The identifier of the item whose metadata to update.
       * - `data`: The general information of this item. Limited in length by `StringLimit`.
       * - `is_frozen`: Whether the metadata should be frozen against further changes.
       * 
       * Emits `CollectionMetadataSet`.
       * 
       * Weight: `O(1)`
       **/
      setCollectionMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, data: Bytes | string | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, bool]>;
      /**
       * Set the metadata for an item.
       * 
       * Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
       * `collection`.
       * 
       * If the origin is Signed, then funds of signer are reserved according to the formula:
       * `MetadataDepositBase + DepositPerByte * data.len` taking into
       * account any already reserved funds.
       * 
       * - `collection`: The identifier of the collection whose item's metadata to set.
       * - `item`: The identifier of the item whose metadata to set.
       * - `data`: The general information of this item. Limited in length by `StringLimit`.
       * - `is_frozen`: Whether the metadata should be frozen against further changes.
       * 
       * Emits `MetadataSet`.
       * 
       * Weight: `O(1)`
       **/
      setMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, data: Bytes | string | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Bytes, bool]>;
      /**
       * Set (or reset) the price for an item.
       * 
       * Origin must be Signed and must be the owner of the asset `item`.
       * 
       * - `collection`: The collection of the item.
       * - `item`: The item to set the price for.
       * - `price`: The price for the item. Pass `None`, to reset the price.
       * - `buyer`: Restricts the buy operation to a specific account.
       * 
       * Emits `ItemPriceSet` on success if the price is not `None`.
       * Emits `ItemPriceRemoved` on success if the price is `None`.
       **/
      setPrice: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, price: Option<u128> | null | object | string | Uint8Array, whitelistedBuyer: Option<MultiAddress> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<u128>, Option<MultiAddress>]>;
      /**
       * Change the Issuer, Admin and Freezer of a collection.
       * 
       * Origin must be Signed and the sender should be the Owner of the `collection`.
       * 
       * - `collection`: The collection whose team should be changed.
       * - `issuer`: The new Issuer of this collection.
       * - `admin`: The new Admin of this collection.
       * - `freezer`: The new Freezer of this collection.
       * 
       * Emits `TeamChanged`.
       * 
       * Weight: `O(1)`
       **/
      setTeam: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, issuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freezer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, MultiAddress]>;
      /**
       * Re-allow unprivileged transfer of an item.
       * 
       * Origin must be Signed and the sender should be the Freezer of the `collection`.
       * 
       * - `collection`: The collection of the item to be thawed.
       * - `item`: The item of the item to be thawed.
       * 
       * Emits `Thawed`.
       * 
       * Weight: `O(1)`
       **/
      thaw: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Re-allow unprivileged transfers for a whole collection.
       * 
       * Origin must be Signed and the sender should be the Admin of the `collection`.
       * 
       * - `collection`: The collection to be thawed.
       * 
       * Emits `CollectionThawed`.
       * 
       * Weight: `O(1)`
       **/
      thawCollection: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Move an item from the sender account to another.
       * 
       * This resets the approved account of the item.
       * 
       * Origin must be Signed and the signing account must be either:
       * - the Admin of the `collection`;
       * - the Owner of the `item`;
       * - the approved delegate for the `item` (in this case, the approval is reset).
       * 
       * Arguments:
       * - `collection`: The collection of the item to be transferred.
       * - `item`: The item of the item to be transferred.
       * - `dest`: The account to receive ownership of the item.
       * 
       * Emits `Transferred`.
       * 
       * Weight: `O(1)`
       **/
      transfer: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, MultiAddress]>;
      /**
       * Change the Owner of a collection.
       * 
       * Origin must be Signed and the sender should be the Owner of the `collection`.
       * 
       * - `collection`: The collection whose owner should be changed.
       * - `owner`: The new Owner of this collection. They must have called
       * `set_accept_ownership` with `collection` in order for this operation to succeed.
       * 
       * Emits `OwnerChanged`.
       * 
       * Weight: `O(1)`
       **/
      transferOwnership: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    utility: {
      /**
       * Send a call through an indexed pseudonym of the sender.
       * 
       * Filter from origin are passed along. The call will be dispatched with an origin which
       * use the same filter as the origin of this call.
       * 
       * NOTE: If you need to ensure that any account-based filtering is not honored (i.e.
       * because you expect `proxy` to have been used prior in the call stack and you do not want
       * the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`
       * in the Multisig pallet instead.
       * 
       * NOTE: Prior to version *12, this was called `as_limited_sub`.
       * 
       * The dispatch origin for this call must be _Signed_.
       **/
      asDerivative: AugmentedSubmittable<(index: u16 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Call]>;
      /**
       * Send a batch of dispatch calls.
       * 
       * May be called from any origin except `None`.
       * 
       * - `calls`: The calls to be dispatched from the same origin. The number of call must not
       * exceed the constant: `batched_calls_limit` (available in constant metadata).
       * 
       * If origin is root then the calls are dispatched without checking origin filter. (This
       * includes bypassing `frame_system::Config::BaseCallFilter`).
       * 
       * # <weight>
       * - Complexity: O(C) where C is the number of calls to be batched.
       * # </weight>
       * 
       * This will return `Ok` in all circumstances. To determine the success of the batch, an
       * event is deposited. If a call failed and the batch was interrupted, then the
       * `BatchInterrupted` event is deposited, along with the number of successful calls made
       * and the error of the failed call. If all were successful, then the `BatchCompleted`
       * event is deposited.
       **/
      batch: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * Send a batch of dispatch calls and atomically execute them.
       * The whole transaction will rollback and fail if any of the calls failed.
       * 
       * May be called from any origin except `None`.
       * 
       * - `calls`: The calls to be dispatched from the same origin. The number of call must not
       * exceed the constant: `batched_calls_limit` (available in constant metadata).
       * 
       * If origin is root then the calls are dispatched without checking origin filter. (This
       * includes bypassing `frame_system::Config::BaseCallFilter`).
       * 
       * # <weight>
       * - Complexity: O(C) where C is the number of calls to be batched.
       * # </weight>
       **/
      batchAll: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * Dispatches a function call with a provided origin.
       * 
       * The dispatch origin for this call must be _Root_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + T::WeightInfo::dispatch_as().
       * # </weight>
       **/
      dispatchAs: AugmentedSubmittable<(asOrigin: RmrkSubstrateRuntimeOriginCaller | { system: any } | { Void: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [RmrkSubstrateRuntimeOriginCaller, Call]>;
      /**
       * Send a batch of dispatch calls.
       * Unlike `batch`, it allows errors and won't interrupt.
       * 
       * May be called from any origin except `None`.
       * 
       * - `calls`: The calls to be dispatched from the same origin. The number of call must not
       * exceed the constant: `batched_calls_limit` (available in constant metadata).
       * 
       * If origin is root then the calls are dispatch without checking origin filter. (This
       * includes bypassing `frame_system::Config::BaseCallFilter`).
       * 
       * # <weight>
       * - Complexity: O(C) where C is the number of calls to be batched.
       * # </weight>
       **/
      forceBatch: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
  } // AugmentedSubmittables
} // declare module
