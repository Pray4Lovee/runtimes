// SPDX-License-Identifier: Apache-2.0
// Copyright (C) Parity Technologies (UK) Ltd.

use crate::*;
use frame_support::traits::{tokens::UnityOrOuterConversion, FromContains};
use parachains_common::pay::VersionedLocatableAccount;
use polkadot_runtime_common::impls::{ContainsParts, VersionedLocatableAsset};
use frame_support::parameter_types;
use frame_support::PalletId;
use sp_runtime::Permill;
use pallet_treasury;
use pallet_bounties;
use pallet_child_bounties;
use xcm_config;
use system_parachains_common::pay::LocalPay;

// ----------------------------
// Treasury configuration
// ----------------------------
parameter_types! {
    pub const SpendPeriod: BlockNumber = 24 * RC_DAYS;
    pub const DisableSpends: BlockNumber = BlockNumber::MAX;
    pub const Burn: Permill = Permill::from_percent(1);
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const PayoutSpendPeriod: BlockNumber = 90 * RC_DAYS;
    pub const MaxApprovals: u32 = 100;
    // Account address: `13UVJyLnbVp9RBZYFwFGyDvVd1y27Tt8tkntv6Q7JVPhFsTB`
    pub TreasuryAccount: AccountId = Treasury::account_id();
}

// LocalPay implementation using the temporary module
pub type TreasuryPaymaster = LocalPay<
    NativeAndAssets,
    TreasuryAccount,
    xcm_config::LocationToAccountId,
>;

impl pallet_treasury::Config for Runtime {
    type PalletId = TreasuryPalletId;
    type Currency = Balances;
    type RejectOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
    type RuntimeEvent = RuntimeEvent;
    type SpendPeriod = pallet_ah_migrator::LeftOrRight<AhMigrator, DisableSpends, SpendPeriod>;
    type Burn = Burn;
    type BurnDestination = ();
    type SpendFunds = Bounties;
    type MaxApprovals = MaxApprovals;
    type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
    type SpendOrigin = TreasurySpender;
    type AssetKind = VersionedLocatableAsset;
    type Beneficiary = VersionedLocatableAccount;
    type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
    type Paymaster = TreasuryPaymaster;
    type BalanceConverter = AssetRateWithNative;
    type PayoutPeriod = PayoutSpendPeriod;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = system_parachains_common::pay::benchmarks::LocalPayArguments<
        xcm_config::TrustBackedAssetsPalletIndex,
    >;
    type BlockNumberProvider = RelaychainDataProvider<Runtime>;
}

// ----------------------------
// Bounties configuration
// ----------------------------
parameter_types! {
    pub const BountyDepositBase: Balance = system_para_deposit(0, 176);
    pub const DataDepositPerByte: Balance = system_para_deposit(0, 1);
    pub const BountyDepositPayoutDelay: BlockNumber = 0;
    pub const BountyUpdatePeriod: BlockNumber = 10 * 12 * 30 * RC_DAYS;
    pub const MaximumReasonLength: u32 = 16384;
    pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
    pub const CuratorDepositMin: Balance = 10 * DOLLARS;
    pub const CuratorDepositMax: Balance = 200 * DOLLARS;
    pub const BountyValueMinimum: Balance = 10 * DOLLARS;
}

impl pallet_bounties::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type BountyDepositBase = BountyDepositBase;
    type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
    type BountyUpdatePeriod = BountyUpdatePeriod;
    type CuratorDepositMultiplier = CuratorDepositMultiplier;
    type CuratorDepositMin = CuratorDepositMin;
    type CuratorDepositMax = CuratorDepositMax;
    type BountyValueMinimum = BountyValueMinimum;
    type ChildBountyManager = ChildBounties;
    type DataDepositPerByte = DataDepositPerByte;
    type MaximumReasonLength = MaximumReasonLength;
    type OnSlash = Treasury;
    type WeightInfo = weights::pallet_bounties::WeightInfo<Runtime>;
}

// ----------------------------
// Child bounties configuration
// ----------------------------
parameter_types! {
    pub const MaxActiveChildBountyCount: u32 = 100;
    pub const ChildBountyValueMinimum: Balance = BountyValueMinimum::get() / 10;
}

impl pallet_child_bounties::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
    type ChildBountyValueMinimum = ChildBountyValueMinimum;
    type WeightInfo = weights::pallet_child_bounties::WeightInfo<Runtime>;
}

// ----------------------------
// Asset rate converter with native decorations
// ----------------------------
pub type AssetRateWithNative = UnityOrOuterConversion<
    ContainsParts<
        FromContains<
            (
                xcm_builder::IsSiblingSystemParachain<ParaId, xcm_config::SelfParaId>,
                Equals<xcm_config::Here>,
            >,
            xcm_builder::IsParentsOnly<ConstU8<1>>,
        >,
    >,
    AssetRate,
>;

impl pallet_asset_rate::Config for Runtime {
    type WeightInfo = weights::pallet_asset_rate::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type CreateOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
    type RemoveOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
    type UpdateOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
    type Currency = Balances;
    type AssetKind = <Runtime as pallet_treasury::Config>::AssetKind;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = polkadot_runtime_common::impls::benchmarks::AssetRateArguments;
}
