use crate::history::History;
use std::env;
use std::env::set_current_dir;
use std::fs::{
    metadata, read_dir, remove_dir_all, remove_file, File, Metadata, OpenOptions, ReadDir,
};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

// Crates for correct formatting of times
use crate::config::Config;
use chrono::prelude::{DateTime, Local};

/// Handles builtins
///
/// # Arguments
///
/// * `commands` - A string slice representing a command and its arguments
/// * `history` - An object that contains all previously entered commands
///
/// # Return value
///
/// True if the command was a builtin, else false.
pub fn builtin(
    commands: &[String],
    mut history: &mut History,
    config: &Config,
) -> Result<bool, Error> {
    match &commands.first().unwrap_or(&String::new())[..] {
        "ls" => {
            if let Err(e) = list_files_builtin(commands, config) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not list contents\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        "rm" => {
            if let Err(e) = file_remove_builtin(commands) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not remove file/directory\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        "touch" => {
            if let Err(e) = touch_builtin(commands) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not create file\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        "cd" => {
            if let Err(e) = change_dir_builtin(commands) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not change directories\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        "pwd" => {
            pwd_builtin();
            Ok(true)
        }
        "history" => {
            if let Err(e) = history_builtin(commands, &mut history) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not display history\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        "clear" => {
            if let Err(e) = clear_builtin(commands) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not clear the screen\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        "cat" => {
            if let Err(e) = display_file_contents(commands) {
                eprintln!(
                    "\x1b[38;2;255;0;0mError: Could not display file contents\n{}\x1b[0m",
                    e
                );
                return Err(e);
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// Implements a built-in version of the 'ls' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn list_files_builtin(args: &[String], config: &Config) -> Result<(), Error> {
    // If no directories were given
    if args.len() == 1 {
        // Get all paths in the current directory
        let paths: ReadDir = read_dir(".")?;

        // Print contents of current directory
        print_directory_contents(paths, config)?;
        println!()
    }
    // If ls was given multiple directories as an argument
    else if args.len() > 1 {
        // Array of directories given as arguments
        let directories: &[String] = &args[1..args.len()];

        // Loop through directories
        for directory in directories {
            // Check if directory exists
            let valid_directory: bool = Path::new(directory).exists();

            if valid_directory == true {
                // Get all paths that exists in the given directory
                let paths: ReadDir = read_dir(directory)?;

                // Length of right side of directory header
                let lhs_width: usize = (60 - directory.len()) / 2;

                // Length of left side of directory header
                let rhs_width: usize = if (lhs_width % 2) == 0 {
                    lhs_width + 1
                } else {
                    lhs_width
                };

                // Print contents of directory
                println!(
                    "{}[{}]{}",
                    "-".repeat(lhs_width),
                    directory,
                    "-".repeat(rhs_width)
                );
                print_directory_contents(paths, config)?;
                println!()
            } else {
                let error_message: String = "Directory ".to_owned() + directory + " does not exist";
                return Err(Error::new(ErrorKind::Other, error_message));
            }
        }
    }
    Ok(())
}

/// Handles printing and styling all the given paths
fn print_directory_contents(paths: ReadDir, config: &Config) -> Result<(), Error> {
    // Displaying content prompts
    println!("{:19}  {:41}", "Modified", "Name");
    println!("{:19}  {:41}", "-".repeat(19), "-".repeat(41));

    for path in paths {
        // Path for file
        let path_str: String = path?.path().display().to_string();

        // Metadata for the file
        let file_metadata: Metadata = metadata(&path_str)?;

        // Last modified time for a file in local time
        let file_modified_time: DateTime<Local> = file_metadata.modified()?.into();

        // If file is a directory
        if PathBuf::from(&path_str).is_dir() {
            // println!("\x1b[38;2;42;125;211mError\x1b[0m");
            let directory_name: String = "\x1b[38;2;".to_owned()
                + &config.get("directory_text_color")
                + "m"
                + &path_str.split("/").collect::<Vec<&str>>()[1].to_owned()
                + "/\x1b[0m";

            println!(
                "{:<19}  {:<41}",
                file_modified_time.format("%m-%d-%Y %I:%M %p"),
                directory_name
            );
        } else {
            let file_name = "\x1b[38;2;".to_owned()
                + &config.get("filename_text_color")
                + "m"
                + path_str.split("/").collect::<Vec<&str>>()[1]
                + "\x1b[0m";
            println!(
                "{:<19}  {:<41}",
                file_modified_time.format("%m-%d-%Y %I:%M %p"),
                file_name
            );
        }
    }
    Ok(())
}

/// Implements a built-in version of the 'rm' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn file_remove_builtin(args: &[String]) -> Result<(), Error> {
    // If no arguments are found
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "usage: rm [-r] <file1 file2 ...>",
        ));
    }
    // If '-r' flag is found
    else if args[1] == "-r" {
        for directory in &args[2..] {
            remove_dir_all(directory)?;
        }
    }
    // Remove all files listed
    else {
        for file in &args[1..] {
            remove_file(file)?;
        }
    }
    Ok(())
}

/// Implements a built-in version of the 'touch' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn touch_builtin(args: &[String]) -> Result<(), Error> {
    // If no arguments are given
    if args.len() <= 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "usage: touch <file1 file2 ...>",
        ));
    }

    //the loop will ignore the first element in the string array as it will be "touch"
    for file_path in &args[1..] {
        // File to be created or have its time updated
        let file: &Path = Path::new(file_path);

        //if the file already exists we add a new line to the file, and immediately remove it
        if file.exists() {
            let file_to_change: File = OpenOptions::new().append(true).write(true).open(file)?;

            //get metadata to access for adding and removing new lines
            let metadata: Metadata = file.metadata()?;

            // adds new line
            file_to_change.set_len(metadata.len() + 1)?;

            // removes new line
            file_to_change.set_len(file.metadata()?.len() - 1)?;
        }
        //if the file does not exist, create it
        else {
            File::create(&file)?;
        }
    }
    Ok(())
}

/// Implements a built-in version of the 'cd' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn change_dir_builtin(args: &[String]) -> Result<(), Error> {
    // If no arguments are given
    if args.len() == 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: cd <directory path>",
        ));
    }

    // If the given path is a valid directory
    return if PathBuf::from(&args[1]).is_dir() == true {
        set_current_dir(&args[1])
    } else {
        let error_message: String = args[1].to_owned() + " is not a valid directory";
        Err(Error::new(ErrorKind::Other, error_message))
    };
}

