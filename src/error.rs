use std::io;
use std::process::ExitStatus;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(io::Error);
    }

    errors {
        ExitStatusNotSuccess(command: String, exit_status: ExitStatus) {
            description("command exit status is not success (0)"),
            display("command (`{}`) is not success (0) but `{}`", command, exit_status)
        }
    }
}
