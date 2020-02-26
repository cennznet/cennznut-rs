// Copyright (C) 2019-2020 Centrality Investments Limited
//! # CENNZnut
//!
//! Collection of versioned `CENNZnuts`
//!

use alloc::fmt::{self, Display, Formatter};

use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use pact::interpreter::types::PactType;

use crate::PartialDecode;
use crate::ValidationErr;

pub mod v0;

use core::convert::TryFrom;
use v0::CENNZnutV0;
use CENNZnut::V0;

pub type ContractAddress = [u8; 32];
pub const CONTRACT_WILDCARD: ContractAddress = [0u8; 32];
pub const WILDCARD: &str = "*";

/// A CENNZnet module permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeDomain {
    Method,
    MethodArguments,
    Module,
}

/// A CENNZnet contract permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum ContractDomain {
    Contract,
}

impl Display for RuntimeDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Method => write!(f, "method"),
            Self::MethodArguments => write!(f, "method arguments"),
            Self::Module => write!(f, "module"),
        }
    }
}

impl Display for ContractDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract => write!(f, "contract"),
        }
    }
}

#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub enum CENNZnut {
    V0(CENNZnutV0),
}

#[allow(unreachable_patterns)]
impl TryFrom<CENNZnut> for CENNZnutV0 {
    type Error = codec::Error;
    fn try_from(v: CENNZnut) -> Result<Self, Self::Error> {
        match v {
            V0(inner) => Ok(inner),
            _ => Err(codec::Error::from("CENNZnut version is not 0")),
        }
    }
}

impl Encode for CENNZnut {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        match &self {
            V0(inner) => inner.encode_to(buf),
        }
    }
}

impl Decode for CENNZnut {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let version = u16::from_le_bytes([
            input.read_byte()?.swap_bits(),
            input.read_byte()?.swap_bits(),
        ]);

        match version {
            0 => match CENNZnutV0::partial_decode(input) {
                Ok(inner) => Ok(V0(inner)),
                Err(e) => Err(e),
            },
            _ => Err(codec::Error::from("unexpected version")),
        }
    }
}

impl CENNZnut {
    fn validate_runtime_call(
        &self,
        module_name: &str,
        method_name: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<RuntimeDomain>> {
        match &self {
            V0(inner) => inner.validate_module(module_name, method_name, args),
        }
    }

    fn validate_contract_call(
        &self,
        contract_address: &ContractAddress,
    ) -> Result<(), ValidationErr<ContractDomain>> {
        match &self {
            V0(inner) => inner.validate_contract(*contract_address),
        }
    }
}
