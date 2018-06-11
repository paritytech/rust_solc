extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;

use_contract!(
    sender_test,
    "GetSenderTest",
    "./contracts/GetSenderTest.abi"
);
use_contract!(value_test, "GetValueTest", "./contracts/GetValueTest.abi");
