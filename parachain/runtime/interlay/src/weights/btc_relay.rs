
//! Autogenerated weights for btc_relay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-21, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `enterprise`, CPU: `Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// btc-relay
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 100
// --repeat
// 10
// --output
// parachain/runtime/interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for btc_relay using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> btc_relay::WeightInfo for WeightInfo<T> {
	/// Storage: BTCRelay BestBlock (r:1 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:1)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: BTCRelay StartBlockHeight (r:0 w:1)
	/// Proof: BTCRelay StartBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:0 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:0 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	fn initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1347`
		//  Estimated: `4520`
		// Minimum execution time: 57_809_000 picoseconds.
		Weight::from_parts(59_917_000, 4520)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: BTCRelay BlockHeaders (r:5 w:3)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:3 w:2)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:3 w:4)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:7 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableBitcoinConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:0 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 6]`.
	fn store_block_header(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2532 + r * (100 ±0)`
		//  Estimated: `35730 + r * (1769 ±29)`
		// Minimum execution time: 155_780_000 picoseconds.
		Weight::from_parts(235_557_071, 35730)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(11_u64))
			.saturating_add(Weight::from_parts(0, 1769).saturating_mul(r.into()))
	}
}