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

//! Autogenerated weights for `pallet_ema_oracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-12, STEPS: `5`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-ema-oracle
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// ./weights-1.1.0/ema_oracle.rs
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

/// Weight functions needed for pallet_ema_oracle.
pub trait WeightInfo {
	fn on_finalize_no_entry() -> Weight;
	fn on_finalize_multiple_tokens(b: u32) -> Weight;
	fn on_trade_multiple_tokens(b: u32) -> Weight;
	fn on_liquidity_changed_multiple_tokens(b: u32) -> Weight;
	fn get_entry() -> Weight;
}

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	/// Storage: `EmaOracle::Accumulator` (r:1 w:0)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn on_finalize_no_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `7406`
		// Minimum execution time: 3_261_000 picoseconds.
		Weight::from_parts(3_342_000, 7406).saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:117 w:117)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_finalize_multiple_tokens(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270 + b * (626 ±0)`
		//  Estimated: `7406 + b * (7956 ±0)`
		// Minimum execution time: 49_005_000 picoseconds.
		Weight::from_parts(11_980_224, 7406)
			// Standard Error: 17_790
			.saturating_add(Weight::from_parts(36_916_571, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7956).saturating_mul(b.into()))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_trade_multiple_tokens(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `718 + b * (163 ±0)`
		//  Estimated: `7406`
		// Minimum execution time: 18_948_000 picoseconds.
		Weight::from_parts(19_508_272, 7406)
			// Standard Error: 3_859
			.saturating_add(Weight::from_parts(435_799, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_liquidity_changed_multiple_tokens(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `718 + b * (163 ±0)`
		//  Estimated: `7406`
		// Minimum execution time: 18_875_000 picoseconds.
		Weight::from_parts(19_397_429, 7406)
			// Standard Error: 4_124
			.saturating_add(Weight::from_parts(438_627, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `604`
		//  Estimated: `6294`
		// Minimum execution time: 18_746_000 picoseconds.
		Weight::from_parts(19_158_000, 6294).saturating_add(T::DbWeight::get().reads(2))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `EmaOracle::Accumulator` (r:1 w:0)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn on_finalize_no_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `7406`
		// Minimum execution time: 3_261_000 picoseconds.
		Weight::from_parts(3_342_000, 7406).saturating_add(RocksDbWeight::get().reads(1))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:117 w:117)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_finalize_multiple_tokens(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270 + b * (626 ±0)`
		//  Estimated: `7406 + b * (7956 ±0)`
		// Minimum execution time: 49_005_000 picoseconds.
		Weight::from_parts(11_980_224, 7406)
			// Standard Error: 17_790
			.saturating_add(Weight::from_parts(36_916_571, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7956).saturating_mul(b.into()))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_trade_multiple_tokens(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `718 + b * (163 ±0)`
		//  Estimated: `7406`
		// Minimum execution time: 18_948_000 picoseconds.
		Weight::from_parts(19_508_272, 7406)
			// Standard Error: 3_859
			.saturating_add(Weight::from_parts(435_799, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_liquidity_changed_multiple_tokens(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `718 + b * (163 ±0)`
		//  Estimated: `7406`
		// Minimum execution time: 18_875_000 picoseconds.
		Weight::from_parts(19_397_429, 7406)
			// Standard Error: 4_124
			.saturating_add(Weight::from_parts(438_627, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `604`
		//  Estimated: `6294`
		// Minimum execution time: 18_746_000 picoseconds.
		Weight::from_parts(19_158_000, 6294).saturating_add(RocksDbWeight::get().reads(2))
	}
}
