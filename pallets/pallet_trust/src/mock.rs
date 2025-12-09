#![cfg(test)]

use crate::*;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system,
        Trust: pallet::<Runtime>,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaxMint: u128 = 1_000_000;
}

impl system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type Header = Header;
    type Index = u64;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
