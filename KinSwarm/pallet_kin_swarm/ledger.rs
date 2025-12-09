use sp_std::vec::Vec;

pub fn record_ledger<T>(_account: T, _amount: u128, payload: Vec<u8>) {
    let mut ledger = crate::Pallet::<T>::EpochLedger::get(0u64);
    ledger.extend(payload);
    crate::Pallet::<T>::EpochLedger::insert(0u64, ledger);
}
