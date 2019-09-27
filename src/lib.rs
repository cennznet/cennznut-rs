#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![warn(clippy::pedantic)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};

mod test;

/// A CENNZnet permission domain module method
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct Method {
    pub name: String,
    pub block_cooldown: Option<u32>,
}

pub trait Validate {
    fn validate(&self, module: &str, method: &str) -> Result<(), &'static str> {
        Ok(())
    }
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
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct Module {
    pub name: String,
    pub block_cooldown: Option<u32>,
    pub methods: Vec<(String, Method)>,
}

impl Module {
    /// Returns the method, if it exists in the Module
    pub fn get_method(&self, method: &str) -> Option<&Method> {
        for (name, m) in self.methods.iter() {
            if name == method {
                return Some(m);
            }
        }
        None
    }
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

        for (_, method) in self.methods.iter() {
            method.encode_to(buf);
        }
    }
}

/// A CENNZnet permission domain struct for embedding in doughnuts
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct CENNZnutV0 {
    pub modules: Vec<(String, Module)>,
}

impl CENNZnutV0 {
    /// Returns the module, if it exists in the CENNZnut
    pub fn get_module(&self, module: &str) -> Option<&Module> {
        for (name, m) in self.modules.iter() {
            if name == module {
                return Some(m);
            }
        }
        None
    }
}

impl Encode for CENNZnutV0 {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        buf.write(&[0, 0]);

        let module_count = ((self.modules.len() as u8) - 1).swap_bits();
        buf.push_byte(module_count);

        for (_, module) in self.modules.iter() {
            module.encode_to(buf);
        }
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

        let module_count = (input.read_byte()?.swap_bits()) + 1;
        let mut modules: Vec<(String, Module)> = Default::default();

        for _ in 0..module_count {
            let m: Module = Decode::decode(input)?;
            modules.push((m.name.to_owned(), m));
        }

        Ok(Self { modules })
    }
}

impl Validate for CENNZnutV0 {
    fn validate(&self, module_name: &str, method_name: &str) -> Result<(), &'static str> {
        let module = self
            .get_module(module_name)
            .ok_or("Doughnut does not grant permission for module")?;
        module
            .get_method(method_name)
            .map(|_| ())
            .ok_or("Doughnut does not grant permission for method")
    }
}

impl Decode for Module {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let block_cooldown_and_method_count: u8 = input.read_byte()?.swap_bits();
        let method_count = block_cooldown_and_method_count >> 1;

        let mut name_buf: [u8; 32] = Default::default();
        let _ = input.read(&mut name_buf);
        let name = core::str::from_utf8(&name_buf)
            .map_err(|_| codec::Error::from("module names should be utf8 encoded"))?
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

        let mut methods: Vec<(String, Method)> = Default::default();

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

impl Decode for Method {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let block_cooldown_and_reserved_byte = input.read_byte()?.swap_bits();

        let mut name_buf: [u8; 32] = Default::default();
        let _ = input.read(&mut name_buf);
        let name = core::str::from_utf8(&name_buf)
            .map_err(|_| codec::Error::from("method names should be utf8 encoded"))?
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

        Ok(Self {
            name,
            block_cooldown,
        })
    }
}
