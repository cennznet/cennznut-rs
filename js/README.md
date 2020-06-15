# @cennznet/cennznut-wasm

Wasm CENNZnut codec and maker.
Currently compliant with the version 0 spec.  

## Create a Cennznut (unsigned)

```js
const Cennznut = require('@cennznet/cennznut-wasm').default;

// modules (first param) is a vector of tuple of module name and module -  Vec<(ModuleName, Module)>
// Method is a vector of tuple method name and method - Vec<(MethodName, Method)>
const methods = "methods":[
                                   [  // method name
                                       "test_method_check1",  { // Method
                                                     "name":"test_method_check11",
                                                     "block_cooldown":270549120,
                                                     "constraints":null
                                                     }
                                   ]
                               ]
const modules =  [
                       [    // Module name
                           "test_module_check1",  { // Module
                               "name":"test_module_check1",
                               "block_cooldown":270549120,
                               "methods":[
                                   [
                                       "test_method_check1",  {
                                                     "name":"test_method_check11",
                                                     "block_cooldown":270549120,
                                                     "constraints":null
                                                     }
                                   ],
                                 ]
                           }
                       ]]
 // contract (second argument passed) is vector of contractAddress[u8;] and contract [Contract] - Vec<(ContractAddress, Contract)>
 contract = [[u8;], Contract]
 const contracts  = [
                      [  // contract_address
                        [27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],
                        // Contract object
                        {
                            "address":
                                [27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],
                            "block_cooldown":
                                270549120
                        }
                      ]
                    ];

return new Cennznut(modules, contracts);
```

## Inspect Cennznut Fields

Getter functions for inspecting a CENNZnut

```js
const cennznut = new Cennznut(...);
  module = cennznut.getModule("module_test"); // returns the module with name 'module_test' if exist else returns undefined
  contracts =  cennznut.getContract(contract_address); // returns contract with contract_address if exist else returns undefined

```

## Cennznut Encoding and Decoding

`Encoding`: Encode a cennznut object

`Decoding`: Create a cennznut object from a encoded cennznut

```js
const Cennznut = require('@cennznet/cennznut-wasm').default;

const payload = [0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, ..., 0, 0, 0, 0, 0, 0, 0, 0, 0];
const cennznut = Cennznut.decode(payload);

const cennznut = new Cennznut(...);
const encoded = cennznut.encode();
```
