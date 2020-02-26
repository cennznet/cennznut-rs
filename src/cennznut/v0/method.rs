// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Method
//!
//! Delegated method permissioning of CENNZnut for use in CENNZnet
//!

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use pact::contract::Contract;

/// A CENNZnet permission domain module method
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct Method {
    pub name: String,
    pub block_cooldown: Option<u32>,
    pub constraints: Option<Vec<u8>>,
}

impl Method {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            block_cooldown: None,
            constraints: None,
        }
    }

    pub fn block_cooldown(mut self, block_cooldown: u32) -> Self {
        self.block_cooldown = Some(block_cooldown);
        self
    }

    pub fn constraints(mut self, constraints: Vec<u8>) -> Self {
        self.constraints = Some(constraints);
        self
    }

    /// Returns the Pact contract, if it exists in the Method
    pub fn get_pact(&self) -> Option<Contract> {
        match &self.constraints {
            Some(constraints) => match Contract::decode(constraints) {
                Ok(contract) => Some(contract),
                // This error case can only occur after initializing a Method with bad constraints.
                // A decoded Method will be checked during decoding.
                Err(_) => None,
            },
            None => None,
        }
    }
}

impl Encode for Method {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        let has_cooldown_byte: u8 = if self.block_cooldown.is_some() {
            0b1000_0000
        } else {
            0
        };
        let has_constraints_byte: u8 = if self.constraints.is_some() {
            0b0100_0000
        } else {
            0
        };
        buf.push_byte(has_cooldown_byte | has_constraints_byte);

        let mut name = [0_u8; 32];

        name[0..self.name.len()].clone_from_slice(&self.name.as_bytes());

        buf.write(&name);

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(b.swap_bits());
            }
        }

        if let Some(constraints) = &self.constraints {
            #[allow(clippy::cast_possible_truncation)]
            buf.push_byte(((constraints.len() as u8).saturating_sub(1)).swap_bits());
            buf.write(&constraints);
        }
    }
}

impl Decode for Method {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let block_cooldown_and_constraints = input.read_byte()?.swap_bits();

        let mut name_buf: [u8; 32] = Default::default();
        input
            .read(&mut name_buf)
            .map_err(|_| "expected 32 byte method name")?;
        let name = core::str::from_utf8(&name_buf)
            .map_err(|_| codec::Error::from("method names should be utf8 encoded"))?
            .trim_matches(char::from(0))
            .to_string();

        let block_cooldown: Option<u32> =
            if (block_cooldown_and_constraints.swap_bits() & 0b1000_0000) == 0b1000_0000 {
                Some(u32::from_le_bytes([
                    input.read_byte()?.swap_bits(),
                    input.read_byte()?.swap_bits(),
                    input.read_byte()?.swap_bits(),
                    input.read_byte()?.swap_bits(),
                ]))
            } else {
                None
            };

        let constraints: Option<Vec<u8>> =
            if (block_cooldown_and_constraints.swap_bits() & 0b0100_0000) == 0b0100_0000 {
                let constraints_length = (input.read_byte()?.swap_bits()).saturating_add(1);
                let mut constraints_buf = Vec::<u8>::default();
                for _ in 0..constraints_length {
                    constraints_buf.push(input.read_byte()?);
                }
                if Contract::decode(&constraints_buf).is_err() {
                    return Err(codec::Error::from("invalid constraints codec"));
                };
                Some(constraints_buf)
            } else {
                None
            };

        Ok(Self {
            name,
            block_cooldown,
            constraints,
        })
    }
}
