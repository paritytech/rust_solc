#[macro_use]
extern crate error_chain;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

pub mod error;
use error::ResultExt;

/// shells out to either `solc` or `solcjs` (whichever is available in that order)
/// to compile all solidity files in `input_dir_path`
/// into abi and bin files in `output_dir_path`.
pub fn compile<A: AsRef<Path>, B: AsRef<Path>>(
    input_dir_path: A,
    output_dir_path: B,
) -> error::Result<()> {
    let is_solc_available = is_solc_available();

    if !is_solc_available && !is_solcjs_available() {
        return Err(error::ErrorKind::NoSolidityCompilerFound.into());
    }

    for input_file_path in solidity_file_paths(input_dir_path)? {
        if is_solc_available {
            solc_compile(input_file_path, &output_dir_path)?;
        } else {
            solcjs_compile(input_file_path, &output_dir_path)?;
        }
    }
    Ok(())
}

/// returns whether `solc` is in path.
///
/// `solc` is the C++ implementation of the solidity compiler.
pub fn is_solc_available() -> bool {
    solc_version().is_ok()
}

/// returns the output of `solc --version`.
///
/// more specifically returns the last output line which is the version string.
/// `solc` is the C++ implementation of the solidity compiler.
pub fn solc_version() -> error::Result<String> {
    common_version("solc")
}

/// shells out to `solc` to compile
/// `input_file_path` into abi and bin files in `output_dir_path`.
///
/// `solc` is the C++ implementation of the solidity compiler.
pub fn solc_compile<A: AsRef<Path>, B: AsRef<Path>>(
    input_file_path: A,
    output_dir_path: B,
) -> error::Result<Output> {
    let command_output = Command::new("solc")
        .arg("--bin")
        .arg("--abi")
        .arg("--overwrite")
        .arg("--optimize")
        .arg("--output-dir")
        .arg(output_dir_path.as_ref())
        .arg(input_file_path.as_ref())
        .output()
        .chain_err(|| "failed to run process `solc`")?;

    if !command_output.status.success() {
        return Err(
            error::ErrorKind::ExitStatusNotSuccess("solc".into(), command_output.status).into(),
        );
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
/// more specifically returns the last output line which is the version string.
/// `solcjs` is the javascript implementation of the solidity compiler.
pub fn solcjs_version() -> error::Result<String> {
    common_version("solcjs")
}

/// shells out to `solcjs` to compile
/// `input_file_path` into abi and bin files in `output_dir_path`.
///
/// `solcjs` is the javascript implementation of the solidity compiler.
pub fn solcjs_compile<A: AsRef<Path>, B: AsRef<Path>>(
    input_file_path: A,
    output_dir_path: B,
) -> error::Result<Output> {
    let command_output = Command::new("solcjs")
        .arg("--bin")
        .arg("--abi")
        .arg("--overwrite")
        .arg("--optimize")
        .arg("--output-dir")
        .arg(output_dir_path.as_ref())
        .arg(input_file_path.as_ref())
        .output()
        .chain_err(|| "failed to run process `solcjs`")?;

    if !command_output.status.success() {
        return Err(
            error::ErrorKind::ExitStatusNotSuccess("solcjs".into(), command_output.status).into(),
        );
    }

    Ok(command_output)
}

/// returns all solidity files in `directory`
pub fn solidity_file_paths<T: AsRef<Path>>(directory: T) -> std::io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();

    for maybe_entry in std::fs::read_dir(directory)? {
        let path = maybe_entry?.path();
        if let Some(extension) = path.extension() {
            if extension != "sol" {
                continue;
            }
        }
        results.push(path);
    }

    Ok(results)
}

/// version code that's common for `solc` and `solcjs`
fn common_version(command_name: &str) -> error::Result<String> {
    let command_output = Command::new(command_name)
        .arg("--version")
        .output()
        .chain_err(|| format!("failed to run `{} --version`", command_name))?;
    if !command_output.status.success() {
        return Err(error::ErrorKind::ExitStatusNotSuccess(
            command_name.to_owned(),
            command_output.status,
        ).into());
    }
    let stdout = String::from_utf8(command_output.stdout)
        .chain_err(|| format!("output from `{} --version` is not utf8", command_name))?;
    let version = stdout
        .lines()
        .last()
        .chain_err(|| format!("output from `{} --version` is empty", command_name))?
        .to_owned();
    Ok(version)
}

pub fn standard_json(input_json: &str) -> error::Result<String> {
    let is_solc_available = is_solc_available();

    if !is_solc_available && !is_solcjs_available() {
        return Err(error::ErrorKind::NoSolidityCompilerFound.into());
    }

    let command_name = if is_solc_available {
        "solc"
    } else {
        "solcjs"
    };

    common_standard_json(command_name, input_json)
}

pub fn common_standard_json(command_name: &str, input_json: &str) -> error::Result<String> {
    let full_command = format!("{} --standard-json", command_name);

    let mut process = Command::new(command_name)
        .arg("--standard-json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .chain_err(|| format!("failed to spawn process `{}`", &full_command))?;

    {
        let stdin = process
            .stdin
            .as_mut()
            .chain_err(|| format!("failed to open stdin for process `{}`", &full_command))?;

        stdin.write_all(input_json.as_bytes()).chain_err(|| {
            format!(
                "failed to write input json to stdin for process `{}`",
                &full_command
            )
        })?;
    }

    let output = process.wait_with_output().chain_err(|| {
        format!(
            "failed to read output json from stdout for process `{}`",
            &full_command
        )
    })?;

    if !output.status.success() {
        return Err(
            error::ErrorKind::ExitStatusNotSuccess(full_command.clone(), output.status).into(),
        );
    }

    let output_json = String::from_utf8(output.stdout).chain_err(|| {
        format!(
            "stdout from process `{}` must be utf8 but isn't",
            full_command
        )
    })?;

    Ok(output_json)
}
