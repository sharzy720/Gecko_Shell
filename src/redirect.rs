use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind, Write};
use std::process::{Child, Command, Output, Stdio};

/// Handles redirection
///
/// # Arguments
///
/// * `redirector` - A string representing the redirect operation to perform, if any
/// * `command` - A slice of strings representing a command and its arguments
/// * `process` - An `Option` representing a read-to-execute Command to be
///               modified/executed/returned
///
/// # Return value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
pub fn redirect(
    redirector: &str,
    command: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    match redirector {
        // ---- Append redirection ----
        ">>" => handle_append_redirect(command, process),

        // ---- stderr redirection ----
        "2>" => handle_stderr_redirect(command, process),

        // ---- stdout and stderr redirection ----
        "&>" => handle_stdout_stderr_redirect(command, process),

        // ---- Stdout redirection ----
        ">" | "1>" => handle_stdout_redirect(command, process),

        // ---- Stdin redirection ----
        "<" => handle_stdin_redirect(command, process),

        // ---- pipe in between processes ----
        "|" => handle_pipe(command, process),
        _ => {
            let mut setup_command: Command = Command::new(&command[0]);
            setup_command.args(&command[1..command.len()]);
            Ok(Option::from(setup_command))
        }
    }
}

/// Redirects standard output from this ready-to-execute Command to the file with the specified
/// name.
/// Data is appended to the file instead of truncating existing file.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_append_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    // File to append to
    let stdout_file: File = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&tokens[0])?;

    // Received process with stdout redirected
    let mut process_redirected: Command = process.unwrap();
    process_redirected.stdout(stdout_file);

    Ok(Option::from(process_redirected))
}

/// Redirects standard error from this ready-to-execute Command to the file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stderr_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //check that a file for redirect was provided
    if tokens.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: <command> [args] 2> <file>",
        ));
    }

    //the name of the file should be the only item in the array
    let file_name: &String = &tokens[0];

    //create the file to write stderr
    let file: File = File::create(file_name)?;

    //create a process from the passed argument
    let mut command: Command = process.unwrap();

    //redirect the standard error of the process to the file
    command.stderr(Stdio::from(file));

    //get output from process
    let output: Output = command.output()?;

    //send stderr from io to the output of the process
    io::stderr().write_all(&output.stderr)?;

    Ok(Option::from(command))
}

/// Redirects stdout and stderr from this ready-to-execute Command to the file with the specified
/// name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stdout_stderr_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    // File that stdout will print to
    let stdout_file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tokens[0])?;

    // File that stderr will print to
    let stderr_file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tokens[0])?;

    // New edited command
    let mut command: Command = process.unwrap();
    command
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file));

    Ok(Option::from(command))
}

/// Redirects standard output from this ready-to-execute Command to a file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stdout_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    // File to write stdout to
    let stdout_file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&tokens[0])?;

    // Process with its stdout redirected to a file
    let mut process_redirected: Command = process.unwrap();
    process_redirected.stdout(stdout_file);

    Ok(Option::from(process_redirected))
}

/// Redirects standard input to this ready-to-execute Command from the file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stdin_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //check that a file for redirect was provided
    if tokens.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: <command> [args] < <file>",
        ));
    }

    //the name of the file should be the only item in the array
    let file_name: &String = &tokens[0];

    //open the file
    let file: File = File::open(file_name)?;

    //create new process from passed parameter
    let mut command: Command = process.ok_or(Error::new(
        ErrorKind::InvalidInput,
        "Usage: <command> [args] < <file>",
    ))?;

    //enable set the stdin of the process to be the file
    command.stdin(Stdio::from(file));

    Ok(Option::from(command))
}

/// Partial implementation of a pipe between two processes.
///
/// # Arguments
///
/// * `commands` - A vector of strings corresponding to a command/operator and its arguments
/// * `process` - A ready to run Command whose output should be set up to be piped into a new
///               ready-to-run-command
///    process is the the left hand side process in a `LHS process | RHS process`
///
/// # Return value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_pipe(commands: &[String], process: Option<Command>) -> Result<Option<Command>, Error> {
    // If RHS of pipe is empty
    if commands.len() == 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: <command> | <command>",
        ));
    }

    // Create the RHS command
    let mut setup_command: Command = Command::new(&commands[0]);

    // If the RHS command has arguments add them
    if commands.len() > 1 {
        let command_args: &[String] = &commands[1..commands.len()];
        setup_command.args(command_args);
    }

    // Get the output of the LHS command
    let process_output: Child = process.unwrap().stdout(Stdio::piped()).spawn()?;

    // Pipe the output of the LHS command to the RHS command
    setup_command.stdin(process_output.stdout.unwrap());

    Ok(Option::from(setup_command))
}
