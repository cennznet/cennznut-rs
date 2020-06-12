// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! CENNZnut - Integration Tests
//!

#![cfg(test)]

use super::contract::Contract;
use super::method::Method;
use super::module::Module;
use crate::cennznut::{
    v0::{MAX_CONTRACTS, MAX_METHODS, MAX_MODULES},
    ContractAddress, ContractDomain, MethodName, ModuleName, RuntimeDomain, WILDCARD,
};
use crate::{CENNZnut, CENNZnutV0, TryFrom, ValidationErr};

use codec::{Decode, Encode};
use pact::contract::{Contract as PactContract, DataTable};
use pact::interpreter::OpCode;
use pact::types::{Numeric, PactType, StringLike};
use std::vec::Vec;

const MODULE_CONTRACT_BYTES: [u8; 67] = [
    0, 0, 99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 99, 111, 110, 116, 114, 97, 99, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn module_for_contracts() -> Vec<(ModuleName, Module)> {
    let method = Method::new("contract");
    let methods = make_methods(&method);
    let module = Module::new("call").methods(methods);
    make_modules(&module)
}

fn make_methods(method: &Method) -> Vec<(MethodName, Method)> {
    let mut methods = Vec::<(MethodName, Method)>::default();
    methods.push((method.name.clone(), method.clone()));
    methods
}

fn make_modules(module: &Module) -> Vec<(ModuleName, Module)> {
    let mut modules = Vec::<(ModuleName, Module)>::default();
    modules.push((module.name.clone(), module.clone()));
    modules
}

fn make_contracts(contract: &Contract) -> Vec<(ContractAddress, Contract)> {
    let mut contracts = Vec::<(ContractAddress, Contract)>::default();
    contracts.push((contract.address, contract.clone()));
    contracts
}

#[test]
fn it_works_encode() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let contract = Contract::new(&[0x5a_u8; 32]);
    let contracts = make_contracts(&contract);

    let cennznut = CENNZnutV0 { modules, contracts };
    let encoded = cennznut.encode();

    let expected_version = vec![0, 0];
    let expected_modules = vec![
        0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let expected_contracts = [vec![0x01, 0x00], vec![0x5a_u8; 32]].concat();
    assert_eq!(
        encoded,
        [expected_version, expected_modules, expected_contracts].concat()
    );
    assert_eq!(encoded[2], 0x00); // 1 module encodes to LE 0 = 0b0000_0000
}

#[test]
fn it_works_encode_one_module() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    );
}

#[test]
fn it_works_encode_one_contract() {
    let modules = module_for_contracts();

    let contract = Contract::new(&[0x5a_u8; 32]);
    let contracts = make_contracts(&contract);

    let cennznut = CENNZnutV0 { modules, contracts };

    let expected_version = vec![0, 0];
    let expected_modules = MODULE_CONTRACT_BYTES.to_vec();
    let expected_contract_header = vec![0x01, 0x00];
    let expected_contract_address = vec![0x5a_u8; 32];
    let expected_contracts = [expected_contract_header, expected_contract_address].concat();

    assert_eq!(
        cennznut.encode(),
        [expected_version, expected_modules, expected_contracts].concat()
    );
}

#[test]
fn it_works_decode() {
    let encoded_version = vec![0, 0];
    let encoded_modules = vec![
        0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let encoded_contract_header = vec![0x01, 0x00];
    let encoded_contract_address = vec![0x5a; 32];
    let encoded_contracts: Vec<u8> = [encoded_contract_header, encoded_contract_address].concat();

    let encoded: Vec<u8> = [encoded_version, encoded_modules, encoded_contracts].concat();
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("it works");

    assert_eq!(c.encode(), encoded);
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(c0.modules.len(), 1);
    assert_eq!(c0.contracts.len(), 1);
}

#[test]
fn it_works_encode_with_module_cooldown() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 0, 109, 101, 116, 104, 111,
            100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );
}

#[test]
fn it_works_encode_with_contract_cooldown() {
    let modules = module_for_contracts();

    let contract = Contract::new(&[0x8b_u8; 32]).block_cooldown(0x4433_2211);
    let contracts = make_contracts(&contract);

    let cennznut = CENNZnutV0 { modules, contracts };

    let expected_version = vec![0, 0];
    let expected_modules = MODULE_CONTRACT_BYTES.to_vec();
    let expected_contract_header = vec![0x01, 0x01];
    let expected_contract_address = vec![0x8b_u8; 32];
    let expected_contract_cooldown = vec![0x11, 0x22, 0x33, 0x44];
    let expected_contracts = [
        expected_contract_header,
        expected_contract_address,
        expected_contract_cooldown,
    ]
    .concat();

    assert_eq!(
        cennznut.encode(),
        [expected_version, expected_modules, expected_contracts].concat()
    );
}

