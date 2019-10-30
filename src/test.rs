#![cfg(test)]

use crate::{CENNZnutV0, Method, Module, Validate};
use bit_reverse::ParallelReverse;
use codec::{Decode, Encode};
use pact::compiler::{Contract, DataTable};
use pact::interpreter::OpCode;
use pact::types::{Numeric, PactType, StringLike};
use std::string::String;
use std::vec::Vec;

fn make_methods(method: &Method) -> Vec<(String, Method)> {
    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));
    methods
}

fn make_modules(module: &Module) -> Vec<(String, Module)> {
    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));
    modules
}

#[test]
fn it_works_encode() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let encoded = cennznut.encode();

    assert_eq!(
        encoded,
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
    assert_eq!(encoded[2], 0); // 1 module encodes to 0
}

#[test]
fn it_works_encode_one_module() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let c: CENNZnutV0 = Decode::decode(&mut &encoded[..]).expect("it works");
    assert_eq!(c.encode(), encoded);
    assert_eq!(c.modules.len(), 1);
}

#[test]
fn it_works_encode_with_module_cooldown() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 0, 109, 101, 116, 104,
            111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode_with_module_cooldown() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 0, 109, 101, 116, 104, 111, 100, 95,
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let c: CENNZnutV0 = Decode::decode(&mut &encoded[..]).expect("It works");
    assert_eq!(
        c.get_module("module_test")
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

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104,
            111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 222, 0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode_with_method_cooldown() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111, 100,
        95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222,
        0, 0, 0,
    ];
    let c: CENNZnutV0 = Decode::decode(&mut &encoded[..]).expect("It works");
    assert_eq!(
        c.get_module("module_test")
            .expect("module exists")
            .block_cooldown,
        Some(86_400)
    );
    assert_eq!(
        c.get_module("module_test")
            .expect("module exists")
            .get_method("method_test")
            .expect("method exists")
            .block_cooldown,
        Some(123)
    );
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
    let contract = Contract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(111)),
            PactType::Numeric(Numeric(333)),
            PactType::StringLike(StringLike(b"testing")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test").constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let encoded = cennznut.encode();

    assert_eq!(
        encoded,
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
            192, 128, 16, 246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224,
            116, 101, 115, 116, 105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1,
        ]
    );
    let constraints_length_byte_cursor: usize = 4 + 32 + 1 + 32;
    assert_eq!(
        encoded[constraints_length_byte_cursor],
        (constraints.len() as u8 - 1).swap_bits()
    );
}

#[test]
fn it_works_decode_with_constraints() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 192, 128, 16,
        246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224, 116, 101, 115, 116,
        105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1,
    ];
    let c: CENNZnutV0 = Decode::decode(&mut &encoded[..]).expect("it works");
    assert_eq!(c.encode(), encoded);

    let method = &c
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists");

    if let Some(constraints) = &method.constraints {
        let constraints_length_byte_cursor: usize = 4 + 32 + 1 + 32;
        assert_eq!(
            encoded[constraints_length_byte_cursor].swap_bits() + 1,
            constraints.len() as u8,
        );
    };
}

#[test]
fn it_works_decode_with_valid_constraints() {
    let encoded_cennznut: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let bad_type_id: Vec<u8> = vec![192, 0, 0b1000_0000, 0b0000_0001, 0b0000_0001];
    let n_too_short: Vec<u8> = vec![128, 0, 1];
    let n_too_large: Vec<u8> = vec![192, 0, 0b1000_0000, 0b1000_0000, 0b0000_1111];

    let encoded_with_bad_type_id: Vec<u8> = [encoded_cennznut.clone(), bad_type_id].concat();
    let encoded_with_n_too_short: Vec<u8> = [encoded_cennznut.clone(), n_too_short].concat();
    let encoded_with_n_too_large: Vec<u8> = [encoded_cennznut.clone(), n_too_large].concat();

    assert_eq!(
        CENNZnutV0::decode(&mut &encoded_with_bad_type_id[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
    assert_eq!(
        CENNZnutV0::decode(&mut &encoded_with_n_too_short[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
    assert_eq!(
        CENNZnutV0::decode(&mut &encoded_with_n_too_large[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
}

#[test]
fn it_works_with_lots_of_things_codec() {
    let method = Method::new("method_test").block_cooldown(123);
    let method2 = Method::new("method_test2").block_cooldown(321);

    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));
    methods.push((method2.name.clone(), method2.clone()));

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods.clone());
    let module2 = Module::new("module_test2")
        .block_cooldown(55_555)
        .methods(methods.clone());

    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));
    modules.push((module2.name.clone(), module2.clone()));

    let cennznut = CENNZnutV0 { modules };

    let encoded = vec![
        0, 0, 128, 160, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111,
        100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        222, 0, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 128, 0, 0, 160, 109, 111, 100, 117, 108,
        101, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 192, 155, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 0, 0, 0, 128, 109, 101, 116, 104,
        111, 100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 130, 128, 0, 0,
    ];
    assert_eq!(cennznut.encode(), encoded);
    assert_eq!(cennznut, CENNZnutV0::decode(&mut &encoded[..]).unwrap());
}

#[test]
fn it_validates() {
    let contract = Contract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(123)),
            PactType::StringLike(StringLike(b"test")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [
        PactType::Numeric(Numeric(123)),
        PactType::StringLike(StringLike(b"test")),
    ];

    assert_eq!(cennznut.validate(&module.name, &method.name, &args), Ok(()));
    assert_eq!(
        cennznut.validate("module_test2", &method.name, &args),
        Err("CENNZnut does not grant permission for module")
    );
    assert_eq!(
        cennznut.validate(&module.name, "method_test2", &args),
        Err("CENNZnut does not grant permission for method")
    );
}

#[test]
fn it_validates_error_with_bad_bytecode() {
    let contract = Contract {
        data_table: DataTable::new(vec![PactType::StringLike(StringLike(b"test"))]),
        bytecode: [OpCode::GT.into(), 0, 0, 1, 0].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [PactType::StringLike(StringLike(b"test"))];

    assert_eq!(
        cennznut.validate(&module.name, &method.name, &args),
        Err("error while interpreting constraints")
    );
}

#[test]
fn it_validates_error_with_false_constraints() {
    let contract = Contract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(123)),
            PactType::StringLike(StringLike(b"a")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [
        PactType::Numeric(Numeric(321)),
        PactType::StringLike(StringLike(b"b")),
    ];

    assert_eq!(
        cennznut.validate(&module.name, &method.name, &args),
        Err("CENNZnut does not grant permission for method arguments")
    );
}

#[test]
fn it_validates_with_empty_constraints() {
    let method = Method::new("method_test").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [
        PactType::Numeric(Numeric(0)),
        PactType::StringLike(StringLike(b"test")),
    ];

    assert_eq!(cennznut.validate(&module.name, &method.name, &args), Ok(()));
}

#[test]
fn it_works_get_pact() {
    // A CENNZnut with constraints set
    let encoded_with: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 192, 128, 16,
        246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224, 116, 101, 115, 116,
        105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1,
    ];

    let cennznut_with: CENNZnutV0 = Decode::decode(&mut &encoded_with[..]).expect("it works");

    let contract_with = cennznut_with
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists")
        .get_pact();

    if let Some(contract) = contract_with {
        assert_eq!(
            contract,
            Contract {
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
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let cennznut_without: CENNZnutV0 = Decode::decode(&mut &encoded_without[..]).expect("it works");

    let contract_without = cennznut_without
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists")
        .get_pact();

    assert_eq!(contract_without, None);
}
