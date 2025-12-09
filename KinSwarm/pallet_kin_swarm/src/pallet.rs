#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::Currency,
};
use frame_system::pallet_prelude::*;
use sp_core::H256;

use crate::types::{Worker, LedgerEntry};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId>;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Map worker_id → Worker struct
    #[pallet::storage]
    pub type Workers<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, Worker<T::AccountId>, OptionQuery>;

    /// Ledger: worker_id → epoch → entry
    #[pallet::storage]
    pub type Ledger<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, LedgerEntry, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn epoch)]
    pub type Epoch<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    pub enum Event<T: Config> {
        WorkerUpdated(u32),
        EpochSettled(u32),
        Paid(u32, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        WorkerNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Insert or update a worker
        #[pallet::weight(10_000)]
        pub fn set_worker(origin: OriginFor<T>, worker: Worker<T::AccountId>) -> DispatchResult {
            ensure_signed(origin)?;

            Workers::<T>::insert(worker.id, &worker);
            Self::deposit_event(Event::WorkerUpdated(worker.id));

            Ok(())
        }

        /// End-of-epoch settlement: wages, PTO, ledger, payments
        #[pallet::weight(100_000)]
        pub fn settle_epoch(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let epoch = Epoch::<T>::get();

            for (id, w) in Workers::<T>::iter() {
                let mut worker = w.clone();

                let effective_hours =
                    worker.hours.saturating_sub(worker.pto_used);

                let pay =
                    worker.base_wage * effective_hours / worker.hours.max(1);

                let entry = LedgerEntry {
                    worker_id: id,
                    pay,
                    epoch,
                    timestamp: <frame_system::Pallet<T>>::block_number()
                        .saturated_into::<u64>(),
                };

                Ledger::<T>::insert(id, epoch, entry.clone());

                // Mutate PTO
                let pto_to_consume = core::cmp::min(worker.pto_balance, 2);
                worker.pto_balance = worker.pto_balance.saturating_sub(pto_to_consume);
                worker.pto_used += pto_to_consume;

                Workers::<T>::insert(id, worker.clone());

                // Pay the worker (on-chain native asset)
                let _ = T::Currency::deposit_creating(&worker.account, pay.into());

                Self::deposit_event(Event::Paid(id, pay));
            }

            Epoch::<T>::put(epoch + 1);
            Self::deposit_event(Event::EpochSettled(epoch));

            Ok(())
        }
    }
}
