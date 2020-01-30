// Copyright (C) 2019-2020 Centrality Investments Limited
// This file is part of CENNZnet.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
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

mod method;
mod module;
mod v0;
mod validation;

pub use method::Method;
pub use module::Module;
pub use v0::CENNZnut as CENNZnutV0;
pub use validation::{Validate, ValidationErr};

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
