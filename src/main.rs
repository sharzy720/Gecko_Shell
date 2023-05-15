pub mod builtin;
pub mod history;
pub mod parser;
pub mod redirect;
pub mod utils;

use ctrlc::set_handler;
use crate::builtin::builtin;
use crate::history::History;
use crate::utils::{execute, parse_line, prompt_and_read};

/// An implementation of a simple UNIX shell.  This program supports:
///    - Running processes
///    - Redirecting standard output (>)
///    - Redirecting standard input (<)
///    - Appending standard output to a file (>>)
///    - Redirecting both standard output and standard input (&>)
///    - Creating process pipelines (p1 | p2 | ...)
///    - Interrupting a running process (e.g., ctrl-C)
///    - A built-in version of the 'ls' command
///    - A built-in version of the 'rm' command
///    - A built-in version of the 'touch' command
///    - A built-in version of the 'cd' command
///    - A built-in version of the 'pwd' command
///    - A built-in 'history' list
///
/// Among the many things it does _NOT_ support are:
///    - Environment variables
///    - Appending standard error to a file (2>>)
///    - Backgrounding processes (p1&)
///    - Unconditionally chaining processes (p1;p2)
///    - Conditionally chaining processes (p1 && p2 or p1 || p2)
///    - re-executing history commands


fn main() {
    // History object to track every command entered during the lifetime of the program
    let mut history: History = History::new();

    // Allows program to not be stopped when 'CTRL+C' is entered
    set_handler(|| eprint!("")).expect("Error setting Ctrl-C handler");

    loop {
        // Entire entered line
        let tokens: Vec<String> = prompt_and_read().unwrap_or(Vec::new());

        history.add_to_history(&tokens);

        // Check if user want to run a builtin or not
        if let Ok(false) = builtin(&tokens, &mut history) {
            // Returned process from parsed line
            let parsed_command = parse_line(&tokens, None);

            if let Ok(Some(mut child)) = parsed_command {

                if let Err(e) = execute(&mut child) {

                    // Stops shell when exit is entered
                    if &tokens[0] == "exit" { break; }
                    else { eprintln!("\x1b[38;2;255;0;0mError: Could not execute process.\n{}\x1b[0m", e); }
                }
            } else {
                // Reasons this will execute:
                // * User entered only whitespace
                // * CTRL+C or CTRL+D was pressed in parent process
                // * One of the redirect functions was last to return
                //      ie: cat input.txt << file.txt
                // * Pipe encountered an error and returned early
                // * An error occurred parsing the line
                //      Specifically, writing to stdout/stderr in parse_line

                if let Err(e) = parsed_command {
                    eprintln!("\x1b[38;2;255;0;0m{}\x1b[0m", e);
                }

            }
        } else {
            // Reasons this will execute:
            // * A builtin function returned an error
            // * A builtin function returned true
        }
    }
}
