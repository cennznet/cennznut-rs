# cennznut-rs
[![CircleCI](https://circleci.com/gh/cennznet/cennznut-rs.svg?style=svg)](https://circleci.com/gh/cennznet/cennznut-rs)

The CENNZnet permission domain set and codec.  
Intended for use with Doughnuts on CENNZnet to provide safe, delegated transactions.  

The formal spec. is available [here](https://github.com/cennznet/doughnut-paper/blob/master/CENNZnet_format.md)  

## Generate JS/Wasm bindings

This crate also generates an npm package [@cennznet/cennznut-wasm](https://www.npmjs.com/package/@cennznet/cennznut-wasm)
using [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/).

To generate the package run:
```bash
# install wasm pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# build
cd js/ && yarn build

# Run tests
yarn test
```

