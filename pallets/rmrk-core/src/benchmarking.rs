#![cfg(feature = "runtime-benchmarks")]

// Benchmarks for rmrk-core pallet

use super::*;
#[allow(unused)]
use crate::Pallet as RmrkCore;

use codec::alloc::string::ToString;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::{Currency, Get};
use frame_system::RawOrigin;
use rmrk_traits::{AccountIdOrCollectionNftTuple, BasicResource};
use sp_runtime::traits::Bounded;
use sp_std::vec;

pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

const SEED: u32 = 0;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

/// Turns a string into a BoundedVec
fn stb<T: Config>(s: &str) -> BoundedVec<u8, T::ValueLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedVec
fn stbk<T: Config>(s: &str) -> BoundedVec<u8, T::KeyLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedVec
fn stbd<T: Config>(s: &str) -> StringLimitOf<T> {
	s.as_bytes().to_vec().try_into().unwrap()
}

fn funded_account<T: Config>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

/// Creates a collection
fn create_test_collection<T: Config>(
	caller: T::AccountId,
	collection_index: u32,
) -> T::CollectionId {
	let collection_id = T::Helper::collection(collection_index);
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

/// Mint a token
fn mint_test_nft<T: Config>(
	owner: T::AccountId,
	mint_for: Option<T::AccountId>,
	collection_id: T::CollectionId,
	nft_index: u32,
) -> T::ItemId {
	let nft_id = T::Helper::item(nft_index);
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

// premint nfts & make deep nested chain of nfts ( send child to parent )
fn mint_and_send_to_parent<T: Config>(owner: T::AccountId, collection_id: T::CollectionId, n: u32) {
	for i in 1..n {
		let id = mint_test_nft::<T>(owner.clone(), None, collection_id, i);
		let parent_nft_id = T::Helper::item(i.saturating_sub(1));
		let new_owner =
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, parent_nft_id);
		send_test_nft::<T>(owner.clone(), collection_id, id, new_owner);
	}
}

// premint nfts & make deep nested chain of nfts ( send child to the specified parent )
fn mint_and_send_to<T: Config>(
	owner: T::AccountId,
	collection_id: T::CollectionId,
	n: u32,
	parent: u32,
) {
	for i in (parent + 1)..(n + parent) {
		let id = mint_test_nft::<T>(owner.clone(), None, collection_id, i);
		let parent_nft_id = T::Helper::item(i.saturating_sub(1));
		let new_owner =
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, parent_nft_id);
		send_test_nft::<T>(owner.clone(), collection_id, id, new_owner);
	}
}

// Send nft to Account or to another nft
fn send_test_nft<T: Config>(
	owner: T::AccountId,
	collection_id: T::CollectionId,
	nft_id: T::ItemId,
	new_owner_enum: AccountIdOrCollectionNftTuple<T::AccountId, T::CollectionId, T::ItemId>,
) {
	let _ = RmrkCore::<T>::send(
		RawOrigin::Signed(owner.clone()).into(),
		collection_id,
		nft_id,
		new_owner_enum,
	);
}

// create collection, mint 1 nft and initialize resource_id
fn prepare_resource<T: Config>(
) -> (T::AccountId, T::AccountId, T::CollectionId, T::ItemId, ResourceId) {
	let alice: T::AccountId = whitelisted_caller();
	let bob = funded_account::<T>("bob", 0);
	let collection_index = 1;
	let collection_id = create_test_collection::<T>(alice.clone(), collection_index);
	let nft_id = mint_test_nft::<T>(alice.clone(), Some(bob.clone()), collection_id, 0);
	let resource_id = 0;
	(alice, bob, collection_id, nft_id, resource_id)
}

fn set_properties<T: Config>(
	caller: T::AccountId,
	collection_id: T::CollectionId,
	maybe_nft_id: Option<T::ItemId>,
	n: u32,
) {
	(0..n).for_each(|i| {
		let _ = RmrkCore::<T>::set_property(
			RawOrigin::Signed(caller.clone()).into(),
			collection_id,
			maybe_nft_id,
			stbk::<T>(i.to_string().as_str()),
			stb::<T>(i.to_string().as_str()),
		);
	});
}

