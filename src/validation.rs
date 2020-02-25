// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Validation
//!
//! Validation trait of CENNZnut for use in CENNZnet
//!

#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use alloc::fmt::{self, Display, Formatter};
use pact::interpreter::types::PactType;

/// Error which may occur while validating the permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum ValidationErr<Domain: Display> {
    NoPermission(Domain),
    ConstraintsInterpretation,
}

impl<Domain: Display> Display for ValidationErr<Domain> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPermission(permission_domain) => write!(
                f,
                "CENNZnut does not grant permission for {}",
                permission_domain
            ),
            Self::ConstraintsInterpretation => write!(f, "error while interpreting constraints"),
        }
    }
}

pub trait Validate<RuntimeDomain: Display, ContractDomain: Display> {
    /// Legacy interface for validating a CENNZnut runtime module
    ///
    /// # Errors
    ///
    /// On failure, returns an error embedded in `RuntimeDomain`
    fn validate(
        &self,
        module: &str,
        method: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<RuntimeDomain>>;

    /// Interface for validating a CENNZnut runtime module
    ///
    /// # Errors
    ///
    /// On failure, returns an error embedded in `RuntimeDomain`
    fn validate_runtime_call(
        &self,
        module: &str,
        method: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<RuntimeDomain>>;

    /// Interface for validating a CENNZnut smart contract
    ///
    /// # Errors
    ///
    /// On failure, returns an error embedded in `ContractDomain`
    fn validate_contract_call(
        &self,
        contract: &[u8; 32],
        method: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<ContractDomain>>;
}
