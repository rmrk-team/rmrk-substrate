#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-equip pallet

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use pallet_rmrk_core::Pallet as RmrkCore;
use rmrk_traits::{ComposableResource, SlotPart, SlotResource};
use sp_runtime::{traits::Bounded, Permill};
use sp_std::vec;

use crate::Pallet as RmrkEquip;

pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

const SEED: u32 = 0;

/// Turns a string into a BoundedVec
fn stb<T: Config>(s: &str) -> BoundedVec<u8, T::StringLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn funded_account<T: Config>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

/// Creates a collection
fn create_test_collection<T: Config>(
	caller: T::AccountId,
	collection_index: u32,
) -> T::CollectionId {
	let collection_id = <T as pallet::Config>::Helper::collection(collection_index);
	let metadata = bvec![0u8; 20];
	let max = None;
	let symbol = bvec![0u8; 15];
	<T as pallet_uniques::Config>::Currency::make_free_balance_be(
		&caller,
		BalanceOf::<T>::max_value(),
	);
	let _ = RmrkCore::<T>::create_collection(
		(RawOrigin::Signed(caller.clone())).into(),
		collection_id.clone(),
		metadata,
		max,
		symbol,
	);

	collection_id
}

fn mint_test_nft<T: Config>(
	owner: T::AccountId,
	mint_for: Option<T::AccountId>,
	collection_id: T::CollectionId,
	nft_index: u32,
) -> T::ItemId {
	let nft_id = <T as pallet::Config>::Helper::item(nft_index);
	let royalty_recipient = owner.clone();
	let royalty = Permill::from_percent(1);
	let nft_metadata = bvec![0u8; 20];
	let resource = None;
	let _ = RmrkCore::<T>::mint_nft(
		RawOrigin::Signed(owner.clone()).into(),
		mint_for,
		nft_id,
		collection_id,
		Some(royalty_recipient),
		Some(royalty),
		nft_metadata,
		true,
		resource,
	);
	nft_id
}

// Send nft to Account or to another nft
fn send_test_nft<T: Config>(
	owner: T::AccountId,
	collection_id: T::CollectionId,
	nft_id: T::ItemId,
	new_owner_enum: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
) {
	let _ =
		RmrkCore::<T>::send(RawOrigin::Signed(owner).into(), collection_id, nft_id, new_owner_enum);
}

/// Creates a base
fn base_create<T: Config>(
	creator: T::AccountId,
	parts: BoundedVec<
		PartType<StringLimitOf<T>, BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>>,
		T::PartsLimit,
	>,
) {
	let _ = RmrkEquip::<T>::create_base(
		RawOrigin::Signed(creator).into(),
		bvec![0u8; 20],
		bvec![0u8; 20],
		parts,
	);
}

fn hand_slot_part<T: Config>(
	collection_id: T::CollectionId,
	id: u32,
) -> SlotPart<
	BoundedVec<u8, T::StringLimit>,
	BoundedVec<T::CollectionId, T::MaxCollectionsEquippablePerPart>,
> {
	SlotPart {
		id,
		z: 0,
		src: Some(stb::<T>("hand")),
		equippable: EquippableList::Custom(bvec![collection_id]),
	}
}

fn add_composable_resource<T: Config>(
	caller: T::AccountId,
	collection_id: T::CollectionId,
	item: T::ItemId,
	base_id: BaseId,
	parts: Vec<u32>,
) {
	let composable_resource = ComposableResource {
		parts: parts.try_into().unwrap(),
		base: base_id,
		metadata: None,
		slot: None,
	};

	let _ = RmrkCore::<T>::add_composable_resource(
		RawOrigin::Signed(caller.clone()).into(),
		collection_id,
		item,
		composable_resource,
		0,
	);
}

fn add_slot_resource<T: Config>(
	caller: T::AccountId,
	collection_id: T::CollectionId,
	item: T::ItemId,
	base_id: BaseId,
	slot: u32,
) {
	let slot_resource = SlotResource { base: base_id, metadata: None, slot };

	let _ = RmrkCore::<T>::add_slot_resource(
		RawOrigin::Signed(caller.clone()).into(),
		collection_id,
		item,
		slot_resource,
		0,
	);
}

