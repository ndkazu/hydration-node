// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=frame-system
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// system.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use frame_system::weights::WeightInfo;

/// Weights for frame_system using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32) -> Weight {
		// Minimum execution time: 1_437 nanoseconds.
		Weight::from_ref_time(1_509_000 as u64) // Standard Error: 2
			.saturating_add(Weight::from_ref_time(307 as u64).saturating_mul(b as u64))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32) -> Weight {
		// Minimum execution time: 4_488 nanoseconds.
		Weight::from_ref_time(4_630_000 as u64) // Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_281 as u64).saturating_mul(b as u64))
	}
	// Storage: System Digest (r:1 w:1)
	// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: unknown `0x3a686561707061676573` (r:0 w:1)
	// Proof Skipped: unknown `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 3_026 nanoseconds.
		Weight::from_ref_time(3_264_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32) -> Weight {
		// Minimum execution time: 1_512 nanoseconds.
		Weight::from_ref_time(1_538_000 as u64) // Standard Error: 3_787
			.saturating_add(Weight::from_ref_time(488_086 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32) -> Weight {
		// Minimum execution time: 1_396 nanoseconds.
		Weight::from_ref_time(1_443_000 as u64) // Standard Error: 3_105
			.saturating_add(Weight::from_ref_time(410_815 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32) -> Weight {
		// Minimum execution time: 3_179 nanoseconds.
		Weight::from_ref_time(3_267_000 as u64) // Standard Error: 3_091
			.saturating_add(Weight::from_ref_time(1_015_659 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}
