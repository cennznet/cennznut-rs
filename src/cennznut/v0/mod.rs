// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # CENNZnut - V0
//!
//! Version 0 CENNZnut type.
//!

#[cfg(test)]
mod tests;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode, Input, Output};
use pact::interpreter::{interpret, types::PactType};

pub mod contract;
pub mod method;
pub mod module;

use super::{ContractDomain, RuntimeDomain};
use crate::PartialDecode;
use crate::ValidationErr;

use contract::Contract;
use module::Module;

use contract::ContractAddress;
use contract::CONTRACT_WILDCARD;

pub const WILDCARD: &str = "*";

/// A CENNZnet permission domain struct for embedding in doughnuts
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub struct CENNZnutV0 {
    pub modules: Vec<(String, Module)>,
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
        buf.write(&[0, 0]);

        #[allow(clippy::cast_possible_truncation)]
        let module_count = (self.modules.len() as u8).swap_bits();
        buf.push_byte(module_count);

        for (_, module) in &self.modules {
            module.encode_to(buf);
        }

        #[allow(clippy::cast_possible_truncation)]
        let contract_count = (self.contracts.len() as u8).swap_bits();
        buf.push_byte(contract_count);

        for (_, contract) in &self.contracts {
            contract.encode_to(buf);
        }
    }
}

impl PartialDecode for CENNZnutV0 {
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let module_count = input.read_byte()?.swap_bits();
        let mut modules = Vec::<(String, Module)>::default();

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
        if let Some(contract) = method.get_pact() {
            match interpret(args, contract.data_table.as_ref(), &contract.bytecode) {
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
    use super::CONTRACT_WILDCARD;

    #[test]
    fn it_gets_no_contract_from_empty_list() {
        let cennznut = CENNZnutV0 {
            modules: Vec::default(),
            contracts: Vec::default(),
        };

        assert_eq!(cennznut.get_contract([0x55; 32]), None);
    }

    #[test]
    fn it_gets_no_contract_from_list() {
        let mut contracts: Vec<(ContractAddress, Contract)> = Vec::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_b = Contract::new(&[0x12_u8; 32]);
        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_b.address, contract_b));

        let cennznut = CENNZnutV0 {
            modules: Vec::default(),
            contracts,
        };

        assert_eq!(cennznut.get_contract([0x55; 32]), None);
    }

    #[test]
    fn it_gets_a_contract() {
        let mut contracts: Vec<(ContractAddress, Contract)> = Vec::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_b = Contract::new(&[0x12_u8; 32]);
        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_b.address, contract_b.clone()));

        let cennznut = CENNZnutV0 {
            modules: Vec::default(),
            contracts,
        };

        assert_eq!(cennznut.get_contract([0x12_u8; 32]), Some(&contract_b));
    }

    #[test]
    fn it_gets_a_wildcard() {
        let mut contracts: Vec<(ContractAddress, Contract)> = Vec::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_wildcard = Contract::new(&CONTRACT_WILDCARD);
        let contract_b = Contract::new(&[0x12_u8; 32]);

        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_wildcard.address, contract_wildcard.clone()));
        contracts.push((contract_b.address, contract_b));

        let cennznut = CENNZnutV0 {
            modules: Vec::default(),
            contracts,
        };

        assert_eq!(
            cennznut.get_contract([0x55_u8; 32]),
            Some(&contract_wildcard)
        );
    }

    #[test]
    fn it_gives_defined_contracts_prescedence_over_wildcards() {
        let mut contracts: Vec<(ContractAddress, Contract)> = Vec::default();
        let contract_a = Contract::new(&[0x45_u8; 32]);
        let contract_wildcard = Contract::new(&CONTRACT_WILDCARD);
        let contract_b = Contract::new(&[0x12_u8; 32]);

        contracts.push((contract_a.address, contract_a));
        contracts.push((contract_wildcard.address, contract_wildcard));
        contracts.push((contract_b.address, contract_b.clone()));

        let cennznut = CENNZnutV0 {
            modules: Vec::default(),
            contracts,
        };

        assert_eq!(cennznut.get_contract([0x12_u8; 32]), Some(&contract_b));
    }
}
