// Copyright 2019 Centrality Investments Limited

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use alloc::fmt::{self, Display, Formatter};
use pact::interpreter::types::PactType;

use crate::Domain;

/// Error which may occur while validating the permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum ValidationErr {
    NoPermission(Domain),
    ConstraintsInterpretation,
}

impl Display for ValidationErr {
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

pub trait Validate {
    fn validate(&self, module: &str, method: &str, args: &[PactType]) -> Result<(), ValidationErr>;
}
