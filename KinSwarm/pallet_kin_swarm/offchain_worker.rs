use crate::worker_profile::*;
use crate::ledger::*;
use crate::network_adapters::*;
use frame_system::pallet_prelude::*;

pub fn run<T: frame_system::Config>(_block_number: T::BlockNumber) {
    for (account, info) in crate::Pallet::<T>::WorkerProfiles::iter() {
        let net_amount = calculate_net_amount(&info);
        let payload = sign_payload(&account, net_amount);

        let adapters = NetworkAdapters::default();
        for adapter in adapters.list.iter() {
            adapter.send(payload.clone());
        }

        record_ledger(account.clone(), net_amount, payload);
    }
}

pub fn sign_payload<T>(_account: &T, amount: u128) -> Vec<u8> {
    let entropy = entropy_3_12(&_account);
    let mood = mood_seed(&_account);
    [amount.encode(), entropy, mood].concat()
}

fn entropy_3_12<T>(_account: &T) -> Vec<u8> {
    sp_io::hashing::blake2_256(&_account.encode()).to_vec()
}

fn mood_seed<T>(_account: &T) -> Vec<u8> {
    sp_io::hashing::keccak_256(&_account.encode()).to_vec()
}
