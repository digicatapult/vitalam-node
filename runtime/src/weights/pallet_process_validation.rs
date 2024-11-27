
//! Autogenerated weights for `pallet_process_validation`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-11-26, STEPS: `50`, REPEAT: `100`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `FNQGF7746D.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/sqnc-node
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --repeat
// 100
// --output
// ./runtime/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_process_validation`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_process_validation::WeightInfo for WeightInfo<T> {
	/// Storage: `ProcessValidation::VersionModel` (r:1 w:1)
	/// Proof: `ProcessValidation::VersionModel` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:1)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 251]`.
	fn create_process(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131`
		//  Estimated: `41613`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(10_962_213, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			// Standard Error: 200
			.saturating_add(Weight::from_parts(217_866, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:1)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// Storage: `ProcessValidation::VersionModel` (r:1 w:0)
	/// Proof: `ProcessValidation::VersionModel` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn disable_process() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `41613`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(16_000_000, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 251]`.
	fn validate_process(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136 + r * (14 ±0)`
		//  Estimated: `41613`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_940_348, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			// Standard Error: 157
			.saturating_add(Weight::from_parts(149_739, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	fn validate_process_min() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `41613`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	fn validate_process_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3650`
		//  Estimated: `41613`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(43_000_000, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}