#[test]
fn it_works_decode_with_module_cooldown() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 0, 109, 101, 116, 104, 111, 100, 95,
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("It works");
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(
        c0.get_module("module_test")
            .expect("module exists")
            .block_cooldown,
        Some(86_400)
    );
}

#[test]
fn it_works_encode_with_method_cooldown() {
    let method = Method::new("method_test").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 1, 109, 101, 116, 104, 111,
            100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 123, 0, 0, 0, 0,
        ]
    );
}

#[test]
fn it_works_decode_with_method_cooldown() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 1, 109, 101, 116, 104, 111, 100, 95,
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0,
        0, 0, 0,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("It works");
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(
        c0.get_module("module_test")
            .expect("module exists")
            .block_cooldown,
        Some(86_400)
    );
    assert_eq!(
        c0.get_module("module_test")
            .expect("module exists")
            .get_method("method_test")
            .expect("method exists")
            .block_cooldown,
        Some(123)
    );
}

#[test]
fn it_works_encode_two_contracts() {
    let modules = module_for_contracts();

    let contract_a = Contract::new(&[0x4a_u8; 32]);
    let contract_b = Contract::new(&[0x8b_u8; 32]).block_cooldown(0xaa55_33cc);
    let mut contracts = make_contracts(&contract_a);
    contracts.push((contract_b.address, contract_b));

    let cennznut = CENNZnutV0 { modules, contracts };

    let expected_version = vec![0, 0];
    let expected_modules = MODULE_CONTRACT_BYTES.to_vec();
    let expected_contract_header = vec![0x02];
    // no cooldown
    let expected_contract_a_header = vec![0x00];
    let expected_contract_a_address = vec![0x4a_u8; 32];
    // has cooldown
    let expected_contract_b_header = vec![0x01];
    let expected_contract_b_address = vec![0x8b_u8; 32];
    let expected_contract_b_cooldown = vec![0xcc, 0x33, 0x55, 0xaa];
    let expected_contracts = [
        expected_contract_header,
        expected_contract_a_header,
        expected_contract_a_address,
        expected_contract_b_header,
        expected_contract_b_address,
        expected_contract_b_cooldown,
    ]
    .concat();

    assert_eq!(
        cennznut.encode(),
        [expected_version, expected_modules, expected_contracts].concat()
    );
}

#[test]
fn it_works_decode_one_contract() {
    let encoded_version = vec![0, 0];
    let encoded_modules = MODULE_CONTRACT_BYTES.to_vec();
    let encoded_contract_header = vec![0x01, 0x00];
    let encoded_contract_address = vec![0x5a; 32];
    let encoded_contracts: Vec<u8> = [encoded_contract_header, encoded_contract_address].concat();

    let encoded: Vec<u8> = [encoded_version, encoded_modules, encoded_contracts].concat();
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("it works");

    assert_eq!(c.encode(), encoded);
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(c0.modules.len(), 1);
    assert_eq!(c0.contracts.len(), 1);
}

#[test]
fn it_works_decode_two_contracts() {
    let encoded_version = vec![0, 0];
    let encoded_modules = MODULE_CONTRACT_BYTES.to_vec();
    let encoded_contract_header = vec![0x02];
    let encoded_contract_a_header = vec![0x00];
    let encoded_contract_a_address = vec![0x4a; 32];
    let encoded_contract_b_header = vec![0x00];
    let encoded_contract_b_address = vec![0x8b; 32];
    let encoded_contracts: Vec<u8> = [
        encoded_contract_header,
        encoded_contract_a_header,
        encoded_contract_a_address,
        encoded_contract_b_header,
        encoded_contract_b_address,
    ]
    .concat();

    let encoded: Vec<u8> = [encoded_version, encoded_modules, encoded_contracts].concat();
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("it works");

    assert_eq!(c.encode(), encoded);
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(c0.modules.len(), 1);
    assert_eq!(c0.contracts.len(), 2);
}

#[test]
fn it_works_decode_with_version_0() {
    let encoded: Vec<u8> = vec![1, 2, 3, 192];
    assert_eq!(
        CENNZnutV0::decode(&mut &encoded[..]),
        Err(codec::Error::from("expected version : 0"))
    );
}

