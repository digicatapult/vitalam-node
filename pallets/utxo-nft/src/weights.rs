
//! Autogenerated weights for `pallet_utxo_nft`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-07, STEPS: `50`, REPEAT: `1000`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-22-93.eu-west-2.compute.internal`, CPU: `AMD EPYC 7571`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/production/dscp-node
// benchmark
// pallet
// --pallet
// pallet_utxo_nft
// --extrinsic
// *
// --repeat
// 1000
// --output
// pallets/utxo-nft/src/weights.rs
// --steps
// 50

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn run_process(i: u32, o: u32) -> Weight;
}

/// Weight functions for `pallet_utxo_nft`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: UtxoNFT TokensById (r:10 w:20)
	/// Proof: UtxoNFT TokensById (max_values: None, max_size: Some(6961), added: 9436, mode: MaxEncodedLen)
	/// Storage: UtxoNFT LastToken (r:1 w:1)
	/// Proof: UtxoNFT LastToken (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: ProcessValidation ProcessModel (r:1 w:0)
	/// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
	/// Storage: System EventTopics (r:3 w:3)
	/// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[1, 10]`.
	/// The range of component `o` is `[1, 10]`.
	fn run_process(i: u32, o: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254 + i * (132 ±0)`
		//  Estimated: `18813 + i * (9436 ±0)`
		// Minimum execution time: 77_721_000 picoseconds.
		Weight::from_parts(39_519_277, 0)
			.saturating_add(Weight::from_parts(0, 18813))
			// Standard Error: 2_272
			.saturating_add(Weight::from_parts(9_148_646, 0).saturating_mul(i.into()))
			// Standard Error: 2_272
			.saturating_add(Weight::from_parts(3_105_724, 0).saturating_mul(o.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
			.saturating_add(Weight::from_parts(0, 9436).saturating_mul(i.into()))
	}
}

impl WeightInfo for () {
	fn run_process(_: u32, _: u32) -> Weight {
			Weight::from_parts(0, 0)
	}
}