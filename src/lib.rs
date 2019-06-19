#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic)]
#![feature(alloc)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use hashbrown::HashMap;
use parity_codec::{Decode, Encode, Input, Output};

mod test;

/// A CENNZnet permission domain module method
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Method {
    pub name: String,
    pub block_cooldown: Option<u32>,
}

impl Encode for Method {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        let has_cooldown_byte: u8 = if self.block_cooldown.is_some() {
            0b1000_0000
        } else {
            0
        };
        buf.push_byte(has_cooldown_byte);

        let mut name = [0u8; 32];
        for i in 0..self.name.len() {
            name[i] = self.name.as_bytes()[i];
        }
        buf.write(&name);

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(b.swap_bits());
            }
        }
    }
}

/// A CENNZnet permission domain module
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub name: String,
    pub block_cooldown: Option<u32>,
    pub methods: HashMap<String, Method>,
    /// Internal fields preserves module order for encoding
    method_order: Option<Vec<String>>,
}

impl Encode for Module {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        let mut method_count_and_has_cooldown_byte = (self.methods.len() as u8) << 1;
        if self.block_cooldown.is_some() {
            method_count_and_has_cooldown_byte |= 0b0000_0001;
        }
        buf.push_byte(method_count_and_has_cooldown_byte.swap_bits());
        let mut name = [0u8; 32];
        for i in 0..self.name.len() {
            name[i] = self.name.as_bytes()[i];
        }
        buf.write(&name);

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(b.swap_bits());
            }
        }

        if let Some(m) = &self.method_order {
            for name in m.iter() {
                self.methods[name].encode_to(buf);
            }
        } else {
            for (_, method) in self.methods.iter() {
                method.encode_to(buf);
            }
        }
    }
}

/// A CENNZnet permission domain struct for embedding in doughnuts
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CENNZnutV0 {
    pub modules: HashMap<String, Module>,
    /// Internal fields preserves module order for encoding
    module_order: Option<Vec<String>>,
}

impl Encode for CENNZnutV0 {
    fn encode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Default::default();
        buf.extend([0, 0].into_iter());

        let module_count = ((self.modules.len() as u8) - 1).swap_bits();
        buf.push(module_count);

        if let Some(m) = &self.module_order {
            for name in m.iter() {
                self.modules[name].encode_to(&mut buf);
            }
        } else {
            for (_, module) in self.modules.iter() {
                module.encode_to(&mut buf);
            }
        }

        buf
    }
}

impl Decode for CENNZnutV0 {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let version = u16::from_le_bytes([
            input.read_byte()?.swap_bits(),
            input.read_byte()?.swap_bits(),
        ]);
        if version != 0 {
            return None;
        }

        let module_count = (input.read_byte()?.swap_bits()) + 1;
        let mut module_order: Vec<String> = Default::default();
        let mut modules: HashMap<String, Module> = Default::default();

        for _ in 0..module_count {
            let m: Module = Decode::decode(input)?;
            modules.insert(m.name.clone(), m.clone());
            module_order.push(m.name);
        }

        Some(Self {
            modules,
            module_order: Some(module_order),
        })
    }
}

impl Decode for Module {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let block_cooldown_and_method_count: u8 = input.read_byte()?.swap_bits();
        let method_count = block_cooldown_and_method_count >> 1;

        let mut name_buf: [u8; 32] = Default::default();
        let _ = input.read(&mut name_buf);
        let name = core::str::from_utf8(&name_buf)
            .ok()?
            .trim_matches(char::from(0))
            .to_string();

        let mut module_cooldown: Option<u32> = None;
        if (block_cooldown_and_method_count.swap_bits() & 0b1000_0000) == 0b1000_0000 {
            module_cooldown = Some(u32::from_le_bytes([
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
            ]));
        }

        let mut method_order: Vec<String> = Default::default();
        let mut methods: HashMap<String, Method> = Default::default();

        for _ in 0..method_count {
            let m: Method = Decode::decode(input)?;
            methods.insert(m.name.clone(), m.clone());
            method_order.push(m.name);
        }

        Some(Self {
            name,
            block_cooldown: module_cooldown,
            methods,
            method_order: Some(method_order),
        })
    }
}

impl Decode for Method {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let block_cooldown_and_reserved_byte = input.read_byte()?.swap_bits();

        let mut name_buf: [u8; 32] = Default::default();
        let _ = input.read(&mut name_buf);
        let name = core::str::from_utf8(&name_buf)
            .ok()?
            .trim_matches(char::from(0))
            .to_string();

        let mut block_cooldown: Option<u32> = None;
        if (block_cooldown_and_reserved_byte.swap_bits() & 0b1000_0000) == 0b1000_0000 {
            block_cooldown = Some(u32::from_le_bytes([
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
            ]));
        }

        Some(Self {
            name,
            block_cooldown,
        })
    }
}