#[test]
fn it_works_encode_with_constraints() {
    let pact = PactContract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(111)),
            PactType::Numeric(Numeric(333)),
            PactType::StringLike(StringLike(b"testing")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    pact.encode(&mut constraints);

    let method = Method::new("method_test").constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let encoded = cennznut.encode();

    assert_eq!(
        encoded,
        vec![
            0, 0, 0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0,
            192, 128, 16, 246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224,
            116, 101, 115, 116, 105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1, 0,
        ]
    );
    let constraints_length_byte_cursor: usize = 4 + 32 + 1 + 32;
    #[allow(clippy::cast_possible_truncation)]
    let len_byte = constraints.len() as u8;
    assert_eq!(encoded[constraints_length_byte_cursor], (len_byte - 1));
}

#[test]
fn it_works_decode_with_constraints() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 192, 128, 16,
        246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224, 116, 101, 115, 116,
        105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1, 0,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("it works");
    assert_eq!(c.encode(), encoded);

    let c0 = CENNZnutV0::try_from(c).unwrap();
    let method = &c0
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists");

    if let Some(constraints) = &method.constraints {
        let constraints_length_byte_cursor: usize = 4 + 32 + 1 + 32;
        #[allow(clippy::cast_possible_truncation)]
        let len_byte = constraints.len() as u8;
        assert_eq!(encoded[constraints_length_byte_cursor] + 1, len_byte,);
    };
}

#[test]
fn it_works_with_lots_of_things_codec() {
    let method = Method::new("method_test").block_cooldown(123);
    let method2 = Method::new("method_test2").block_cooldown(321);

    let mut methods: Vec<(MethodName, Method)> = Vec::default();
    methods.push((method.name.clone(), method));
    methods.push((method2.name.clone(), method2));

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods.clone());
    let module2 = Module::new("module_test2")
        .block_cooldown(55_555)
        .methods(methods);

    let mut modules: Vec<(ModuleName, Module)> = Vec::default();
    modules.push((module.name.clone(), module));
    modules.push((module2.name.clone(), module2));

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };

    let encoded = vec![
        0, 0, 1, 3, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 1, 109, 101, 116, 104, 111, 100, 95,
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0,
        0, 0, 1, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 1, 0, 0, 3, 109, 111, 100, 117, 108, 101, 95, 116,
        101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 217, 0,
        0, 1, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 1, 109, 101, 116, 104, 111, 100, 95, 116,
        101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 1, 0, 0,
        0,
    ];
    assert_eq!(cennznut.encode(), encoded);
    assert_eq!(cennznut, CENNZnutV0::decode(&mut &encoded[..]).unwrap());
}

#[test]
fn it_validates_modules() {
    let pact = PactContract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(123)),
            PactType::StringLike(StringLike(b"test")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    pact.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [
        PactType::Numeric(Numeric(123)),
        PactType::StringLike(StringLike(b"test")),
    ];

    assert_eq!(
        cennznut.validate_module(&module.name, &method.name, &args),
        Ok(())
    );
    assert_eq!(
        cennznut.validate_module("module_test2", &method.name, &args),
        Err(ValidationErr::NoPermission(RuntimeDomain::Module))
    );
    assert_eq!(
        cennznut.validate_module(&module.name, "method_test2", &args),
        Err(ValidationErr::NoPermission(RuntimeDomain::Method))
    );
}

#[test]
fn it_validates_contracts() {
    let modules = Vec::<(ModuleName, Module)>::default();

    let contract = Contract::new(&[0x12_u8; 32]);
    let contracts = make_contracts(&contract);

    let cennznut = CENNZnutV0 { modules, contracts };

    assert_eq!(cennznut.validate_contract(contract.address), Ok(()));
}

#[test]
fn it_invalidates_missing_contract() {
    let modules = Vec::<(ModuleName, Module)>::default();

    let contract = Contract::new(&[0x12_u8; 32]);
    let contracts = make_contracts(&contract);

    let cennznut = CENNZnutV0 { modules, contracts };

    assert_eq!(
        cennznut.validate_contract([0x34_u8; 32]),
        Err(ValidationErr::NoPermission(ContractDomain::Contract))
    );
}

#[test]
fn it_validates_wildcard_contract() {
    let modules = Vec::<(ModuleName, Module)>::default();

    let contract = Contract::wildcard();
    let contracts = make_contracts(&contract);

    let cennznut = CENNZnutV0 { modules, contracts };

    assert_eq!(cennznut.validate_contract(contract.address), Ok(()));
}

