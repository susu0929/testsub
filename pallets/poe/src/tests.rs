use crate::{mock::*, Error, Proofs};
use frame_support::{assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		// let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).unwrap();
		// assert_eq!(
		// 	Proofs::<Test>::get(&bounded_claim),
		// 	Some((1, frame_system::Pallet::<Test>::block_number))),
		// );
	})
}
