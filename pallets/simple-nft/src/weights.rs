//! Autogenerated weights for pallet_simple_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-14, STEPS: [], REPEAT: 1, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/vitalam-node
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
    fn run_process(i: usize, o: usize) -> Weight;
}

/// Weight functions for pallet_simple_nft.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn run_process(i: usize, o: usize) -> Weight {
        (0 as Weight)
            // Standard Error: 3_008_000
            .saturating_add((14_764_000 as Weight).saturating_mul(i as Weight))
            // Standard Error: 3_008_000
            .saturating_add((10_806_000 as Weight).saturating_mul(o as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(o as Weight)))
    }
}

impl WeightInfo for () {
    fn run_process(_: usize, _: usize) -> Weight {
        (0 as Weight)
    }
}
