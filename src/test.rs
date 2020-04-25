// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! CENNZnut - Integration Tests
//!

#![warn(clippy::pedantic)]
#![cfg(test)]

use crate::CENNZnut;

use codec::Decode;
use std::vec::Vec;

#[test]
fn it_fails_decode_with_invalid_constraints() {
    let encoded_cennznut: Vec<u8> = vec![
        0, 0, 128, 64, 182, 246, 38, 174, 54, 166, 250, 46, 166, 206, 46, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 182, 166, 46, 22, 246, 38, 250, 46, 166, 206,
        46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let bad_type_id: Vec<u8> = vec![192, 0, 0b1000_0000, 0b0000_0001, 0b0000_0001];
    let n_too_short: Vec<u8> = vec![128, 0, 1];
    let n_too_large: Vec<u8> = vec![192, 0, 0b1000_0000, 0b1000_0000, 0b0000_1111];

    let encoded_with_bad_type_id: Vec<u8> = [encoded_cennznut.clone(), bad_type_id].concat();
    let encoded_with_n_too_short: Vec<u8> = [encoded_cennznut.clone(), n_too_short].concat();
    let encoded_with_n_too_large: Vec<u8> = [encoded_cennznut, n_too_large].concat();

    assert_eq!(
        CENNZnut::decode(&mut &encoded_with_bad_type_id[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
    assert_eq!(
        CENNZnut::decode(&mut &encoded_with_n_too_short[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
    assert_eq!(
        CENNZnut::decode(&mut &encoded_with_n_too_large[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
}
