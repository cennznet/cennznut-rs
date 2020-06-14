const Cennznut = require('../libNode/cennznut').CENNZnut;

// The test used is same as it_works_decode_with_method_cooldown in rust
let encodedCennznut = new Uint8Array([
    0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 0, 109, 101, 116, 104, 111,
    100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0
]);

describe("wasm cennznut", () => {
  test("it decodes and verifies", () => {
    let cennznut = Cennznut.decode(encodedCennznut);
    expect(cennznut.encode()).toEqual(encodedCennznut);
    let module = cennznut.getModule("module_test");
    expect(module.name).toEqual('module_test');
    expect(module.block_cooldown).toEqual(86400);
    expect(module.methods[0]).toContain("method_test");
  });

  test ("create instance of cennznut", () => {
    const contract_address = new Uint8Array([
      27, 137,  65,  29, 182,  25, 157,  61,
      226,  13, 230,  14, 111,   6,  25, 186,
      227, 117, 177, 244, 172, 147,  40, 119,
      209,  78,  13, 109, 236, 119, 205, 202
    ]);

    const module = [
        [
            "test_module_check1",  {
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
                    [
                        "test_method_check2", {
                              "name":"test_method_check12",
                              "block_cooldown":270545024,
                              "constraints":null
                          }
                    ]
                ]
            }
        ],
      [
        "test_module_check2",  {
        "name":"test_module_check2",
        "block_cooldown":270541120,
        "methods":[
          [
            "test_method_check2",  {
            "name":"test_method_check21",
            "block_cooldown":270541120,
            "constraints":null
          }
          ]
        ]
      }
      ],
    ];

    const contract  = [[[27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],{"address":[27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],"block_cooldown":270549120}]];
    let cennznutNew = new Cennznut(module, contract);
    let extract_module = cennznutNew.getModule("test_module_check1");
    expect(extract_module.name).toEqual('test_module_check1');
    expect(extract_module.block_cooldown).toEqual(270549120);
    expect(extract_module.methods[0]).toContain("test_method_check1");
    let extract_contract = cennznutNew.getContract(contract_address);
    expect(extract_contract.block_cooldown).toEqual(270549120);
    expect(cennznutNew.verifyContract(contract_address)).toEqual(true);
  });

});
