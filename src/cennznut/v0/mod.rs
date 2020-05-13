// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - V0
//!
//! Version 0 CENNZnut type.
//!

#[cfg(test)]
mod tests;

use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use core::convert::TryFrom;
use pact::interpreter::{interpret, types::PactType};

pub mod contract;
pub mod method;
pub mod module;

use super::{ContractDomain, RuntimeDomain};
use crate::{PartialDecode, ValidationErr};

use contract::Contract;
use module::Module;

use super::{ContractAddress, ModuleName, CONTRACT_WILDCARD, WILDCARD};

pub const MAX_MODULES: usize = 256;
pub const MAX_METHODS: usize = 128;
pub const MAX_CONTRACTS: usize = 255;
pub const VERSION_BYTES: [u8; 2] = [0, 0];
pub const MAX_CENNZNUT_BYTES: usize = u16::max_value() as usize;

/// A CENNZnet permission domain struct for embedding in doughnuts
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct CENNZnutV0 {
    pub modules: Vec<(ModuleName, Module)>,
    pub contracts: Vec<(ContractAddress, Contract)>,
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

    /// Returns the contract, if it exists in the CENNZnut
    /// Wildcard contracts (addr: 0) have lower priority than defined contracts
    pub fn get_contract(&self, contract: ContractAddress) -> Option<&Contract> {
        let mut outcome: Option<&Contract> = None;
        for (address, c) in &self.contracts {
            if address == &contract {
                outcome = Some(c);
                break;
            } else if address == &CONTRACT_WILDCARD {
                outcome = Some(c);
            }
        }
        outcome
    }
}

impl Encode for CENNZnutV0 {
    fn encode_to<T: Output>(&self, buf: &mut T) {
        if self.modules.is_empty() || self.modules.len() > MAX_MODULES {
            return;
        }
        let module_count = u8::try_from(self.modules.len() - 1);
        let contract_count = u8::try_from(self.contracts.len());
        if module_count.is_err() || contract_count.is_err() {
            return;
        }

        // Encode all modules, but make sure each encoding is valid
        // before modifying the output buffer.
        let mut module_payload_buf: Vec<u8> = Vec::<u8>::default();
        for (_, module) in &self.modules {
            let mut module_buf: Vec<u8> = Vec::<u8>::default();
            module.encode_to(&mut module_buf);
            if module_buf.is_empty() {
                return;
            }
            module_payload_buf.write(module_buf.as_slice());
        }

        let mut preliminary_buf = Vec::<u8>::default();

        preliminary_buf.write(&VERSION_BYTES);

        preliminary_buf.push_byte(module_count.unwrap().swap_bits());
        preliminary_buf.write(module_payload_buf.as_slice());

        preliminary_buf.push_byte(contract_count.unwrap().swap_bits());
        for (_, contract) in &self.contracts {
            contract.encode_to(&mut preliminary_buf);
        }

        // Avoid writing outside of the allocated domain buffer
        if preliminary_buf.len() <= MAX_CENNZNUT_BYTES {
            buf.write(preliminary_buf.as_slice());
        }
    }
}

impl PartialDecode for CENNZnutV0 {
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let module_count = input.read_byte()?.swap_bits() + 1;
        let mut modules = Vec::<(ModuleName, Module)>::default();

        for _ in 0..module_count {
            let m: Module = Decode::decode(input)?;
            modules.push((m.name.to_owned(), m));
        }

        let contract_count = input.read_byte()?.swap_bits();
        let mut contracts = Vec::<(ContractAddress, Contract)>::default();

        for _ in 0..contract_count {
            let c: Contract = Decode::decode(input)?;
            contracts.push((c.address, c));
        }

