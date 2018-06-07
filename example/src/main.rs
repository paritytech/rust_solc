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
                        "metadata",
                        "abi",
                        "evm.bytecode",
                        "evm.gasEstimates"
                    ]
                }
            }
        }
    });

    let output_json = solc::common_compile("solc", &input_json.to_string()).unwrap();

    println!("output_json =");
    println!("{}", output_json);
}
