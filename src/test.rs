#![cfg(test)]

use crate::{CENNZnutV0, Method, Module, Validate};
use codec::{Decode, Encode};
use std::string::{String, ToString};
use std::vec::Vec;

#[test]
fn it_works_encode() {
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: None,
    };
    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: None,
        methods,
    };

    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));

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
fn it_works_encode_one_module() {
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: None,
    };
    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: None,
        methods,
    };

    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));

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
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: None,
    };
    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods,
    };

    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));

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
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: Some(123),
    };
    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods,
    };

    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));

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
#[should_panic(expected = "expected version : 0")]
fn it_works_decode_with_version_0() {
    let encoded: Vec<u8> = vec![
        1, 2, 3, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111, 100,
        95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222,
        0, 0, 0,
    ];
    CENNZnutV0::decode(&mut &encoded[..]).unwrap();
}

#[test]
fn it_works_with_lots_of_things_codec() {
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: Some(123),
    };
    let method2 = Method {
        name: "method_test2".to_string(),
        block_cooldown: Some(321),
    };

    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));
    methods.push((method2.name.clone(), method2.clone()));

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods: methods.clone(),
    };

    let module2 = Module {
        name: "module_test2".to_string(),
        block_cooldown: Some(55_555),
        methods: methods,
    };

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
fn it_works_with_validation() {
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: Some(123),
    };

    let mut methods: Vec<(String, Method)> = Default::default();
    methods.push((method.name.clone(), method.clone()));

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods: methods.clone(),
    };
    let mut modules: Vec<(String, Module)> = Default::default();
    modules.push((module.name.clone(), module.clone()));

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(cennznut.validate(&module.name, &method.name), Ok(()));
    assert_eq!(cennznut.validate("module_test2", &method.name), Err("Doughnut does not grant permission for module"));
    assert_eq!(cennznut.validate(&module.name, "method_test2"), Err("Doughnut does not grant permission for method"));
}
