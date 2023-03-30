//! Autogenerated weights for pallet_simple_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-14, STEPS: [], REPEAT: 1, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/dscp-node
// benchmark
// --pallet
// pallet_simple_nft
// --extrinsic
// run_process
// --output
// ./weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn run_process(i: u32, o: u32) -> Weight;
}

/// Weight functions for pallet_simple_nft.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn run_process(i: u32, o: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `176 + i * (165 ±0)`
        //  Estimated: `25936 + i * (9601 ±0)`
        // Minimum execution time: 67_000 nanoseconds.
        Weight::from_ref_time(32_372_888)
            .saturating_add(Weight::from_proof_size(25936))
            // Standard Error: 7_804
            .saturating_add(Weight::from_ref_time(7_291_111).saturating_mul(i.into()))
            // Standard Error: 7_804
            .saturating_add(Weight::from_ref_time(2_888_000).saturating_mul(o.into()))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
            .saturating_add(T::DbWeight::get().writes(4))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
            .saturating_add(Weight::from_proof_size(9601).saturating_mul(i.into()))
    }
}

impl WeightInfo for () {
    fn run_process(_: u32, _: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
    }
}
