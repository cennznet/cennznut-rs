#![cfg(test)]

use crate::{CENNZnutV0, Method, Module};
use hashbrown::HashMap;
use parity_codec::{Decode, Encode};
use std::string::{String, ToString};
use std::vec::Vec;

#[test]
fn it_works_encode() {
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: None,
    };
    let mut methods: HashMap<String, Method> = Default::default();
    methods.insert(method.name.clone(), method.clone());

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: None,
        methods,
        method_order: None,
    };

    let mut modules: HashMap<String, Module> = Default::default();
    modules.insert(module.name.clone(), module.clone());

    let cennznut = CENNZnutV0 {
        modules,
        module_order: None,
    };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
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
    let mut methods: HashMap<String, Method> = Default::default();
    methods.insert(method.name.clone(), method.clone());

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: None,
        methods,
        method_order: None,
    };

    let mut modules: HashMap<String, Module> = Default::default();
    modules.insert(module.name.clone(), module.clone());

    let cennznut = CENNZnutV0 {
        modules,
        module_order: None,
    };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101,
        115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    let mut methods: HashMap<String, Method> = Default::default();
    methods.insert(method.name.clone(), method.clone());

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods,
        method_order: None,
    };

    let mut modules: HashMap<String, Module> = Default::default();
    modules.insert(module.name.clone(), module.clone());

    let cennznut = CENNZnutV0 {
        modules,
        module_order: None,
    };

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
        0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 0, 109, 101, 116, 104, 111, 100,
        95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let c: CENNZnutV0 = Decode::decode(&mut &encoded[..]).expect("It works");
    assert_eq!(c.modules["module_test"].block_cooldown, Some(86_400));
}

#[test]
fn it_works_encode_with_method_cooldown() {
    let method = Method {
        name: "method_test".to_string(),
        block_cooldown: Some(123),
    };
    let mut methods: HashMap<String, Method> = Default::default();
    methods.insert(method.name.clone(), method.clone());

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods,
        method_order: None,
    };

    let mut modules: HashMap<String, Module> = Default::default();
    modules.insert(module.name.clone(), module.clone());

    let cennznut = CENNZnutV0 {
        modules,
        module_order: None,
    };

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
        0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111,
        100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        222, 0, 0, 0,
    ];
    let c: CENNZnutV0 = Decode::decode(&mut &encoded[..]).expect("It works");
    assert_eq!(c.modules["module_test"].block_cooldown, Some(86_400));
    assert_eq!(
        c.modules["module_test"].methods["method_test"].block_cooldown,
        Some(123)
    );
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

    let mut methods: HashMap<String, Method> = Default::default();
    methods.insert(method.name.clone(), method.clone());
    methods.insert(method2.name.clone(), method2.clone());

    let module = Module {
        name: "module_test".to_string(),
        block_cooldown: Some(86_400),
        methods: methods.clone(),
        method_order: None,
    };

    let module2 = Module {
        name: "module_test2".to_string(),
        block_cooldown: Some(55_555),
        methods: methods,
        method_order: None,
    };

    let mut modules: HashMap<String, Module> = Default::default();
    modules.insert(module.name.clone(), module.clone());
    modules.insert(module2.name.clone(), module2.clone());

    let cennznut = CENNZnutV0 {
        modules,
        module_order: None,
    };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 128, 160, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104,
            111, 100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 130, 128, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 0, 0, 0, 160, 109,
            111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 155, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130,
            128, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 0, 0, 0
        ]
    );
    let encoded = vec![
        0, 0, 128, 160, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111,
        100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 130, 128, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 0, 0, 0, 160, 109, 111, 100, 117,
        108, 101, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 192, 155, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 50, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 128, 0, 0, 128, 109, 101, 116,
        104, 111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 222, 0, 0, 0,
    ];
    // TODO: ignore method order / module order internal fields
    // assert_eq!(cennznut, CENNZnutV0::decode(&mut &encoded[..]).unwrap());
}
