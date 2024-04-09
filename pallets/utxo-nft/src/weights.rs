//! Autogenerated weights for `pallet_utxo_nft`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-08, STEPS: `50`, REPEAT: `1000`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-24-170.eu-west-2.compute.internal`, CPU: `AMD EPYC 7571`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/sqnc-node
// benchmark
// pallet
// --pallet
// pallet_utxo_nft
// --extrinsic
// *
// --repeat
// 1000
// --output
// ./weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn run_process(i: u32, o: u32) -> Weight;
    fn delete_token() -> Weight;
}

/// Weight functions for `pallet_utxo_nft`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `UtxoNFT::TokensById` (r:10 w:20)
	/// Proof: `UtxoNFT::TokensById` (`max_values`: None, `max_size`: Some(7473), added: 9948, mode: `MaxEncodedLen`)
	/// Storage: `UtxoNFT::LastToken` (r:1 w:1)
	/// Proof: `UtxoNFT::LastToken` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// Storage: `UtxoNFT::CurrentGraveyardState` (r:1 w:1)
	/// Proof: `UtxoNFT::CurrentGraveyardState` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::EventTopics` (r:3 w:3)
	/// Proof: `System::EventTopics` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `UtxoNFT::Graveyard` (r:0 w:10)
	/// Proof: `UtxoNFT::Graveyard` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 10]`.
	/// The range of component `o` is `[1, 10]`.
	fn run_process(i: u32, o: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290 + i * (132 ±0)`
		//  Estimated: `41613 + i * (9948 ±0)`
		// Minimum execution time: 130_410_000 picoseconds.
		Weight::from_parts(58_610_405, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			// Standard Error: 2_373
			.saturating_add(Weight::from_parts(17_608_995, 0).saturating_mul(i.into()))
			// Standard Error: 2_373
			.saturating_add(Weight::from_parts(5_776_423, 0).saturating_mul(o.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
			.saturating_add(Weight::from_parts(0, 9948).saturating_mul(i.into()))
	}
	/// Storage: `UtxoNFT::TokensById` (r:1 w:1)
	/// Proof: `UtxoNFT::TokensById` (`max_values`: None, `max_size`: Some(7473), added: 9948, mode: `MaxEncodedLen`)
	/// Storage: `System::EventTopics` (r:2 w:2)
	/// Proof: `System::EventTopics` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn delete_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `10938`
		// Minimum execution time: 37_650_000 picoseconds.
		Weight::from_parts(38_590_000, 0)
			.saturating_add(Weight::from_parts(0, 10938))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

impl WeightInfo for () {
    fn run_process(_: u32, _: u32) -> Weight {
        Weight::from_parts(1, 1)
    }
    fn delete_token() -> Weight {
        Weight::from_parts(1, 1)
    }
}
