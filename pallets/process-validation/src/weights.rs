
//! Autogenerated weights for `pallet_process_validation`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-06, STEPS: `50`, REPEAT: `1000`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-22-93.eu-west-2.compute.internal`, CPU: `AMD EPYC 7571`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/production/dscp-node
// benchmark
// pallet
// --pallet
// pallet_process_validation
// --extrinsic
// *
// --repeat
// 1000
// --output
// pallets/process-validation/src/weights.rs
// --steps
// 50

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use dscp_pallet_traits::ValidateProcessWeights;
use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

use crate::Config;

pub trait WeightInfo {
    fn create_process(i: u32) -> Weight;
    fn disable_process() -> Weight;
}

/// Weight functions for `pallet_process_validation`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config + Config> ValidateProcessWeights<u32> for SubstrateWeight<T> {
    /// Storage: ProcessValidation ProcessModel (r:1 w:0)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    /// The range of component `r` is `[1, 101]`.
    fn validate_process(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `136 + r * (14 ±0)`
        //  Estimated: `18813`
        // Minimum execution time: 10_850_000 picoseconds.
        Weight::from_parts(14_536_328, 0)
            .saturating_add(Weight::from_parts(0, 18813))
            // Standard Error: 157
            .saturating_add(Weight::from_parts(138_656, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(1))
    }
    /// Storage: ProcessValidation ProcessModel (r:1 w:0)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    fn validate_process_min() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `146`
        //  Estimated: `18813`
        // Minimum execution time: 11_040_000 picoseconds.
        Weight::from_parts(11_350_000, 0)
            .saturating_add(Weight::from_parts(0, 18813))
            .saturating_add(T::DbWeight::get().reads(1))
    }
    /// Storage: ProcessValidation ProcessModel (r:1 w:0)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    fn validate_process_max() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1550`
        //  Estimated: `18813`
        // Minimum execution time: 44_570_000 picoseconds.
        Weight::from_parts(45_570_000, 0)
            .saturating_add(Weight::from_parts(0, 18813))
            .saturating_add(T::DbWeight::get().reads(1))
    }
}

impl<T: frame_system::Config + Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: ProcessValidation VersionModel (r:1 w:1)
    /// Proof: ProcessValidation VersionModel (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
    /// Storage: ProcessValidation ProcessModel (r:1 w:1)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    /// The range of component `r` is `[1, 101]`.
    fn create_process(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `131`
        //  Estimated: `18813`
        // Minimum execution time: 22_721_000 picoseconds.
        Weight::from_parts(25_992_796, 0)
            .saturating_add(Weight::from_parts(0, 18813))
            // Standard Error: 129
            .saturating_add(Weight::from_parts(261_326, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: ProcessValidation ProcessModel (r:1 w:1)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    /// Storage: ProcessValidation VersionModel (r:1 w:0)
    /// Proof: ProcessValidation VersionModel (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
    fn disable_process() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `189`
        //  Estimated: `18813`
        // Minimum execution time: 29_020_000 picoseconds.
        Weight::from_parts(29_660_000, 0)
            .saturating_add(Weight::from_parts(0, 18813))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}

impl WeightInfo for () {
    fn create_process(_: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn disable_process() -> Weight {
        Weight::from_parts(0, 0)
    }
}
