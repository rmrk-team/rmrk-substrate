#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-equip pallet

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use pallet_rmrk_core::Pallet as RmrkCore;
use rmrk_traits::{ComposableResource, SlotPart, SlotResource};
use sp_runtime::{traits::Bounded, Permill};

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
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
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

fn mint_test_nft_directly_to<T: Config>(
	owner: T::AccountId,
	mint_to: (T::CollectionId, T::ItemId),
	collection_id: T::CollectionId,
	nft_index: u32,
) -> T::ItemId {
	let nft_id = <T as pallet::Config>::Helper::item(nft_index);
	let royalty_recipient = owner.clone();
	let royalty = Permill::from_percent(1);
	let nft_metadata = bvec![0u8; 20];
	let resource = None;
	let _ = RmrkCore::<T>::mint_nft_directly_to_nft(
		RawOrigin::Signed(owner.clone()).into(),
		mint_to,
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

/// Creates a base
fn create_base<T: Config>(
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
		create_base::<T>(caller.clone(), bvec![]);
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
		create_base::<T>(caller.clone(), bvec![PartType::SlotPart(slot_part_hand)]);

		let character = mint_test_nft::<T>(caller.clone(), None, collection_id, 0);
		let sword = mint_test_nft_directly_to::<T>(caller.clone(), (collection_id, character), collection_id, 1);

		add_composable_resource::<T>(caller.clone(), collection_id, character, 0, vec![201]);
		add_slot_resource::<T>(caller.clone(), collection_id, sword, 0, 201);
		let item = (collection_id, sword);
		let equipper = (collection_id, character);
	}: _(RawOrigin::Signed(caller.clone()), item, equipper, 0u32, 0, 201)
	verify {

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
