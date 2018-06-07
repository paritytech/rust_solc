extern crate solc;

fn main() {
    // always rerun build script if contract has changed
    println!("cargo:rerun-if-changed=./contracts/test.sol");

    // solc::common_compile("./contracts", "./contracts").unwrap();
}