/// Implements a built-in version of the 'pwd' command.
fn pwd_builtin() {
    println!(
        "{}",
        env::current_dir()
            .expect("Error: Could not access current directory env")
            .display()
    )
}

/// Implements a built-in command history
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
/// * `history` - An object that contains all previously entered commands
fn history_builtin(args: &[String], history: &mut History) -> Result<(), Error> {
    // If no arguments are given
    if args.len() == 1 {
        // print all history
        history.display_full_history();
    }
    // If two arguments are given
    else if args.len() == 2 {
        // Check if the received argument is a number
        match &args[1].parse::<usize>() {
            Ok(ok) => history.display_num_commands(*ok),
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Non-number argument given",
                ))
            }
        }
    } else {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: history <num of previous commands>",
        ));
    }
    Ok(())
}

/// Implements a built-in command 'clear'
fn clear_builtin(args: &[String]) -> Result<(), Error> {
    // If too many arguments are given
    if args.len() > 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "Usage: clear"));
    }

    // Clearing the users screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    Ok(())
}

/// Implements a built-in command 'cat'
fn display_file_contents(args: &[String]) -> Result<(), Error> {
    // If no arguments or too many arguments are given
    if args.len() == 1 || args.len() > 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: cat <path to file>",
        ));
    }

    // Given file to display to the screen
    let display_file: File = OpenOptions::new().read(true).open(&args[1])?;

    // Buffered reader of given file
    let reader: BufReader<File> = BufReader::new(display_file);

    // Loop and print all lines of the file
    for line in reader.lines() {
        match line {
            Ok(ok) => println!("{}", ok),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
