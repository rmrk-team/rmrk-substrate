use super::*;
use frame_support::traits::IsType;

impl<T: Config> Pallet<T> {
	pub fn is_x_descendent_of_y(
		child_collection_id: T::CollectionId,
		child_nft_id: T::NftId,
		parent_collection_id: T::CollectionId,
		parent_nft_id: T::NftId,
	) -> bool {
		let mut found_child = false;
		if let Some(children) = Children::<T>::get(parent_collection_id, parent_nft_id) {
			for child in children {
				if child == (child_collection_id, child_nft_id) {
					return true;
				} else {
					if Pallet::<T>::is_x_descendent_of_y(
						child_collection_id,
						child_nft_id,
						child.0,
						child.1,
					) {
						found_child = true;
					}
				}
			}
		}
		found_child
	}

	pub fn recursive_update_rootowner(
		collection_id: T::CollectionId,
		nft_id: T::NftId,
		new_rootowner: T::AccountId,
		max_recursions: u32,
	) -> DispatchResult {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::try_mutate_exists(collection_id, nft_id, |nft| -> DispatchResult {
			if let Some(n) = nft.into_mut() {
				n.rootowner = new_rootowner.clone();
			}
			Ok(())
		})?;
		if let Some(children) = Children::<T>::get(collection_id, nft_id) {
			for child in children {
				Pallet::<T>::recursive_update_rootowner(
					child.0,
					child.1,
					new_rootowner.clone(),
					max_recursions - 1,
				)?;
			}
		}
		Ok(())
	}

	pub fn recursive_burn(
		collection_id: T::CollectionId,
		nft_id: T::NftId,
		max_recursions: u32,
	) -> DispatchResult {
		ensure!(max_recursions > 0, Error::<T>::TooManyRecursions);
		NFTs::<T>::remove(collection_id, nft_id);
		if let Some(kids) = Children::<T>::take(collection_id, nft_id) {
			for child in kids {
				Pallet::<T>::recursive_burn(child.0, child.1, max_recursions - 1)?;
			}
		}
		Ok(())
	}
}
