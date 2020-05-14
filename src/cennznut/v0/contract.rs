// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - Contracts
//!
//! Delegated smart contract permissioning of CENNZnut for use in CENNZnet
//!

use crate::cennznut::{ContractAddress, CONTRACT_WILDCARD};
use codec::{Decode, Encode, Input, Output};

const BLOCK_COOLDOWN_MASK: u8 = 0x01;

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
        let has_cooldown_byte: u8 = if self.block_cooldown.is_some() {
            BLOCK_COOLDOWN_MASK
        } else {
            0x00_u8
        };
        buf.push_byte(has_cooldown_byte);
        let address: ContractAddress = self.address;

        for byte in &address {
            buf.push_byte(*byte);
        }

        if let Some(cooldown) = self.block_cooldown {
            for b in &cooldown.to_le_bytes() {
                buf.push_byte(*b);
            }
        }
    }
}

impl Decode for Contract {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let has_cooldown_byte: u8 = input.read_byte()?;
        let has_cooldown: bool = (has_cooldown_byte & BLOCK_COOLDOWN_MASK) == BLOCK_COOLDOWN_MASK;

        let mut address = ContractAddress::default();
        input
            .read(&mut address)
            .map_err(|_| "expected 32 byte address")?;

        let block_cooldown = if has_cooldown {
            Some(u32::from_le_bytes([
                input.read_byte()?,
                input.read_byte()?,
                input.read_byte()?,
                input.read_byte()?,
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
    use super::{Contract, ContractAddress, BLOCK_COOLDOWN_MASK, CONTRACT_WILDCARD};
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

        let contract = Contract::wildcard();

        assert_eq!(contract.address.len(), expected_length);
        assert_eq!(contract.address, CONTRACT_WILDCARD);
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
        let contract_address: ContractAddress = [
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02,
        ];
        let contract = Contract::new(&contract_address);

        let result: Vec<u8> = contract.encode();

        assert_eq!(
            result,
            [vec![0x00], vec![0x01; 16], vec![0x02; 16]].concat()
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
        let address_length: usize = 32;
        let cooldown_length: usize = 4;
        let address_value: u8 = 0x00;
        let expected_length: usize = address_length + cooldown_length + 1;

        let contract = Contract::wildcard().block_cooldown(0x1337_b33f);
        let result: Vec<u8> = contract.encode();

        assert_eq!(result.len(), expected_length);
        assert_eq!(result[0], BLOCK_COOLDOWN_MASK);
        for i in 0..address_length {
            assert_eq!(result[i + 1], address_value);
        }
        // LE 1337_b33f
        assert_eq!(result[address_length + 1], 0x3f);
        assert_eq!(result[address_length + 2], 0xb3);
        assert_eq!(result[address_length + 3], 0x37);
        assert_eq!(result[address_length + 4], 0x13);
    }

    // Decoding tests
    #[test]
    fn it_decodes_basic_case() {
        let contract_address: ContractAddress = [
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02,
        ];

        let contract_expected = Contract::new(&contract_address);

        let encoded = [vec![0x00], vec![0x01; 16], vec![0x02; 16]].concat();

        let contract_result = Contract::decode(&mut &encoded[..]).expect("it works");

        assert_eq!(contract_result.encode(), encoded);
        assert_eq!(contract_result, contract_expected);
    }

    #[test]
    fn it_decodes_block_cooldown() {
        let contract_expected = Contract::wildcard().block_cooldown(0x1337_b33f);

        let encoded = vec![
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xb3, 0x37, 0x13,
        ];

        let constract_result = Contract::decode(&mut &encoded[..]).expect("it works");

        assert_eq!(constract_result.encode(), encoded);
        assert_eq!(constract_result, contract_expected);
    }

    #[test]
    fn it_throws_error_on_short_address_length() {
        let encoded = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
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
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(
            Contract::decode(&mut &encoded[..]),
            Err(codec::Error::from("Not enough data to fill buffer")),
        );
    }
}
