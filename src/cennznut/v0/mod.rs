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
//! # CENNZnut - V0
//!
//! Version 0 CENNZnut type.
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use pact::interpreter::{interpret, types::PactType};

use crate::Domain;
use crate::Module;
use crate::PartialDecode;
use crate::Validate;
use crate::ValidationErr;
use crate::WILDCARD;

/// A CENNZnet permission domain struct for embedding in doughnuts
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct CENNZnutV0 {
    pub modules: Vec<(String, Module)>,
}

impl CENNZnutV0 {
    /// Returns the module, if it exists in the CENNZnut
    /// Wildcard modules have lower priority than defined modules
    pub fn get_module(&self, module: &str) -> Option<&Module> {
        let mut outcome: Option<&Module> = None;
        for (name, m) in &self.modules {
            if name == module {
                outcome = Some(m);
                break;
            } else if name == WILDCARD {
                outcome = Some(m);
            }
        }
        outcome
    }
}

impl Encode for CENNZnutV0 {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        buf.write(&[0, 0]);

        #[allow(clippy::cast_possible_truncation)]
        let module_count = ((self.modules.len() as u8).saturating_sub(1)).swap_bits();
        buf.push_byte(module_count);

        for (_, module) in &self.modules {
            module.encode_to(buf);
        }
    }
}

impl PartialDecode for CENNZnutV0 {
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let module_count = (input.read_byte()?.swap_bits()).saturating_add(1);
        let mut modules = Vec::<(String, Module)>::default();

        for _ in 0..module_count {
            let m: Module = Decode::decode(input)?;
            modules.push((m.name.to_owned(), m));
        }

        Ok(Self { modules })
    }
}

impl Decode for CENNZnutV0 {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let version = u16::from_le_bytes([
            input.read_byte()?.swap_bits(),
            input.read_byte()?.swap_bits(),
        ]);
        if version != 0 {
            return Err(codec::Error::from("expected version : 0"));
        }
        Self::partial_decode(input)
    }
}

impl Validate for CENNZnutV0 {
    /// Validates a CENNZnut by (1) looking for `module_name` and `method_name` and (2) executing the
    /// Pact interpreter if constraints exist
    fn validate(
        &self,
        module_name: &str,
        method_name: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr> {
        let module = self
            .get_module(module_name)
            .ok_or_else(|| ValidationErr::NoPermission(Domain::Module))?;
        let method = module
            .get_method(method_name)
            .ok_or_else(|| ValidationErr::NoPermission(Domain::Method))?;
        if let Some(contract) = method.get_pact() {
            match interpret(args, contract.data_table.as_ref(), &contract.bytecode) {
                Ok(true) => {}
                Ok(false) => return Err(ValidationErr::NoPermission(Domain::MethodArguments)),
                Err(_) => return Err(ValidationErr::ConstraintsInterpretation),
            }
        }
        Ok(())
    }
}
