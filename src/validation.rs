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

// use crate::Domain;

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

pub trait Validate<Domain: Display> {
    fn validate(
        &self,
        module: &str,
        method: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<Domain>>;
}
