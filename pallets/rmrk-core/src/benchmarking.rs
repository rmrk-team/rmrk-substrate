//! Benchmarking setup for pallet-template
// To build:
// cargo build --release --features runtime-benchmarks
// 
// To run:
// ./target/release/rmrk-substrate benchmark --pallet pallet-rmrk-core --extrinsic 'create_collection'

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

use sp_runtime::traits::Bounded;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

type DepositBalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
    where_clause { where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>
	}
	create_collection {
		let origin: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&origin, DepositBalanceOf::<T>::max_value());
	}: _(
        RawOrigin::Signed(origin.clone()),
        bvec![0u8; 20],
        Some(5),
        bvec![0u8; 15]
    )
	verify {
		assert_last_event::<T>(Event::CollectionCreated { issuer: origin, collection_id: 0, }.into())
    }

    mint_nft {
        let origin: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&origin, DepositBalanceOf::<T>::max_value());
        let metadata: BoundedVec<u8, T::StringLimit> = bvec![0u8; 20];
        let symbol = bvec![0u8; 15];
        let max = Some(5);
        Pallet::<T>::create_collection(RawOrigin::Signed(origin.clone()).into(), metadata.clone(), max, symbol);
    }: _(
        RawOrigin::Signed(origin.clone()),
        origin.clone(),
        0u32.into(),
        None,
        None,
        metadata.clone()
    )
    verify {
        assert_last_event::<T>(Event::NftMinted { owner: origin, collection_id: 0, nft_id: 0 }.into())
    }

    burn_nft {
        // WIP
        let origin: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&origin, DepositBalanceOf::<T>::max_value());
        let metadata: BoundedVec<u8, T::StringLimit> = bvec![0u8; 20];
        let symbol = bvec![0u8; 15];
        let max = Some(5);
        Pallet::<T>::create_collection(RawOrigin::Signed(origin.clone()).into(), metadata.clone(), max, symbol);

        let metadata: BoundedVec<u8, T::StringLimit> = bvec![1u8; 20]; // TODO max metadata
        for _ in 0..3 {
            Pallet::<T>::mint_nft(
                RawOrigin::Signed(origin.clone()).into(),
                origin.clone(),
                0,
                None,
                None, // TODO add royalty?
                metadata.clone(),
            );
        }
        Pallet::<T>::send(
            RawOrigin::Signed(origin.clone()).into(),
            0,
            1,
            AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0),
        );

    }: _(RawOrigin::Signed(origin.clone()), 0, 0)
    verify {
        // assert_last_event::<T>(Event::CollectionCreated { issuer: origin, collection_id: 0, }.into())
    }
    
	impl_benchmark_test_suite!(Template, crate::mock::ExtBuilder::default().build(whitelisted_caller()), crate::mock::Test);
}
