#[macro_use]
extern crate error_chain;

use std::process::Command;

pub mod error;
use error::ResultExt;

/// returns whether `solc` is in path.
/// `solc` is the C++ implementation of the solidity compiler.
pub fn is_solc_available() -> bool {
    solc_version().is_ok()
}

/// returns the output of `solc --version`.
/// more specifically the last line which is the version string.
/// `solc` is the C++ implementation of the solidity compiler.
pub fn solc_version() -> error::Result<String> {
    version("solcjs")
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
    version("solcjs")
}

fn version(command_name: &str) -> error::Result<String> {
    let output = Command::new(command_name)
        .args(&["--version"])
        .output()
        .chain_err(|| format!("failed to run `{} --version`", command_name))?;
    let output_string = String::from_utf8(output.stdout)
        .chain_err(|| format!("output from `{} --version` is not utf8", command_name))?;
    let version = output_string
        .lines()
        .last()
        .chain_err(|| format!("output from `{} --version` is empty", command_name))?
        .to_owned();
    Ok(version)
}
