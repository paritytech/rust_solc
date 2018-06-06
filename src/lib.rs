#[macro_use]
extern crate error_chain;

use std::process::{Command, Output};
use std::path::Path;

pub mod error;
use error::ResultExt;

/// returns whether `solc` is in path.
///
/// `solc` is the C++ implementation of the solidity compiler.
pub fn is_solc_available() -> bool {
    solc_version().is_ok()
}

/// returns the output of `solc --version`.
///
/// more specifically the last line which is the version string.
/// `solc` is the C++ implementation of the solidity compiler.
pub fn solc_version() -> error::Result<String> {
    version("solcjs")
}

/// shells out to `solc` to compile
/// `input_file_path` into abi and bin files in `output_dir_path`.
///
/// `solc` is the C++ implementation of the solidity compiler.
pub fn solc_compile<A: AsRef<Path>, B: AsRef<Path>>(input_file_path: A, output_dir_path: B) -> error::Result<Output> {
    let command_output = Command::new("solc")
        .arg("--bin")
        .arg("--abi")
        .arg("--overwrite")
        .arg("--optimize")
        .arg("--output-dir").arg(output_dir_path.as_ref())
        .arg(input_file_path.as_ref())
        .output()
        .chain_err(|| "failed to run process `solc`")?;

     if !command_output.status.success() {
         return Err(error::ErrorKind::ExitStatusNotSuccess("solc".into(), command_output.status).into());
     }

    Ok(command_output)
}

/// returns whether `solcjs` is in path.
///
/// `solcjs` is the javascript implementation of the solidity compiler.
pub fn is_solcjs_available() -> bool {
    solcjs_version().is_ok()
}

/// returns the output of `solcjs --version`.
///
/// more specifically the last line which is the version string.
/// `solcjs` is the javascript implementation of the solidity compiler.
pub fn solcjs_version() -> error::Result<String> {
    version("solcjs")
}

fn version(command_name: &str) -> error::Result<String> {
    let command_output = Command::new(command_name)
        .arg("--version")
        .output()
        .chain_err(|| format!("failed to run `{} --version`", command_name))?;
    let stdout = String::from_utf8(command_output.stdout)
        .chain_err(|| format!("output from `{} --version` is not utf8", command_name))?;
    let version = stdout
        .lines()
        .last()
        .chain_err(|| format!("output from `{} --version` is empty", command_name))?
        .to_owned();
    Ok(version)
}
