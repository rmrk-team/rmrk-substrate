use super::*;

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
}
