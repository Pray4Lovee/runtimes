use frame_support::storage::StorageMap;
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct WorkerInfo {
    pub wage_per_hour: u128,
    pub hours_worked: u32,
    pub pto_allocated: u32,
    pub pto_used: u32,
    pub last_epoch_paid: u64,
}

pub fn register_worker<T: frame_system::Config>(
    account: T::AccountId,
    wage_per_hour: u128,
    pto_allocated: u32
) -> frame_support::dispatch::DispatchResult {
    crate::Pallet::<T>::WorkerProfiles::insert(account, WorkerInfo {
        wage_per_hour,
        hours_worked: 0,
        pto_allocated,
        pto_used: 0,
        last_epoch_paid: 0,
    });
    Ok(())
}

pub fn calculate_net_amount(info: &WorkerInfo) -> u128 {
    (info.hours_worked as u128 + info.pto_used as u128) * info.wage_per_hour
}

pub fn remaining_pto(info: &WorkerInfo) -> u32 {
    info.pto_allocated - info.pto_used
}

