extern crate solc;

fn main() {
    println!("solc available = {}", solc::is_solc_available());
    println!("solc version = {:?}", solc::solc_version());
    println!("");
    println!("solcjs available = {}", solc::is_solcjs_available());
    println!("solcjs version = {:?}", solc::solcjs_version());
}
