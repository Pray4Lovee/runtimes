#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use frame_support::{assert_ok, assert_noop};
    use sp_core::H256;

    #[test]
    fn test_worker_insert_and_epoch() {
        new_test_ext().execute_with(|| {
            let w = Worker {
                id: 1,
                wallet_id: H256::zero(),
                base_wage: 100,
                hours: 10,
                pto_balance: 4,
                pto_used: 0,
                account: 1,
            };

            assert_ok!(KinSwarm::set_worker(RuntimeOrigin::signed(1), w));

            assert_ok!(KinSwarm::settle_epoch(RuntimeOrigin::signed(1)));

            assert!(Ledger::<Test>::contains_key(1, 0));
        });
    }
}
