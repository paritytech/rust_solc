# rust-solc

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/snd/rust_solc.svg?branch=master
[travis-url]: https://travis-ci.org/snd/rust_solc

**easily to compile solidity files from rust**

```rust
extern crate solc;

fn main() {
    // first tries solc, then tries solcjs, errors if no compiler available
    solc::compile("./contracts/test.sol", "./contracts").unwrap();
}

```

this is an early version that likely misses features.
[open an issue if you're missing something](https://github.com/snd/rust_solc/issues/new)
