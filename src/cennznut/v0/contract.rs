// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Contracts
//!
//! Delegated smart contract permissioning of CENNZnut for use in CENNZnet
//!

#![warn(clippy::pedantic)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

// use alloc::borrow::ToOwned;
// use alloc::string::{String, ToString};
// use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Encode, Output};
// use codec::{Decode, Encode, Input, Output};

// use super::WILDCARD;

/// A CENNZnet permission domain contract
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct Contract {
    pub address: [u8; 32],
    pub block_cooldown: Option<u32>,
}

impl Contract {
    pub fn new(address: &[u8; 32]) -> Self {
        Self {
            address: address.clone(),
            block_cooldown: None,
        }
    }

    pub fn wildcard() -> Self {
        Self {
            address: [0x00; 32],
            block_cooldown: None,
        }
    }

    pub fn block_cooldown(mut self, block_cooldown: u32) -> Self {
        self.block_cooldown = Some(block_cooldown);
        self
    }
}

impl Encode for Contract {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        let mut has_cooldown_byte = 0x00_u8;
        if self.block_cooldown.is_some() {
            has_cooldown_byte |= 0b0000_0001_u8;
        }
        buf.push_byte(has_cooldown_byte.swap_bits());
        let address = self.address.clone();
        buf.write(&address);

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(b.swap_bits());
            }
        }
    }
}

// impl Decode for Contract {
//     fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
//         let block_cooldown_and_method_count: u8 = input.read_byte()?.swap_bits();
//         let method_count = block_cooldown_and_method_count >> 1;

//         let mut name_buf: [u8; 32] = Default::default();
//         input
//             .read(&mut name_buf)
//             .map_err(|_| "expected 32 byte module name")?;
//         let name = core::str::from_utf8(&name_buf)
//             .map_err(|_| codec::Error::from("module names should be utf8 encoded"))?
//             .trim_matches(char::from(0))
//             .to_string();

//         let module_cooldown =
//             if (block_cooldown_and_method_count.swap_bits() & 0b1000_0000) == 0b1000_0000 {
//                 Some(u32::from_le_bytes([
//                     input.read_byte()?.swap_bits(),
//                     input.read_byte()?.swap_bits(),
//                     input.read_byte()?.swap_bits(),
//                     input.read_byte()?.swap_bits(),
//                 ]))
//             } else {
//                 None
//             };

//         let mut methods: Vec<(String, Method)> = Vec::default();

//         for _ in 0..method_count {
//             let m: Method = Decode::decode(input)?;
//             methods.push((m.name.to_owned(), m));
//         }

//         Ok(Self {
//             name,
//             block_cooldown: module_cooldown,
//             methods,
//         })
//     }
// }

#[cfg(test)]
mod test {
    use super::Contract;
    use codec::{Encode, Output};
    use std::assert_eq;

    #[test]
    fn it_initializes() {
        let expected: usize = 32;
        let contract = Contract::new(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38,
        ]);

        assert_eq!(contract.address.len(), expected);
    }

    #[test]
    fn it_initializes_with_correct_address() {
        let expected_0: u8 = 0x01;
        let expected_9: u8 = 0x12;
        let expected_18: u8 = 0x23;
        let expected_27: u8 = 0x34;
        let contract = Contract::new(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38,
        ]);

        assert_eq!(contract.address[0], expected_0);
        assert_eq!(contract.address[9], expected_9);
        assert_eq!(contract.address[18], expected_18);
        assert_eq!(contract.address[27], expected_27);
    }

    #[test]
    fn it_initializes_wildcard() {
        let expected_length: usize = 32;
        let expected_value: u8 = 0x00;

        let contract = Contract::wildcard();

        assert_eq!(contract.address.len(), expected_length);
        for i in 0..32 {
            assert_eq!(contract.address[i], expected_value);
        }
    }

    #[test]
    fn it_initializes_with_no_cooldown() {
        let contract = Contract::wildcard();

        assert_eq!(contract.block_cooldown, Option::None);
    }

    #[test]
    fn it_can_set_a_cooldown() {
        let contract = Contract::wildcard().block_cooldown(0x1337_b33f);

        assert_eq!(contract.block_cooldown, Option::Some(0x1337_b33f));
    }

    #[test]
    fn it_encodes_basic_case() {
        let contract = Contract::new(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38,
        ]);

        let r: Vec<u8> = contract.encode();
        assert_eq!(
            r,
            vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15,
                0x16, 0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33,
                0x34, 0x35, 0x36, 0x37, 0x38,
            ]
        );
    }
}