benchmarks! {
	create_collection {
		let caller: T::AccountId = whitelisted_caller();
		let collection_index = 42;
		let collection_id: <T as pallet_uniques::Config>::CollectionId = T::Helper::collection(collection_index);
		let metadata = bvec![0u8; 20];
		let max = None;
		let symbol = bvec![0u8; 15];
		<T as pallet_uniques::Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

	}: _(RawOrigin::Signed(caller.clone()), collection_id, metadata, max, symbol)
	verify {
		assert_last_event::<T>(Event::CollectionCreated { issuer: caller, collection_id }.into());
	}

	mint_nft {
		let owner: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id: <T as pallet_uniques::Config>::ItemId = T::Helper::item(42);
		let royalty_recipient: T::AccountId = owner.clone();
		let royalty = Permill::from_percent(1);
		let nft_metadata = bvec![0u8; 20];
		let resource = None;
		let owner_enum = AccountIdOrCollectionNftTuple::AccountId(owner.clone());

		<T as pallet_uniques::Config>::Currency::make_free_balance_be(&owner, BalanceOf::<T>::max_value());

	}: _(RawOrigin::Signed(owner.clone()), None, nft_id, collection_id, Some(royalty_recipient), Some(royalty), nft_metadata, true, resource)
	verify {
		assert_last_event::<T>(Event::NftMinted{ owner: owner_enum, collection_id, nft_id }.into());
	}

	mint_nft_directly_to_nft {
		let n in 1 .. (T::NestingBudget::get() - 1);

		let owner: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		mint_test_nft::<T>(owner.clone(), None, collection_id, 0);

		mint_and_send_to_parent::<T>(owner.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let nft_child_id: <T as pallet_uniques::Config>::ItemId = T::Helper::item(n);
		let owner_tuple = AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, nft_id);
		let nft_owner_tuple = (collection_id, nft_id);
		let royalty_recipient = owner.clone();
		let royalty = Permill::from_percent(1);
		let nft_metadata = bvec![0u8; 20];
		let resource = None;

	}: _(RawOrigin::Signed(owner.clone()), nft_owner_tuple, nft_child_id, collection_id, Some(royalty_recipient), Some(royalty), nft_metadata, true, resource)
	verify {
		assert!(RmrkCore::<T>::nfts(collection_id, nft_id).is_some());
		assert!(RmrkCore::<T>::nfts(collection_id, nft_child_id).is_some());
		assert_last_event::<T>(Event::NftMinted{ owner: owner_tuple, collection_id, nft_id: nft_child_id }.into());
	}

	destroy_collection {
		let owner: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);

	}:  _(RawOrigin::Signed(owner.clone()), collection_id)
	verify {
		// assert!(RmrkCore::<T>::collection_index() == 1);
		assert_last_event::<T>(Event::CollectionDestroyed { issuer: owner, collection_id }.into());
	}

	send_to_account {
		let n in 1 .. T::NestingBudget::get();
		let owner: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		mint_test_nft::<T>(owner.clone(), None, collection_id, 0);

		mint_and_send_to_parent::<T>(owner.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let bob = funded_account::<T>("bob", 0);
		let new_owner = AccountIdOrCollectionNftTuple::AccountId(bob);

	}: send(RawOrigin::Signed(owner.clone()), collection_id, nft_id, new_owner.clone())
	verify {
		assert_last_event::<T>(Event::NFTSent {
			sender: owner,
			recipient: new_owner,
			collection_id,
			nft_id,
			approval_required: false,
		}.into());
	}

	send_to_nft {
		let n in 1 .. T::NestingBudget::get();
		let alice: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(alice.clone(), collection_index);
		let nft_id1 = mint_test_nft::<T>(alice.clone(), None, collection_id, 0);
		mint_test_nft::<T>(alice.clone(), None, collection_id, 1);
		mint_and_send_to::<T>(alice.clone(), collection_id, n, 1);
		let child_nft = T::Helper::item(n);
		// Alice sends NFT (0,1) to Bob's account
		let bob = funded_account::<T>("bob", 0);
		let new_owner = AccountIdOrCollectionNftTuple::AccountId(bob);
		send_test_nft::<T>(alice.clone(), collection_id, nft_id1, new_owner.clone());

		// Alice sends child NFT (0,n) to parent NFT (0,1)
		let parent_nft = AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, nft_id1);

	}: send(RawOrigin::Signed(alice.clone()), collection_id, child_nft, parent_nft.clone())
	verify {
		assert_last_event::<T>(Event::NFTSent {
			sender: alice,
			recipient: parent_nft,
			collection_id,
			nft_id: child_nft,
			approval_required: true,
		}.into());
	}

	burn_nft {
		let n in 1 .. T::NestingBudget::get();
		let k in 0 .. T::PropertiesLimit::get();

		let owner: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(owner.clone(), collection_index);
		let nft_id = mint_test_nft::<T>(owner.clone(), None, collection_id, 0);

		set_properties::<T>(owner.clone(), collection_id, Some(nft_id), k);
		mint_and_send_to_parent::<T>(owner.clone(), collection_id, n);
		RmrkCore::<T>::set_lock((collection_id, nft_id), true);
	}: _(RawOrigin::Signed(owner.clone()), collection_id, nft_id)
	verify {
		assert_last_event::<T>(Event::NFTBurned { owner, collection_id, nft_id }.into());
	}

	accept_nft {
		let n in 1 .. T::NestingBudget::get();

		let alice: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(alice.clone(), collection_index);
		let nft_id1 = mint_test_nft::<T>(alice.clone(), None, collection_id, 0);
		mint_test_nft::<T>(alice.clone(), None, collection_id, 1);
		mint_and_send_to::<T>(alice.clone(), collection_id, n, 1);
		let child_nft = T::Helper::item(n);

		// Alice sends NFT (0,1) to Bob's account
		let bob = funded_account::<T>("bob", 0);
		let new_owner = AccountIdOrCollectionNftTuple::AccountId(bob.clone());
		send_test_nft::<T>(alice.clone(), collection_id, nft_id1, new_owner.clone());

		// Alice sends child NFT (0,2) to parent NFT (0,1)
		let parent_nft = AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, nft_id1);
		send_test_nft::<T>(alice.clone(), collection_id, child_nft, parent_nft.clone());

	}: _(RawOrigin::Signed(bob.clone()), collection_id, child_nft, parent_nft.clone())
	verify {
		assert_last_event::<T>(Event::NFTAccepted {
			sender: bob,
			recipient: parent_nft,
			collection_id,
			nft_id: child_nft,
		}.into());
	}

	reject_nft {
		let n in 1 .. T::NestingBudget::get();
		let alice: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(alice.clone(), collection_index);
		let nft_id1 = mint_test_nft::<T>(alice.clone(), None, collection_id, 0);
		let child_nft = mint_test_nft::<T>(alice.clone(), None, collection_id, 42);
		// Alice sends NFT (0,1) to Bob's account
		let bob = funded_account::<T>("bob", 0);
		let new_owner = AccountIdOrCollectionNftTuple::AccountId(bob.clone());
		send_test_nft::<T>(alice.clone(), collection_id, nft_id1, new_owner.clone());


		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);

		// Alice sends child NFT (0,2) to parent NFT (0,1)
		let parent_nft = AccountIdOrCollectionNftTuple::CollectionAndNftTuple(collection_id, nft_id1);
		send_test_nft::<T>(alice.clone(), collection_id, child_nft, parent_nft.clone());

	}: _(RawOrigin::Signed(bob.clone()), collection_id, child_nft)
	verify {
		assert_last_event::<T>(Event::NFTRejected {
			sender: bob,
			collection_id,
			nft_id: child_nft,
		}.into());
	}

	change_collection_issuer {
		let alice: T::AccountId = whitelisted_caller();
		let collection_index = 1;
		let collection_id = create_test_collection::<T>(alice.clone(), collection_index);
		let _ = mint_test_nft::<T>(alice.clone(), None, collection_id, 1);

		// Must set ownership acceptance with BOB before transfer due to uniques dependency
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		T::Currency::make_free_balance_be(&target, BalanceOf::<T>::max_value());
		let origin = RawOrigin::Signed(target.clone()).into();
		pallet_uniques::Pallet::<T>::set_accept_ownership(origin, Some(collection_id))?;

	}: _(RawOrigin::Signed(alice.clone()), collection_id, target_lookup)
	verify {
		assert_last_event::<T>(Event::IssuerChanged {
			old_issuer: alice,
			new_issuer: target,
			collection_id,
		}.into());
	}

	set_property {
		let alice: T::AccountId = whitelisted_caller();

		let key = stbk::<T>("test-key");
		let value = stb::<T>("test-value");

		let collection_index = 1;
		let collection_id = create_test_collection::<T>(alice.clone(), collection_index);
		let maybe_nft_id = mint_test_nft::<T>(alice.clone(), None, collection_id, 1);

	}: _(RawOrigin::Signed(alice.clone()), collection_id, Some(maybe_nft_id), key.clone(), value.clone())
	verify {
		assert_last_event::<T>(Event::PropertySet {
			collection_id, maybe_nft_id: Some(maybe_nft_id), key, value }.into());
	}

	lock_collection {
		let alice: T::AccountId = whitelisted_caller();
	let collection_index = 1;
	let collection_id = create_test_collection::<T>(alice.clone(), collection_index);

	}: _(RawOrigin::Signed(alice.clone()), collection_id)
	verify {
		assert_last_event::<T>(Event::CollectionLocked { issuer: alice, collection_id }.into());
	}

	replace_resource {
		let (alice, _, collection_id, nft_id, resource_id) = prepare_resource::<T>();
		let basic_resource = BasicResource{ metadata: stbd::<T> ("basic test metadata") };
		let _ = RmrkCore::<T>::add_basic_resource(RawOrigin::Signed(alice.clone()).into(), collection_id, nft_id, basic_resource, resource_id);
		let resource = ResourceTypes::Basic(BasicResource { metadata: stbd::<T> ("replaced basic test metadata") }); // new_resource
	}:  _(RawOrigin::Signed(alice.clone()), collection_id, nft_id, resource, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceReplaced { nft_id, resource_id, collection_id }.into());
	}

	add_basic_resource{
		let (alice, _, collection_id, _, resource_id) = prepare_resource::<T>();

		let n in 1 .. T::NestingBudget::get();
		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let basic_resource = BasicResource{ metadata: stbd::<T> ("basic test metadata") };
	}: _(RawOrigin::Signed(alice.clone()), collection_id, nft_id, basic_resource, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceAdded { nft_id, resource_id, collection_id }.into());
	}

	add_composable_resource{
		let (alice, _, collection_id, _, resource_id) = prepare_resource::<T>();

		let n in 1 .. T::NestingBudget::get();
		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let composable_resource = ComposableResource {
			parts: vec![0, 1].try_into().unwrap(), // BoundedVec of Parts
			base: 0,                               // BaseID
			metadata: Some(stbd::<T> ("basic test metadata")),
			slot: None,
		};

	}: _(RawOrigin::Signed(alice.clone()), collection_id, nft_id, composable_resource, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceAdded { nft_id, resource_id, collection_id }.into());
	}

	add_slot_resource{
		let (alice, _, collection_id, _, resource_id) = prepare_resource::<T>();

		let n in 1 .. T::NestingBudget::get();
		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let slot_resource = SlotResource {
			base: 0, // BaseID
			metadata: Some(stbd::<T> ("basic test metadata")),
			slot: 0, // SlotID
		};

	}: _(RawOrigin::Signed(alice.clone()), collection_id, nft_id, slot_resource, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceAdded { nft_id, resource_id, collection_id }.into());
	}

	accept_resource{
		let (alice, bob, collection_id, _, resource_id) = prepare_resource::<T>();

		let n in 1 .. T::NestingBudget::get();
		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let basic_resource = BasicResource{ metadata: stbd::<T> ("basic test metadata") };
		// Alice is collection issuer and she adds resource to bob's nft
		let _ = RmrkCore::<T>::add_basic_resource(RawOrigin::Signed(alice.clone()).into(), collection_id, nft_id, basic_resource, resource_id);
		// Bob accepts new resource
	}: _(RawOrigin::Signed(bob), collection_id, nft_id, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceAccepted { nft_id, resource_id, collection_id }.into());
	}

	remove_resource{
		let (alice, bob, collection_id, _, resource_id) = prepare_resource::<T>();

		let n in 1 .. T::NestingBudget::get();
		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let basic_resource = BasicResource{ metadata: stbd::<T> ("basic test metadata") };
		// Alice is collection issuer and she adds resource to bob's nft
		let _ = RmrkCore::<T>::add_basic_resource(RawOrigin::Signed(alice.clone()).into(), collection_id, nft_id, basic_resource, resource_id);
		// Bob accepts new resource
		let _ = RmrkCore::<T>::accept_resource(RawOrigin::Signed(bob).into(), collection_id, nft_id, resource_id);
		// Only collection issuer can request resource removal
	}: _(RawOrigin::Signed(alice), collection_id, nft_id, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceRemoval { nft_id, resource_id, collection_id }.into());
	}

	accept_resource_removal{
		let (alice, bob, collection_id, _, resource_id) = prepare_resource::<T>();

		let n in 1 .. T::NestingBudget::get();
		mint_and_send_to_parent::<T>(alice.clone(), collection_id, n);
		let nft_id = T::Helper::item(n-1);

		let basic_resource = BasicResource{ metadata: stbd::<T> ("basic test metadata") };
		// Alice is collection issuer and she adds resource to bob's nft
		let _ = RmrkCore::<T>::add_basic_resource(RawOrigin::Signed(alice.clone()).into(), collection_id, nft_id, basic_resource, resource_id);
		// Bob accepts new resource
		let _ = RmrkCore::<T>::accept_resource(RawOrigin::Signed(bob.clone()).into(), collection_id, nft_id, resource_id);
		// Only collection issuer can request resource removal
		let _ = RmrkCore::<T>::remove_resource(RawOrigin::Signed(alice).into(), collection_id, nft_id, resource_id);
		// Bob accepts resource removal
	}: _(RawOrigin::Signed(bob), collection_id, nft_id, resource_id)
	verify {
		assert_last_event::<T>(Event::ResourceRemovalAccepted { nft_id, resource_id, collection_id }.into());
	}

	set_priority{
		let n in 1 .. T::MaxPriorities::get();
		let k in 1 .. T::NestingBudget::get();
		let (alice, bob, collection_id, _, resource_id) = prepare_resource::<T>();

		mint_and_send_to_parent::<T>(alice.clone(), collection_id, k);
		let nft_id = T::Helper::item(k-1);

		let basic_resource = BasicResource{ metadata: stbd::<T> ("basic test metadata") };
		let mut priorities: BoundedVec<ResourceId, T::MaxPriorities> = vec![].try_into().unwrap();
		for resource_id in 1 .. n{
			let _ = priorities.try_push(resource_id);
		}

	}: _(RawOrigin::Signed(bob), collection_id, nft_id, priorities)
	verify {
		assert_last_event::<T>(Event::PrioritySet { collection_id, nft_id }.into());
	}

	// This line generates test cases for benchmarking, and could be run by:
	//   `cargo test --package pallet-rmrk-core --features runtime-benchmarks`
	impl_benchmark_test_suite!(RmrkCore, crate::benchmarking::tests::new_test_ext(), crate::mock::Test);
}
#[cfg(test)]
mod tests {
	use crate::mock;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		mock::ExtBuilder::build()
	}
}
