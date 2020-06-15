// Copyright 2019-2020 Centrality Investments Limited

//! Provide JS-Rust API bindings to create and inspect Cennznut
use cennznut::{
    v0::{contract::Contract, module::Module, CENNZnutV0},
    CENNZnut,
};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[inline]
fn from_slice_32(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    if bytes.len() < 32 {
        log("expected 32 byte array");
        return array;
    }
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

/// A js handle for a rust versioned cennznut struct
#[wasm_bindgen(js_name = CENNZnut)]
pub struct JsHandle(CENNZnut);

#[wasm_bindgen(js_class = CENNZnut)]
#[allow(irrefutable_let_patterns)]
impl JsHandle {
    #[wasm_bindgen(constructor)]
    /// Create a new Cennznut, it is always v0 for now
    pub fn new(modules: &JsValue, contracts: &JsValue) -> Self {
        let modules_vec: Vec<(String, Module)> = modules
            .into_serde()
            .expect("Deserialization of modules failed");
        let contract_vec: Vec<([u8; 32], Contract)> = contracts
            .into_serde()
            .expect("Deserialization of contracts failed");
        let cennznut: CENNZnutV0 = CENNZnutV0 {
            modules: modules_vec,
            contracts: contract_vec,
        };
        JsHandle(CENNZnut::V0(cennznut))
    }

    #[allow(non_snake_case)]
    /// Return the cennznut module
    pub fn getModule(&self, module: &str) -> JsValue {
        if let CENNZnut::V0(cennznut) = &self.0 {
            if cennznut.get_module(module).is_none() {
                return JsValue::UNDEFINED;
            }
            return JsValue::from_serde(&cennznut.get_module(module).unwrap()).unwrap();
        }
        panic!("unsupported cennznut version");
    }

    #[allow(non_snake_case)]
    /// Return the cennznut contract
    pub fn getContract(&self, contract_address: &[u8]) -> JsValue {
        if let CENNZnut::V0(cennznut) = &self.0 {
            if cennznut
                .get_contract(from_slice_32(contract_address))
                .is_none()
            {
                return JsValue::UNDEFINED;
            }
            return JsValue::from_serde(
                &cennznut
                    .get_contract(from_slice_32(contract_address))
                    .unwrap(),
            )
            .unwrap();
        }
        panic!("unsupported cennznut version");
    }

    #[allow(non_snake_case)]
    /// Verify cennznut is valid for contract_address
    pub fn verifyContract(&self, contract_address: &[u8]) -> bool {
        if let CENNZnut::V0(cennznut) = &self.0 {
            return cennznut
                .validate_contract(from_slice_32(contract_address))
                .is_ok();
        }
        panic!("unsupported cennznut version");
    }

    /// Encode the cennznut into bytes
    pub fn encode(&mut self) -> Vec<u8> {
        self.0.encode()
    }

    /// Decode a version 0 cennznut from `input` bytes
    pub fn decode(input: &[u8]) -> Result<JsHandle, JsValue> {
        match CENNZnut::decode(&mut &input[..]) {
            Ok(cennznut) => Ok(JsHandle(cennznut)),
            Err(err) => {
                log(&format!("failed decoding: {:?}", err));
                Err(JsValue::undefined())
            }
        }
    }
}