        Ok(Self { modules, contracts })
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

impl CENNZnutV0 {
    /// Validates a CENNZnut runtime module by:
    /// (1) looking for `module_name` and `method_name`
    /// (2) executing the Pact interpreter if constraints exist
    ///
    /// # Errors
    ///
    /// Will return error if validation fails with the type of error embedded in `RuntimeDomain`
    pub fn validate_module(
        &self,
        module_name: &str,
        method_name: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<RuntimeDomain>> {
        let module = self
            .get_module(module_name)
            .ok_or_else(|| ValidationErr::NoPermission(RuntimeDomain::Module))?;
        let method = module
            .get_method(method_name)
            .ok_or_else(|| ValidationErr::NoPermission(RuntimeDomain::Method))?;
        if let Some(pact) = method.get_pact() {
            match interpret(args, pact.data_table.as_ref(), &pact.bytecode) {
                Ok(true) => {}
                Ok(false) => {
                    return Err(ValidationErr::NoPermission(RuntimeDomain::MethodArguments))
                }
                Err(_) => return Err(ValidationErr::ConstraintsInterpretation),
            }
        }
        Ok(())
    }

    /// Validates a CENNZnut smart contract by
    /// (1) looking for `contract_address`
    ///
    /// # Errors
    ///
    /// Will return error if validation fails with the type of error embedded in `ContractDomain`
    pub fn validate_contract(
        &self,
        contract_address: ContractAddress,
    ) -> Result<(), ValidationErr<ContractDomain>> {
        self.get_contract(contract_address)
            .ok_or_else(|| ValidationErr::NoPermission(ContractDomain::Contract))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::CENNZnutV0;
    use super::Contract;
    use super::ContractAddress;
    use super::Module;
    use super::ModuleName;
    use super::CONTRACT_WILDCARD;

    #[test]
    fn it_gets_no_contract_from_empty_list() {
        let cennznut = CENNZnutV0 {
            modules: Vec::<(ModuleName, Module)>::default(),
            contracts: Vec::<(ContractAddress, Contract)>::default(),
        };

        assert_eq!(cennznut.get_contract([0x55; 32]), None);
    }

    #[test]
    fn it_gets_no_contract_from_list() {
        let mut contracts = Vec::<(ContractAddress, Contract)>::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_b = Contract::new(&[0x12_u8; 32]);
        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_b.address, contract_b));

        let cennznut = CENNZnutV0 {
            modules: Vec::<(ModuleName, Module)>::default(),
            contracts,
        };

        assert_eq!(cennznut.get_contract([0x55; 32]), None);
    }

    #[test]
    fn it_gets_a_contract() {
        let mut contracts = Vec::<(ContractAddress, Contract)>::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_b = Contract::new(&[0x12_u8; 32]);
        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_b.address, contract_b.clone()));

        let cennznut = CENNZnutV0 {
            modules: Vec::<(ModuleName, Module)>::default(),
            contracts,
        };

        assert_eq!(cennznut.get_contract([0x12_u8; 32]), Some(&contract_b));
    }

    #[test]
    fn it_gets_a_wildcard() {
        let mut contracts = Vec::<(ContractAddress, Contract)>::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_wildcard = Contract::new(&CONTRACT_WILDCARD);
        let contract_b = Contract::new(&[0x12_u8; 32]);

        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_wildcard.address, contract_wildcard.clone()));
        contracts.push((contract_b.address, contract_b));

        let cennznut = CENNZnutV0 {
            modules: Vec::<(ModuleName, Module)>::default(),
            contracts,
        };

        assert_eq!(
            cennznut.get_contract([0x55_u8; 32]),
            Some(&contract_wildcard)
        );
    }

    #[test]
    fn it_gives_defined_contracts_prescedence_over_wildcards() {
        let mut contracts = Vec::<(ContractAddress, Contract)>::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_wildcard = Contract::new(&CONTRACT_WILDCARD);
        let contract_b = Contract::new(&[0x12_u8; 32]);

        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_wildcard.address, contract_wildcard));
        contracts.push((contract_b.address, contract_b.clone()));

        let cennznut = CENNZnutV0 {
            modules: Vec::<(ModuleName, Module)>::default(),
            contracts,
        };

        assert_eq!(cennznut.get_contract([0x12_u8; 32]), Some(&contract_b));
    }
}
