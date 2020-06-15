// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut
//!
//! Delegated authority nut for CENNZnet
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use codec::Input;
pub use core::convert::TryFrom;

mod cennznut;
mod validation;

pub use crate::cennznut::ContractDomain;
pub use crate::cennznut::RuntimeDomain;

pub use crate::cennznut::v0;

pub use crate::cennznut::v0::CENNZnutV0;
pub use crate::cennznut::CENNZnut;
pub use crate::validation::ValidationErr;

mod test;

pub trait PartialDecode: Sized {
    /// decode an input which is not including the version as the up front two bytes
    ///
    /// # Errors
    ///
    /// On failure, returns a `codec::Error`
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error>;
}
