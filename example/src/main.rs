extern crate solc;

fn main() {
    println!("{}", solc::is_solc_available());
    println!("{}", solc::solc_version().unwrap());
}