benchmarks! {
	change_base_issuer {
		let caller: T::AccountId = whitelisted_caller();
		let new_issuer = funded_account::<T>("new_issuer", 0);
		let new_issuer_lookup = T::Lookup::unlookup(new_issuer.clone());
		base_create::<T>(caller.clone(), bvec![]);
	}: _(RawOrigin::Signed(caller.clone()), 0u32, new_issuer_lookup)
	verify {
		assert_last_event::<T>(Event::BaseIssuerChanged {
			old_issuer: caller,
			new_issuer: new_issuer,
			base_id: 0u32,
		}.into());
	}

	equip {
		let caller: T::AccountId = whitelisted_caller();
		let collection_id = create_test_collection::<T>(caller.clone(), 1);

		let slot_part_hand = hand_slot_part::<T>(collection_id, 201);
		base_create::<T>(caller.clone(), bvec![PartType::SlotPart(slot_part_hand)]);

		let character = mint_test_nft::<T>(caller.clone(), None, collection_id, 0);
		let sword = mint_test_nft::<T>(caller.clone(), None, collection_id, 1);
		let new_owner = AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, character);
		send_test_nft::<T>(caller.clone(), collection_id, sword, new_owner);

		add_composable_resource::<T>(caller.clone(), collection_id, character, 0, vec![201]);
		add_slot_resource::<T>(caller.clone(), collection_id, sword, 0, 201);
		let item = (collection_id, sword);
		let equipper = (collection_id, character);
	}: _(RawOrigin::Signed(caller.clone()), item, equipper, 0u32, 0, 201)
	verify {
		assert_last_event::<T>(Event::SlotEquipped {
			item_collection: collection_id,
			item_nft: item.1,
			base_id: 0,
			slot_id: 201,
		}.into())
	}

	unequip {
		let caller: T::AccountId = whitelisted_caller();
		let collection_id = create_test_collection::<T>(caller.clone(), 1);

		let slot_part_hand = hand_slot_part::<T>(collection_id, 201);
		base_create::<T>(caller.clone(), bvec![PartType::SlotPart(slot_part_hand)]);

		let character = mint_test_nft::<T>(caller.clone(), None, collection_id, 0);
		let sword = mint_test_nft::<T>(caller.clone(), None, collection_id, 1);
		let new_owner = AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, character);
		send_test_nft::<T>(caller.clone(), collection_id, sword, new_owner);

		add_composable_resource::<T>(caller.clone(), collection_id, character, 0, vec![201]);
		add_slot_resource::<T>(caller.clone(), collection_id, sword, 0, 201);
		let item = (collection_id, sword);
		// the equipper is going to be the unequipper.
		let equipper = (collection_id, character);
		let _ = RmrkEquip::<T>::equip(RawOrigin::Signed(caller.clone()).into(), item, equipper, 0u32, 0, 201);
	}: _(RawOrigin::Signed(caller.clone()), item, equipper, 0, 201)
	verify {
		assert_last_event::<T>(Event::SlotUnequipped {
			item_collection: collection_id,
			item_nft: item.1,
			base_id: 0,
			slot_id: 201,
		}.into())
	}

	equippable {
		let caller: T::AccountId = whitelisted_caller();
		let collection_0 = <T as pallet::Config>::Helper::collection(0);

		let slot_part_hand = hand_slot_part::<T>(collection_0, 201);
		base_create::<T>(caller.clone(), bvec![PartType::SlotPart(slot_part_hand)]);

		let collection_1 = <T as pallet::Config>::Helper::collection(1);
	}: _(RawOrigin::Signed(caller.clone()), 0, 201, EquippableList::Custom(bvec![collection_1]))
	verify {
		assert_last_event::<T>(Event::EquippablesUpdated { base_id: 0, slot_id: 201 }.into())
	}

	equippable_add {
		let caller: T::AccountId = whitelisted_caller();
		let collection_0 = <T as pallet::Config>::Helper::collection(0);

		let slot_part_hand = hand_slot_part::<T>(collection_0, 201);
		base_create::<T>(caller.clone(), bvec![PartType::SlotPart(slot_part_hand)]);

		let collection_1 = <T as pallet::Config>::Helper::collection(1);
	}: _(RawOrigin::Signed(caller.clone()), 0, 201, collection_1)
	verify {
		assert_last_event::<T>(Event::EquippablesUpdated { base_id: 0, slot_id: 201 }.into())
	}

	equippable_remove {
		let caller: T::AccountId = whitelisted_caller();
		let collection_0 = <T as pallet::Config>::Helper::collection(0);

		let slot_part_hand = hand_slot_part::<T>(collection_0, 201);
		base_create::<T>(caller.clone(), bvec![PartType::SlotPart(slot_part_hand)]);
	}: _(RawOrigin::Signed(caller.clone()), 0, 201, collection_0)
	verify {
		assert_last_event::<T>(Event::EquippablesUpdated { base_id: 0, slot_id: 201 }.into())
	}

	theme_add {
		let caller: T::AccountId = whitelisted_caller();
		let default_theme = Theme {
			name: stb::<T>("default"),
			properties: bvec![
				ThemeProperty { key: stb::<T>("primary_color"), value: stb::<T>("red") },
				ThemeProperty { key: stb::<T>("secondary_color"), value: stb::<T>("blue") },
			],
			inherit: false,
		};
		base_create::<T>(caller.clone(), bvec![]);
	}: _(RawOrigin::Signed(caller.clone()), 0, default_theme)
	verify {

	}

	create_base {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.clone()), bvec![42u8; 20], bvec![25u8; 20], bvec![])
	verify {
		assert_last_event::<T>(Event::BaseCreated { issuer: caller, base_id: 0 }.into())
	}

	impl_benchmark_test_suite!(RmrkEquip, crate::benchmarking::tests::new_test_ext(), crate::mock::Test);
}

#[cfg(test)]
mod tests {
	use crate::mock;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		mock::ExtBuilder::default().build()
	}
}
