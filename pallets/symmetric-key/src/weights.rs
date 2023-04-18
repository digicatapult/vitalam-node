//! Autogenerated weights for pallet_symmetric_key
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-14, STEPS: [], REPEAT: 1, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/dscp-node
// benchmark
// --pallet
// pallet_symmetric_key
// --extrinsic
// run_process
// --output
// ./weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};

use sp_std::marker::PhantomData;
use sp_std::prelude::*;

pub trait WeightInfo {
    fn update_key() -> Weight;
    fn rotate_key() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn update_key() -> Weight {
        (Weight::from_ref_time(2_000_000)).saturating_add(T::DbWeight::get().writes(1u64))
    }
    fn rotate_key() -> Weight {
        (Weight::from_ref_time(18_000_000))
            .saturating_add(T::DbWeight::get().reads(1u64))
            .saturating_add(T::DbWeight::get().writes(1u64))
    }
}

impl WeightInfo for () {
    fn update_key() -> Weight {
        Weight::from_ref_time(0)
    }
    fn rotate_key() -> Weight {
        Weight::from_ref_time(0)
    }
}
