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

//! Autogenerated weights for orml_tokens
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=orml-tokens
// --extra
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// tokens.rs
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

use orml_tokens::weights::WeightInfo;

/// Weights for orml_tokens using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn transfer() -> Weight {
        // Minimum execution time: 53_663 nanoseconds.
        Weight::from_ref_time(54_165_000 as u64)            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn transfer_all() -> Weight {
        // Minimum execution time: 54_175 nanoseconds.
        Weight::from_ref_time(54_699_000 as u64)            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn transfer_keep_alive() -> Weight {
        // Minimum execution time: 47_068 nanoseconds.
        Weight::from_ref_time(47_482_000 as u64)            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn force_transfer() -> Weight {
        // Minimum execution time: 48_447 nanoseconds.
        Weight::from_ref_time(49_406_000 as u64)            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn set_balance() -> Weight {
        // Minimum execution time: 43_051 nanoseconds.
        Weight::from_ref_time(45_089_000 as u64)            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
}
