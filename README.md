# cennznut-rs
[![CircleCI](https://circleci.com/gh/cennznet/cennznut-rs.svg?style=svg)](https://circleci.com/gh/cennznet/cennznut-rs)

The CENNZNet permission domain codec rust implementation.  
Targeted for embedding within doughnut protocol permission domain.  

The full spec. is available [here](https://github.com/cennznet/doughnut-paper/blob/master/CENNZnet_format.md)  
```

 ## Generate JS/Wasm bindings
 This crate also provides generated JS bindings using [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/). To generate the package run:
 ```bash
 # install wasm pack
 curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

 # build
 cd js/ && yarn build

 # Run tests
 yarn test
 ```

