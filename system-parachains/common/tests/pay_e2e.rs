#![cfg(test)]

use frame_support::traits::tokens::fungibles::{Create, Mutate};
use frame_support::dispatch::DispatchResult;
use sp_runtime::traits::AccountIdConversion;
use xcm::latest::prelude::*;
use parachains_common::pay::{LocalPay, VersionedLocatableAccount, VersionedLocatableAsset};

type Balance = u128;
type AssetId = xcm::v5::Location;
type Account = VersionedLocatableAccount;

struct DummyFungibles;

impl Create<AssetId> for DummyFungibles {
    type Balance = Balance;
    fn create(_asset: AssetId, _owner: [u8;32], _is_sufficient: bool, _amount: Self::Balance) -> DispatchResult {
        Ok(())
    }
}

impl Mutate<AssetId> for DummyFungibles {
    type Balance = Balance;
    fn mint_into(_asset: AssetId, _who: &[u8;32], _amount: Self::Balance) -> DispatchResult { Ok(()) }
    fn transfer(_asset: AssetId, _from: &[u8;32], _to: &[u8;32], _amount: Self::Balance, _preservation: xcm_executor::traits::Preservation) -> DispatchResult { Ok(()) }
}

struct DummyConverter;

impl xcm_executor::traits::ConvertLocation<[u8;32]> for DummyConverter {
    fn convert_location(location: [u8;32]) -> Option<[u8;32]> { Some(location) }
}

#[test]
fn test_v3_asset_payment() {
    let beneficiary = VersionedLocatableAccount::V4 { location: Location::here(), account_id: [0u8;32] };
    let asset = VersionedLocatableAsset::V3 { location: Location::here(), asset_id: Box::new([1u8;32]) };
    assert!(LocalPay::<DummyFungibles, Account, DummyConverter>::pay(&beneficiary, asset, 1000).is_ok());
}

#[test]
fn test_v4_asset_payment() {
    let beneficiary = VersionedLocatableAccount::V5 { location: Location::here(), account_id: [0u8;32] };
    let asset = VersionedLocatableAsset::V4 { location: Location::here(), asset_id: Box::new([2u8;32]) };
    assert!(LocalPay::<DummyFungibles, Account, DummyConverter>::pay(&beneficiary, asset, 500).is_ok());
}

#[test]
fn test_v5_asset_payment() {
    let beneficiary = VersionedLocatableAccount::V5 { location: Location::here(), account_id: [1u8;32] };
    let asset = VersionedLocatableAsset::V5 { location: Location::here(), asset_id: Box::new([3u8;32]) };
    assert!(LocalPay::<DummyFungibles, Account, DummyConverter>::pay(&beneficiary, asset, 750).is_ok());
}
