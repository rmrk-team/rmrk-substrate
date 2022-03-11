#![allow(clippy::too_many_arguments)]
use super::*;

impl<T: Config<I>, I: 'static> Locker<T::ClassId, T::InstanceId> for Pallet<T, I> {
	fn check_should_lock(_class: T::ClassId, _instance: T::InstanceId) -> bool {
		false
	}
}