#[test]
fn it_validate_modules_error_with_bad_bytecode() {
    let pact = PactContract {
        data_table: DataTable::new(vec![PactType::StringLike(StringLike(b"test"))]),
        bytecode: [OpCode::GT.into(), 0, 0, 1, 0].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    pact.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [PactType::StringLike(StringLike(b"test"))];

    assert_eq!(
        cennznut.validate_module(&module.name, &method.name, &args),
        Err(ValidationErr::ConstraintsInterpretation)
    );
}

#[test]
fn it_validate_modules_error_with_false_constraints() {
    let pact = PactContract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(123)),
            PactType::StringLike(StringLike(b"a")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    pact.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [
        PactType::Numeric(Numeric(321)),
        PactType::StringLike(StringLike(b"b")),
    ];

    assert_eq!(
        cennznut.validate_module(&module.name, &method.name, &args),
        Err(ValidationErr::NoPermission(RuntimeDomain::MethodArguments))
    );
}

#[test]
fn it_validate_modules_with_empty_constraints() {
    let method = Method::new("method_test").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [
        PactType::Numeric(Numeric(0)),
        PactType::StringLike(StringLike(b"test")),
    ];

    assert_eq!(
        cennznut.validate_module(&module.name, &method.name, &args),
        Ok(())
    );
}

#[test]
fn it_works_get_pact() {
    // A CENNZnut with constraints set
    let encoded_with: Vec<u8> = vec![
        0, 0, 0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 192, 128, 16,
        246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224, 116, 101, 115, 116,
        105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1, 0,
    ];

    let cennznut_with: CENNZnut = Decode::decode(&mut &encoded_with[..]).expect("it works");
    let cennznut_with_v0 = CENNZnutV0::try_from(cennznut_with).unwrap();
    let pact_with = cennznut_with_v0
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists")
        .get_pact();

    if let Some(pact) = pact_with {
        assert_eq!(
            pact,
            PactContract {
                data_table: DataTable::new(vec![
                    PactType::Numeric(Numeric(111)),
                    PactType::Numeric(Numeric(333)),
                    PactType::StringLike(StringLike(b"testing")),
                ]),
                bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
            }
        );
    }

    // A CENNZnut without constraints set
    let encoded_without: Vec<u8> = vec![
        0, 0, 0, 0, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let cennznut_without: CENNZnut = Decode::decode(&mut &encoded_without[..]).expect("it works");
    let cennznut_without_v0 = CENNZnutV0::try_from(cennznut_without).unwrap();
    let contract_without = cennznut_without_v0
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists")
        .get_pact();

    assert_eq!(contract_without, None);
}

#[test]
fn wildcard_method() {
    let method = Method::new(WILDCARD).block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(1)
        .methods(methods);

    let result = module.get_method("my_unregistered_method");
    assert_eq!(result, Some(&method));
}

#[test]
fn wildcard_method_validate_modules() {
    let method = Method::new(WILDCARD).block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(1)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [];

    assert_eq!(
        cennznut.validate_module(&module.name, "my_unregistered_method", &args),
        Ok(())
    );
}

#[test]
fn wildcard_module() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new(WILDCARD).block_cooldown(1).methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };

    let result = cennznut.get_module("my_unregistered_module");
    assert_eq!(result, Some(&module));
}

#[test]
fn wildcard_module_validate_modules() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new(WILDCARD).block_cooldown(1).methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [];

    assert_eq!(
        cennznut.validate_module("my_unregistered_module", "registered_method", &args),
        Ok(())
    );
}

#[test]
fn wildcard_module_wildcard_method_validate_modules() {
    let method = Method::new(WILDCARD).block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new(WILDCARD).block_cooldown(1).methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [];

    assert_eq!(
        cennznut.validate_module("my_unregistered_module", "my_unregistered_method", &args),
        Ok(())
    );
}

#[test]
fn unregistered_module_fails_validation() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("registered_module")
        .block_cooldown(1)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [];

    assert_eq!(
        cennznut.validate_module("my_unregistered_module", "registered_method", &args),
        Err(ValidationErr::NoPermission(RuntimeDomain::Module))
    );
}

#[test]
fn unregistered_method_fails_validation() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("registered_module")
        .block_cooldown(1)
        .methods(methods);
    let modules = make_modules(&module);

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };
    let args = [];

    assert_eq!(
        cennznut.validate_module("registered_module", "my_unregistered_method", &args),
        Err(ValidationErr::NoPermission(RuntimeDomain::Method))
    );
}

