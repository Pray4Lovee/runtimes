#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn set_trust() -> Weight;
    fn mint_trusted_asset() -> Weight;
}

impl WeightInfo for () {
    fn set_trust() -> Weight { 10_000 }
    fn mint_trusted_asset() -> Weight { 50_000 }
}
