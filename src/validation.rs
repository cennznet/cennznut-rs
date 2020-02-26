// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Validation
//!
//! Validation trait of CENNZnut for use in CENNZnet
//!

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
