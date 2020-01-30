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

//! # CENNZnut
//!
//! Collection of versioned `CENNZnuts`
//!

#![warn(clippy::pedantic)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use pact::interpreter::types::PactType;

use crate::PartialDecode;
use crate::Validate;
use crate::ValidationErr;

pub mod v0;

use core::convert::TryFrom;
use v0::CENNZnutV0;
use CENNZnut::V0;

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

impl Validate for CENNZnut {
    fn validate(
        &self,
        module_name: &str,
        method_name: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr> {
        match &self {
            V0(inner) => inner.validate(module_name, method_name, args),
        }
    }
}
