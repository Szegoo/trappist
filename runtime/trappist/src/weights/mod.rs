// This file is part of Trappist.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ::trappist_runtime_benchmarks::WeightInfo;
use frame_support::weights::Weight;
use xcm_primitives::DropAssetsWeigher;

use crate::Runtime;

pub mod block_weights;
pub mod cumulus_pallet_xcmp_queue;
pub mod extrinsic_weights;
pub mod frame_system;
pub mod pallet_asset_registry;
pub mod pallet_assets;
pub mod pallet_balances;
pub mod pallet_collator_selection;
pub mod pallet_collective;
pub mod pallet_contracts;
pub mod pallet_democracy;
pub mod pallet_identity;
pub mod pallet_multisig;
pub mod pallet_preimage;
pub mod pallet_safe_mode;
pub mod pallet_scheduler;
pub mod pallet_session;
pub mod pallet_timestamp;
pub mod pallet_tx_pause;
pub mod pallet_uniques;
pub mod pallet_utility;
pub mod pallet_withdraw_teleport;
pub mod trappist_runtime_benchmarks;
pub mod xcm;

pub struct TrappistDropAssetsWeigher();
impl DropAssetsWeigher for TrappistDropAssetsWeigher {
	fn fungible() -> Weight {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_fungible()
	}

	fn native() -> Weight {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_native()
	}

	fn default() -> Weight {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_default()
	}
}
