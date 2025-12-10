// SPDX-License-Identifier: Apache-2.0
//! Local implementation of the pay module with V3 to V5 conversion support.

use frame_support::traits::{
    fungibles,
    tokens::{PaymentStatus, Preservation},
};
use parachains_common::pay::VersionedLocatableAccount;
use polkadot_runtime_common::impls::VersionedLocatableAsset;
use sp_runtime::{traits::TypedGet, DispatchError};
use xcm::latest::prelude::*;
use xcm_executor::traits::ConvertLocation;

/// Local pay on chain
pub struct LocalPay<F, A, C>(core::marker::PhantomData<(F, A, C)>);

impl<A, F, C> frame_support::traits::tokens::Pay for LocalPay<F, A, C>
where
    A: TypedGet,
    F: fungibles::Mutate<A::Type, AssetId = xcm::v5::Location> + fungibles::Create<A::Type>,
    C: ConvertLocation<A::Type>,
    A::Type: Eq + Clone,
{
    type Balance = F::Balance;
    type Beneficiary = VersionedLocatableAccount;
    type AssetKind = VersionedLocatableAsset;
    type Id = QueryId;
    type Error = DispatchError;

    fn pay(who: &Self::Beneficiary, asset: Self::AssetKind, amount: Self::Balance) -> Result<Self::Id, Self::Error> {
        let who = Self::match_location(who).map_err(|_| DispatchError::Unavailable)?;
        let asset = Self::match_asset(&asset).map_err(|_| DispatchError::Unavailable)?;
        <F as fungibles::Mutate<_>>::transfer(asset, &A::get(), &who, amount, Preservation::Expendable)?;
        Ok(Self::Id::MAX)
    }

    fn check_payment(_: Self::Id) -> PaymentStatus {
        PaymentStatus::Success
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn ensure_successful(_: &Self::Beneficiary, asset: Self::AssetKind, amount: Self::Balance) {
        let asset = Self::match_asset(&asset).expect("invalid asset");
        <F as fungibles::Create<_>>::create(asset.clone(), A::get(), true, amount).unwrap();
        <F as fungibles::Mutate<_>>::mint_into(asset, &A::get(), amount).unwrap();
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn ensure_concluded(_: Self::Id) {}
}

impl<A, F, C> LocalPay<F, A, C>
where
    A: TypedGet,
    F: fungibles::Mutate<A::Type> + fungibles::Create<A::Type>,
    C: ConvertLocation<A::Type>,
    A::Type: Eq + Clone,
{
    fn match_location(who: &VersionedLocatableAccount) -> Result<A::Type, ()> {
        match who {
            VersionedLocatableAccount::V4 { location, account_id } if location.is_here() =>
                &account_id.clone().try_into().map_err(|_| ())?,
            VersionedLocatableAccount::V5 { location, account_id } if location.is_here() =>
                account_id,
            _ => return Err(()),
        };
        C::convert_location(account_id).ok_or(())
    }

    fn match_asset(asset: &VersionedLocatableAsset) -> Result<xcm::v5::Location, ()> {
        match asset {
            VersionedLocatableAsset::V3 { location, asset_id } if location.is_here() => {
                let v4_asset_id: xcm::v4::AssetId = (*asset_id).try_into().map_err(|_| ())?;
                let v5_asset_id: xcm::v5::AssetId = v4_asset_id.try_into().map_err(|_| ())?;
                Ok(v5_asset_id.0)
            },
            VersionedLocatableAsset::V4 { location, asset_id } if location.is_here() =>
                asset_id.clone().try_into().map(|a: xcm::v5::AssetId| a.0).map_err(|_| ()),
            VersionedLocatableAsset::V5 { location, asset_id } if location.is_here() =>
                Ok(asset_id.clone().0),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarks {
    use super::*;
    use core::marker::PhantomData;
    use frame_support::traits::Get;
    use pallet_treasury::ArgumentsFactory as TreasuryArgumentsFactory;
    use sp_core::ConstU8;

    pub struct LocalPayArguments<PalletId = ConstU8<0>>(PhantomData<PalletId>);
    impl<PalletId: Get<u8>> TreasuryArgumentsFactory<VersionedLocatableAsset, VersionedLocatableAccount>
        for LocalPayArguments<PalletId>
    {
        fn create_asset_kind(seed: u32) -> VersionedLocatableAsset {
            VersionedLocatableAsset::V5 {
                location: Location::new(0, []),
                asset_id: Location::new(0, [PalletInstance(PalletId::get()), GeneralIndex(seed.into())]).into(),
            }
        }

        fn create_beneficiary(seed: [u8; 32]) -> VersionedLocatableAccount {
            VersionedLocatableAccount::V5 {
                location: Location::new(0, []),
                account_id: Location::new(0, [AccountId32 { network: None, id: seed }]),
            }
        }
    }
}
