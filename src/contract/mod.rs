// RGB Core Library: consensus layer for RGB smart contracts.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2019-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use bp::seals::txout::blind::{BlindSeal, SecretSeal};

pub mod fungible;
pub mod attachment;
pub mod data;
pub mod assignment;
mod global;
mod operations;
mod bundle;
pub mod state;
mod contract;

pub use assignment::{
    Assign, AssignAttach, AssignData, AssignFungible, AssignRights, StateCommitment, StateData,
    StateType, TypedAssigns,
};
pub use attachment::AttachId;
use bp::seals::txout::TxoSeal;
pub use bundle::{BundleId, TransitionBundle};
pub use contract::ContractContainer;
pub use fungible::{
    BlindingFactor, FieldOrderOverflow, FungibleState, NoiseDumb, PedersenCommitment, RangeProof,
    RangeProofError,
};
pub use global::{GlobalState, GlobalValues};
pub use operations::{
    ContractId, Extension, Genesis, OpId, OpRef, Operation, OwnedState, PrevOuts, Redeemed,
    Transition, Valencies,
};
pub use state::ContractState;

/// Marker trait for types of state which are just a commitment to the actual
/// state data.
pub trait ConfidentialState:
    core::fmt::Debug
    + core::hash::Hash
    + strict_encoding::StrictDumb
    + strict_encoding::StrictEncode
    + strict_encoding::StrictDecode
    + Eq
    + Copy
{
    fn state_type(&self) -> StateType;
    fn state_commitment(&self) -> StateCommitment;
}

/// Marker trait for types of state holding explicit state data.
pub trait ExposedState:
    core::fmt::Debug
    + strict_encoding::StrictDumb
    + strict_encoding::StrictEncode
    + strict_encoding::StrictDecode
    + commit_verify::Conceal<Concealed = Self::Confidential>
    + Eq
    + Ord
    + Clone
{
    type Confidential: ConfidentialState;
    fn state_type(&self) -> StateType;
    fn state_data(&self) -> StateData;
}

pub trait ConfidentialSeal:
    core::fmt::Debug
    + core::hash::Hash
    + strict_encoding::StrictDumb
    + strict_encoding::StrictEncode
    + strict_encoding::StrictDecode
    + Eq
    + Ord
    + Copy
{
}

pub trait ExposedSeal:
    core::fmt::Debug
    + strict_encoding::StrictDumb
    + strict_encoding::StrictEncode
    + strict_encoding::StrictDecode
    + commit_verify::Conceal<Concealed = Self::Confidential>
    + Eq
    + Ord
    + Copy
    + TxoSeal
{
    type Confidential: ConfidentialSeal;
}

impl ExposedSeal for BlindSeal {
    type Confidential = SecretSeal;
}

impl ConfidentialSeal for SecretSeal {}
