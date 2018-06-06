#[macro_use]
extern crate error_chain;

use std::process::Command;

pub mod error;
use error::ResultExt;

/// returns whether `solc` is in path
/// `solc` is the C++ implementation of the solidity compiler.
pub fn is_solc_available() -> bool {
    solc_version().is_ok()
}

/// returns the output of `solc --version`.
/// more specifically the last line which is the version string.
/// `solc` is the C++ implementation of the solidity compiler.
pub fn solc_version() -> error::Result<String> {
    let output = Command::new("solc")
        .args(&["--version"])
        .output()
        .chain_err(|| "failed to run `solc --version`")?;
    let output_string =
        String::from_utf8(output.stdout).chain_err(|| "output from `solc --version` is not utf8")?;
    let version = output_string
        .lines()
        .last()
        .chain_err(|| "output from `solc --version` is empty")?
        .to_owned();
    Ok(version)
}

/// returns whether `solcjs` is in path.
/// `solcjs` is the javascript implementation of the solidity compiler.
pub fn is_solcjs_available() -> bool {
    solcjs_version().is_ok()
}

/// returns the output of `solcjs --version`.
/// more specifically the last line which is the version string.
/// `solcjs` is the javascript implementation of the solidity compiler.
pub fn solcjs_version() -> error::Result<String> {
    let output = Command::new("solcjs")
        .args(&["--version"])
        .output()
        .chain_err(|| "failed to run `solcjs --version`")?;
    let output_string =
        String::from_utf8(output.stdout).chain_err(|| "output from `solcjs --version` is not utf8")?;
    let version = output_string
        .lines()
        .last()
        .chain_err(|| "output from `solcjs --version` is empty")?
        .to_owned();
    Ok(version)
}
