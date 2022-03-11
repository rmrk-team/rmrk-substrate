#![allow(clippy::too_many_arguments)]
use super::*;

/// Trait to handle NFT Locking mechanism to ensure interactions with the NFT can be implemented
/// downstream to extend logic of Uniques current functionality
#[allow(clippy::upper_case_acronyms)]
pub trait Locker<ClassId, InstanceId> {
	/// Check if the NFT should be locked and prevent interactions with the NFT from executing
	fn check_should_lock(class: ClassId, instance: InstanceId) -> bool;
}

impl<ClassId, InstanceId> Locker<ClassId, InstanceId> for () {
	fn check_should_lock(_class: ClassId, _instance: InstanceId) -> bool {
		false
	}
}

impl<T: Config<I>, I: 'static> Locker<T::ClassId, T::InstanceId> for Pallet<T, I> {
	fn check_should_lock(_class: T::ClassId, _instance: T::InstanceId) -> bool {
		false
	}
}
