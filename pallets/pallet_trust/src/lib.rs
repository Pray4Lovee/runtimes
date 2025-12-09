#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::Get, storage::StorageMap};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_core::H256;
use xcm::v3::prelude::*;
use xcm::VersionedMultiLocation;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, Debug, TypeInfo)]
    pub enum TrustedAssetKind {
        Fungible,
        Bonded,
    }

    #[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, Debug, TypeInfo)]
    pub struct TrustedAssetId {
        pub id: H256,
        pub kind: TrustedAssetKind,
    }

    #[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, Debug, TypeInfo)]
    pub struct TrustScore(pub u32);

    #[pallet::storage]
    #[pallet::getter(fn trust_score)]
    pub type TrustLedger<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, TrustScore, OptionQuery>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TrustOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type MaxMint: Get<u128>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TrustAssigned(T::AccountId, u32),
        AssetMinted(T::AccountId, TrustedAssetId, u128),
    }

    #[pallet::error]
    pub enum Error<T> {
        NotTrusted,
        AmountExceedsMax,
        ConversionFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::set_trust())]
        pub fn set_trust(origin: OriginFor<T>, who: T::AccountId, score: u32) -> DispatchResult {
            T::TrustOrigin::ensure_origin(origin)?;
            TrustLedger::<T>::insert(&who, TrustScore(score));
            Self::deposit_event(Event::TrustAssigned(who, score));
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::mint_trusted_asset())]
        pub fn mint_trusted_asset(
            origin: OriginFor<T>,
            asset: TrustedAssetId,
            amount: u128,
            loc: VersionedMultiLocation,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let trust = TrustLedger::<T>::get(&who).unwrap_or(TrustScore(0));
            ensure!(trust.0 > 0, Error::<T>::NotTrusted);
            ensure!(amount <= T::MaxMint::get(), Error::<T>::AmountExceedsMax);

            let _v5_location: MultiLocation = match loc {
                VersionedMultiLocation::V3(l) => l.try_into().map_err(|_| Error::<T>::ConversionFailed)?,
                VersionedMultiLocation::V4(l) => l.try_into().map_err(|_| Error::<T>::ConversionFailed)?,
                VersionedMultiLocation::V5(l) => l,
            };

            Self::deposit_event(Event::AssetMinted(who, asset, amount));
            Ok(())
        }
    }

    #[pallet::weight]
    pub trait WeightInfo {
        fn set_trust() -> Weight;
        fn mint_trusted_asset() -> Weight;
    }
}
