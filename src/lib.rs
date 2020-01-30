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

// #[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
// pub enum CENNZnut {
//     V0(CENNZnutV0),
// }

// #[allow(unreachable_patterns)]
// impl TryFrom<CENNZnut> for CENNZnutV0 {
//     type Error = codec::Error;
//     fn try_from(v: CENNZnut) -> Result<Self, Self::Error> {
//         match v {
//             V0(inner) => Ok(inner),
//             _ => Err(codec::Error::from("CENNZnut version is not 0")),
//         }
//     }
// }

// impl Encode for CENNZnut {
//     fn encode_to<T: Output>(&self, buf: &mut T) {
//         match &self {
//             V0(inner) => inner.encode_to(buf),
//         }
//     }
// }

// impl Decode for CENNZnut {
//     fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
//         let version = u16::from_le_bytes([
//             input.read_byte()?.swap_bits(),
//             input.read_byte()?.swap_bits(),
//         ]);

//         match version {
//             0 => match CENNZnutV0::partial_decode(input) {
//                 Ok(inner) => Ok(V0(inner)),
//                 Err(e) => Err(e),
//             },
//             _ => Err(codec::Error::from("unexpected version")),
//         }
//     }
// }

// impl Validate for CENNZnut {
//     fn validate(
//         &self,
//         module_name: &str,
//         method_name: &str,
//         args: &[PactType],
//     ) -> Result<(), ValidationErr> {
//         match &self {
//             V0(inner) => inner.validate(module_name, method_name, args),
//         }
//     }
// }