#[test]
fn registered_methods_have_priority_over_wildcard_methods() {
    let wild_method = Method::new(WILDCARD).block_cooldown(123);
    let registered_method = Method::new("registered_method").block_cooldown(123);

    let mut methods: Vec<(MethodName, Method)> = Vec::default();
    methods.push((wild_method.name.clone(), wild_method));
    methods.push((registered_method.name.clone(), registered_method));

    let module = Module::new("module_test")
        .block_cooldown(1)
        .methods(methods);

    let result = module.get_method("registered_method").unwrap();

    assert_eq!(result.name, "registered_method");
}

#[test]
fn registered_modules_have_priority_over_wildcard_modules() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let wild_module = Module::new(WILDCARD)
        .block_cooldown(123)
        .methods(methods.clone());
    let registered_module = Module::new("registered_module")
        .block_cooldown(123)
        .methods(methods);

    let mut modules: Vec<(ModuleName, Module)> = Vec::default();
    modules.push((wild_module.name.clone(), wild_module));
    modules.push((registered_module.name.clone(), registered_module));

    let contracts = Vec::<(ContractAddress, Contract)>::default();

    let cennznut = CENNZnutV0 { modules, contracts };

    let result = cennznut.get_module("registered_module").unwrap();

    assert_eq!(result.name, "registered_module");
}

#[test]
fn it_fails_to_encode_with_zero_modules() {
    let modules: Vec<(ModuleName, Module)> = Vec::default();
    let contracts = Vec::<(ContractAddress, Contract)>::default();
    let cennznut = CENNZnutV0 { modules, contracts };
    assert_eq!(cennznut.encode(), []);
}

#[test]
fn it_fails_to_encode_with_zero_methods() {
    let methods: Vec<(MethodName, Method)> = Vec::default();
    let module = Module::new("TestModule").methods(methods);
    let modules = make_modules(&module);
    let contracts = Vec::<(ContractAddress, Contract)>::default();
    let cennznut = CENNZnutV0 { modules, contracts };
    assert_eq!(cennznut.encode(), []);
}

#[test]
fn it_fails_to_encode_with_too_many_modules() {
    let method = Method::new("registered_method");
    let methods = make_methods(&method);
    let mut modules: Vec<(ModuleName, Module)> = Vec::default();
    for x in 0..MAX_MODULES + 1 {
        let module = Module::new(&x.to_string()).methods(methods.clone());
        modules.push((module.name.clone(), module));
    }
    let contracts = Vec::<(ContractAddress, Contract)>::default();
    let cennznut = CENNZnutV0 { modules, contracts };
    assert_eq!(cennznut.encode(), []);
}

#[test]
fn it_fails_to_encode_with_too_many_methods() {
    let mut methods: Vec<(MethodName, Method)> = Vec::default();
    for x in 0..MAX_METHODS + 1 {
        let method = Method::new(&x.to_string());
        methods.push((method.name.clone(), method));
    }
    let module = Module::new("registered_module").methods(methods);
    let modules = make_modules(&module);
    let contracts = Vec::<(ContractAddress, Contract)>::default();
    let cennznut = CENNZnutV0 { modules, contracts };
    assert_eq!(cennznut.encode(), []);
}

#[test]
fn it_fails_to_encode_with_too_many_contracts() {
    let method = Method::new("registered_method");
    let methods = make_methods(&method);
    let module = Module::new("registered_module").methods(methods);
    let modules = make_modules(&module);
    let mut contracts = Vec::<(ContractAddress, Contract)>::default();
    for x in 0..MAX_CONTRACTS + 1 {
        let mut address = [0; 32];
        address[0] = x as u8 & 0xff;
        address[1] = (x >> 8) as u8 & 0xff;
        let contract = Contract::new(&address);
        contracts.push((contract.address, contract.clone()));
    }
    let cennznut = CENNZnutV0 { modules, contracts };
    assert_eq!(cennznut.encode(), []);
}

#[test]
fn it_fails_to_encode_when_cennznut_is_too_large() {
    // 33 bytes per method, 33 + 33 * Method bytes per module
    // if 64 methods, per 64 modules, total bytes > 137,000
    let mut methods: Vec<(MethodName, Method)> = Vec::default();
    let mut modules: Vec<(ModuleName, Module)> = Vec::default();
    for x in 0..64 + 1 {
        let method = Method::new(&x.to_string());
        methods.push((method.name.clone(), method));
    }
    for x in 0..64 + 1 {
        let module = Module::new(&x.to_string()).methods(methods.clone());
        modules.push((module.name.clone(), module));
    }
    let contracts = Vec::<(ContractAddress, Contract)>::default();
    let cennznut = CENNZnutV0 { modules, contracts };
    assert_eq!(cennznut.encode(), []);
}
