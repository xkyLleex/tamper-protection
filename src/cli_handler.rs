

pub enum CliMode {
    Help,
    GetHash,
    Validate,
    Error,
}

pub enum CliColor {
    Default,
    Blue,
    Yellow,
    Green,
    Red,
}

use crate::global;

/// Print colored text to the console based on the specified color.
///
/// # Arguments
/// - `text` - The text to print.
/// - `color` - The color to use for printing.
/// 
/// Examples
/// ```
/// print("This is a blue message", CliColor::Blue);
/// ```
pub fn print(text: &str, color: CliColor) {
    // ANSI escape codes for coloring
    let color_code = match color {
        CliColor::Default => "\x1b[0m",
        CliColor::Blue    => "\x1b[94m\x1b[103m",
        CliColor::Yellow  => "\x1b[93m\x1b[104m",
        CliColor::Green   => "\x1b[92m\x1b[105m",
        CliColor::Red     => "\x1b[91m\x1b[106m",
    };

    if global::get_color_mode() {
        print!("{}", color_code);
    }

    println!("{}\x1b[0m", text);
}

pub fn help() {
    print("
Tamper Protection - Folder Hash Code Calculator
- Usage:
    - tamperProtection <folder_path> <salt> [no | nocolor]               Calculate hash for a folder
    - tamperProtection <-h | --help | help> [no | nocolor]               Show help information
    - tamperProtection <hash_code> <folder_path> <salt> [no | nocolor]   Validate folder integrity

- Example:
    - tamperProtection ./my_folder my_salt
    - tamperProtection help
    - tamperProtection my_folder_hash_code ./my_folder my_salt
    - tamperProtection my_folder_hash_code ./my_folder my_salt nocolor
    ", CliColor::Default);
}


/// Determine the CLI mode based on the provided arguments.
/// 
/// # Arguments
/// - `args` - A vector of command-line arguments.
/// 
/// # Returns
/// A `CliMode` enum indicating the operation mode.
/// 
pub fn args_handler(args: &Vec<String>) -> CliMode {
    if args[args.len() - 1] == "no" || args[args.len() - 1] == "nocolor" {
        global::set_color_mode(false);
        return args_handler(&args[0..args.len() - 1].to_vec()); // Re-evaluate without the last argument
    }

    if args.len() == 2 {
        if &args[1] == "-h" || &args[1] == "--help" || &args[1] == "help" {
            return CliMode::Help;
        }
    } else if args.len() == 3 {
        return CliMode::GetHash;
    } else if args.len() == 4 {
        return CliMode::Validate;
    }
    CliMode::Error
}