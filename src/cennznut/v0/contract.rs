// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Contracts
//!
//! Delegated smart contract permissioning of CENNZnut for use in CENNZnet
//!

use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};

pub type ContractAddress = [u8; 32];
pub const CONTRACT_WILDCARD: ContractAddress = [0u8; 32];

/// A CENNZnet permission domain contract
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct Contract {
    pub address: ContractAddress,
    pub block_cooldown: Option<u32>,
}

impl Contract {
    pub fn new(address: &ContractAddress) -> Self {
        Self {
            address: *address,
            block_cooldown: None,
        }
    }

    pub fn wildcard() -> Self {
        Self {
            address: CONTRACT_WILDCARD,
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
        let address = self.address;
        buf.write(&address);

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(b.swap_bits());
            }
        }
    }
}

impl Decode for Contract {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let has_cooldown_byte: u8 = input.read_byte()?.swap_bits();
        let has_cooldown: bool = (has_cooldown_byte & 0x01) == 0x01;

        let mut address: ContractAddress = Default::default();
        input
            .read(&mut address)
            .map_err(|_| "expected 32 byte address")?;

        let block_cooldown = if has_cooldown {
            Some(u32::from_le_bytes([
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
                input.read_byte()?.swap_bits(),
            ]))
        } else {
            None
        };

        Ok(Self {
            address,
            block_cooldown,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Contract;
    use codec::{Decode, Encode};
    use std::assert_eq;

    // Constructor tests
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

    // Encoding Tests
    #[test]
    fn it_encodes_basic_case() {
        let contract = Contract::new(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38,
        ]);

        let result: Vec<u8> = contract.encode();
        assert_eq!(
            result,
            vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15,
                0x16, 0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33,
                0x34, 0x35, 0x36, 0x37, 0x38,
            ]
        );
    }

    #[test]
    fn it_encodes_wildcard_case() {
        let has_cooldown_byte = 0x00;
        let address_length: usize = 32;
        let address_value: u8 = 0x00;
        let expected_length: usize = address_length + 1;

        let contract = Contract::wildcard();

        let result: Vec<u8> = contract.encode();
        assert_eq!(result.len(), expected_length);
        assert_eq!(result[0], has_cooldown_byte);
        for i in 0..address_length {
            assert_eq!(result[i + 1], address_value);
        }
    }

    #[test]
    fn it_encodes_block_cooldown() {
        // 0b1000_0000 = cooldown flag (note: bits get flipped)
        let has_cooldown_byte = 0x80;
        let address_length: usize = 32;
        let cooldown_length: usize = 4;
        let address_value: u8 = 0x00;
        let expected_length: usize = address_length + cooldown_length + 1;

        let contract = Contract::wildcard().block_cooldown(0x1337_b33f);

        let result: Vec<u8> = contract.encode();
        assert_eq!(result.len(), expected_length);
        assert_eq!(result[0], has_cooldown_byte);
        for i in 0..address_length {
            assert_eq!(result[i + 1], address_value);
        }
        // 1337_b33f flipped becomes fccd_ecc8 (due to LE encoding)
        assert_eq!(result[address_length + 1], 0xfc);
        assert_eq!(result[address_length + 2], 0xcd);
        assert_eq!(result[address_length + 3], 0xec);
        assert_eq!(result[address_length + 4], 0xc8);
    }

    // Decoding tests
    #[test]
    fn it_decodes_basic_case() {
        let contract_expected = Contract::new(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38,
        ]);

        let encoded = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33,
            0x34, 0x35, 0x36, 0x37, 0x38,
        ];

        let constract_result = Contract::decode(&mut &encoded[..]).expect("it works");

        assert_eq!(constract_result.encode(), encoded);
        assert_eq!(constract_result, contract_expected);
    }

    #[test]
    fn it_decodes_block_cooldown() {
        let contract_expected = Contract::wildcard().block_cooldown(0x1337_b33f);

        // 0b1000_0000 = cooldown flag (note: bits get flipped)
        let encoded = vec![
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xcd, 0xec, 0xc8,
        ];

        let constract_result = Contract::decode(&mut &encoded[..]).expect("it works");

        assert_eq!(constract_result.encode(), encoded);
        assert_eq!(constract_result, contract_expected);
    }

    #[test]
    fn it_throws_error_on_short_address_length() {
        let encoded = vec![
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(
            Contract::decode(&mut &encoded[..]),
            Err(codec::Error::from("expected 32 byte address")),
        );
    }

    #[test]
    fn it_ignores_unannounced_bytes() {
        let encoded = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xcd, 0xec, 0xc8,
        ];

        Contract::decode(&mut &encoded[..]).expect("it works");
    }

    #[test]
    fn it_throws_error_on_missing_cooldown() {
        let encoded = vec![
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(
            Contract::decode(&mut &encoded[..]),
            Err(codec::Error::from("Not enough data to fill buffer")),
        );
    }
}
