// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Module
//!
//! Delegated runtime module permissioning of CENNZnut for use in CENNZnet
//!

use crate::cennznut::{MethodName, ModuleName};
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use core::convert::TryFrom;

use super::method::Method;
use super::MAX_METHODS;
use super::WILDCARD;

const BLOCK_COOLDOWN_MASK: u8 = 0b0000_0001;

/// A CENNZnet permission domain module
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct Module {
    pub name: ModuleName,
    pub block_cooldown: Option<u32>,
    pub methods: Vec<(MethodName, Method)>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            block_cooldown: None,
            methods: Vec::new(),
        }
    }

    pub fn block_cooldown(mut self, block_cooldown: u32) -> Self {
        self.block_cooldown = Some(block_cooldown);
        self
    }

    pub fn methods(mut self, methods: Vec<(MethodName, Method)>) -> Self {
        self.methods = methods;
        self
    }

    /// Returns the method, if it exists in the Module
    /// Wildcard methods have lower priority than defined methods
    pub fn get_method(&self, method: &str) -> Option<&Method> {
        let mut outcome: Option<&Method> = None;
        for (name, m) in &self.methods {
            if name == method {
                outcome = Some(m);
                break;
            } else if name == WILDCARD {
                outcome = Some(m);
            }
        }
        outcome
    }
}

impl Encode for Module {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        if self.methods.is_empty() || self.methods.len() > MAX_METHODS {
            return;
        }
        let method_count = u8::try_from(self.methods.len() - 1);
        if method_count.is_err() {
            return;
        }
        let mut method_count_and_has_cooldown_byte = method_count.unwrap() << 1;
        if self.block_cooldown.is_some() {
            method_count_and_has_cooldown_byte |= BLOCK_COOLDOWN_MASK;
        }
        buf.push_byte(method_count_and_has_cooldown_byte.swap_bits());
        let mut name = [0_u8; 32];
        name[0..self.name.len()].clone_from_slice(&self.name.as_bytes());

        buf.write(&name);

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(b.swap_bits());
            }
        }

        for (_, method) in &self.methods {
            method.encode_to(buf);
        }
    }
}

impl Decode for Module {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let block_cooldown_and_method_count: u8 = input.read_byte()?.swap_bits();
        let method_count = (block_cooldown_and_method_count >> 1) + 1;

        let mut name_buf: [u8; 32] = Default::default();
        input
            .read(&mut name_buf)
            .map_err(|_| "expected 32 byte module name")?;
        let name = core::str::from_utf8(&name_buf)
            .map_err(|_| codec::Error::from("module names should be utf8 encoded"))?
            .trim_matches(char::from(0))
            .to_string();

        let module_cooldown =
            if (block_cooldown_and_method_count & BLOCK_COOLDOWN_MASK) == BLOCK_COOLDOWN_MASK {
                Some(u32::from_le_bytes([
                    input.read_byte()?.swap_bits(),
                    input.read_byte()?.swap_bits(),
                    input.read_byte()?.swap_bits(),
                    input.read_byte()?.swap_bits(),
                ]))
            } else {
                None
            };

        let mut methods: Vec<(MethodName, Method)> = Vec::default();

        for _ in 0..method_count {
            let m: Method = Decode::decode(input)?;
            methods.push((m.name.to_owned(), m));
        }

        Ok(Self {
            name,
            block_cooldown: module_cooldown,
            methods,
        })
    }
}
