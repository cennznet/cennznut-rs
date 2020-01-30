// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut
//!
//! Delegated authority nut for CENNZnet
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use alloc::fmt::{self, Display, Formatter};
use codec::Input;
pub use core::convert::TryFrom;

mod cennznut;
mod method;
mod module;
mod validation;

pub use crate::cennznut::v0::CENNZnutV0;
pub use crate::cennznut::CENNZnut;
pub use crate::method::Method;
pub use crate::module::Module;
pub use crate::validation::{Validate, ValidationErr};

pub const WILDCARD: &str = "*";

mod test;

/// A CENNZnet permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum Domain {
    Method,
    MethodArguments,
    Module,
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Method => write!(f, "method"),
            Self::MethodArguments => write!(f, "method arguments"),
            Self::Module => write!(f, "module"),
        }
    }
}

pub trait PartialDecode: Sized {
    /// decode an input which is not including the version as the up front two bytes
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error>;
}
