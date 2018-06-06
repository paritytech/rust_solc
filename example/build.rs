extern crate solc;

fn main() {
    // always rerun build script if contract has changed
    println!("cargo:rerun-if-changed=./contracts/test.sol");

    // println!("cargo:rustc-env=SOLC_VERSION={}", solc::solc_version);
    solc::compile("./contracts/test.sol", "./contracts").unwrap();
}
