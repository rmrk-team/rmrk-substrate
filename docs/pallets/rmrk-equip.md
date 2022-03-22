# Equip NFT Pallet Design

![](https://static.swimlanes.io/9b72cd84a9ca752cbb7f2c6348d6a50b.png)

Equips an owned NFT into a slot on its parent, or unequips it.
* You can only EQUIP an existing NFT (one that has not been BURNed yet). 
* You can only EQUIP an NFT into its immediate parent. 
* You cannot equip across ancestors, or even across other NFTs. 
* You can only unequip an equipped NFT.
* You can only equip a non-pending child NFT.
* You can equip/unequip a non-transferable NFT. As an example, putting a helmet on or taking it off does not change the ownership of the helmet.

Equip logic has many implications - per-slot limits, nested rendering across multiple depths (ie configurable cap on configurable kanaria bird), unequip on sale, burn, etc. Equip "addon lego" not useful by itself, it needs the core.
Should extend RMRK Core pallet.

Full RMRK2 spec for [Equip](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/interactions/equip.md) and [Base](https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk2.0.0/entities/base.md)

## Calls



### **equip**
Equip a child NFT into a parent's slot, or unequip
```rust
    item: (CollectionId, NftId),
    equipper: (CollectionId, NftId),
    base: BaseId,
    slot: SlotId
```

### **equippable**
Changes the list of equippable collections on a base's part

```rust
    base_id: BaseId,
    slot_id: SlotId,
    equippables: EquippableList
```
### **theme_add**
Add a new theme to a base

```rust
    base_id: BaseId,
    theme: Theme<BoundedVec<u8, T::StringLimit>>
```

### **create_base**
Create a base. catalogue of parts. It is not an NFT

```rust
    base_type: BoundedVec<u8, T::StringLimit>,
    symbol: BoundedVec<u8, T::StringLimit>,
    parts: Vec<PartType<StringLimitOf<T>>>
```
    

## Storages
Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-equip/src/lib.rs#L51-L83)

* Bases
* Parts
* NextBaseId
* NextPartId
* Equippings
* Themes

## Events
Current implementation [here](https://github.com/rmrk-team/rmrk-substrate/blob/main/pallets/rmrk-equip/src/lib.rs#L105-L126)
* BaseCreated
* SlotEquipped
* SlotUnequipped
* EquippablesUpdated

## Traits / Types
Set of re-usable traits describing the total interface located [here](https://github.com/rmrk-team/rmrk-substrate/tree/main/traits/src)

### Primitives
```rust
type BaseId = u32;
type SlotId = u32;
type PartId = u32;
type ZIndex = u32;
```

### BaseInfo
```rust
pub struct BaseInfo<AccountId, BoundedString> {
	/// Original creator of the Base
	pub issuer: AccountId,
	/// Specifies how an NFT should be rendered, ie "svg"
	pub base_type: BoundedString,
	/// User provided symbol during Base creation
	pub symbol: BoundedString,
	/// Parts, full list of both Fixed and Slot parts
	pub parts: Vec<PartType<BoundedString>>,
}
```

### Part
```rust 
pub enum PartType<BoundedString> {
	FixedPart(FixedPart<BoundedString>),
	SlotPart(SlotPart<BoundedString>),
}

pub struct FixedPart<BoundedString> {
	pub id: PartId,
	pub z: ZIndex,
	pub src: BoundedString,
}

pub struct SlotPart<BoundedString> {
	pub id: PartId,
	pub equippable: EquippableList,
	pub src: BoundedString,
	pub z: ZIndex,
}

pub enum EquippableList {
	All,
	Empty,
	Custom(Vec<CollectionId>),
}
```

### Theme

```rust
pub struct Theme<BoundedString> {
	/// Name of the theme
	pub name: BoundedString,
	/// Theme properties
	pub properties: Vec<ThemeProperty<BoundedString>>,
}

pub struct ThemeProperty<BoundedString> {
	/// Key of the property
	pub key: BoundedString,
	/// Value of the property
	pub value: BoundedString,
	/// Inheritability
	pub inherit: Option<bool>,
}
```