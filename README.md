# rust-solc

**this is a work in progress**

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/snd/rust_solc.svg?branch=master
[travis-url]: https://travis-ci.org/snd/rust_solc

**easily compile solidity files from rust**

```rust
extern crate solc;

fn main() {
    let input_directory = "./contracts";
    let output_directory = "./contracts";

    // first tries solc
    // then tries solcjs
    // returns error if no compiler available
    solc::compile(&input_directory, &output_directory).unwrap();

    // now `./contracts` contains a `*.bin` and a `*.abi` file
    // for every contract found in `./contracts/test.sol`
}
```

this is an early version that likely misses features.
[open an issue if you're missing something](https://github.com/snd/rust_solc/issues/new)
