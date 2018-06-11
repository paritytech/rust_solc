extern crate solc;
#[macro_use]
extern crate serde_json;

fn main() {
    println!("solc available = {}", solc::is_solc_available());
    println!("solc version = {:?}", solc::solc_version());
    println!("");
    println!("solcjs available = {}", solc::is_solcjs_available());
    println!("solcjs version = {:?}", solc::solcjs_version());

    let input_json = json!({
        "language": "Solidity",
        "sources": {
            "test.sol": {
                "content": include_str!("../contracts/test.sol"),
            }
        },
        "settings": {
            "optimizer": {
                "enabled": true,
                "runs": 200,
            },
            "outputSelection": {
                "*": {
                    "*": [
                        // "metadata",
                        "abi",
                        "evm.bytecode.object",
                        "evm.bytecode.sourceMap",
                        "evm.gasEstimates"
                    ]
                },
                // "*": {
                //     "": ["ast"],
                // }
                // "*": {
                //     "*": {
                //         "*": [
                //             "evm.bytecode.sourceMap",
                //         ]
                //     }
                // }
            }
        }
    });

    let output_string = solc::standard_json(&input_json.to_string()).unwrap();
    let output_json: serde_json::Value = serde_json::from_str(&output_string).unwrap();

    println!("output_json =");
    println!("{:#?}", output_json);
}
