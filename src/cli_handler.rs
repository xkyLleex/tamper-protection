

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
    Red,
}

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
        CliColor::Blue => "\x1b[47m\x1b[94m",
        CliColor::Yellow => "\x1b[47m\x1b[93m",
        CliColor::Red => "\x1b[47m\x1b[91m",
    };
    print!("{}", color_code);
    println!("{}\x1b[0m", text);
}

pub fn help() {
    print("
        Tamper Protection - Folder Hash Code Calculator
        - Usage:
          - tamperProtection <folder_path> <salt>                Calculate hash for a folder
          - tamperProtection <-h | --help | help>                Show help information
          - tamperProtection <-v | --validate | validate> <hash_code> <folder_path> <salt>    Validate folder integrity

        - Example:
          - tamperProtection ./my_folder my_salt
          - tamperProtection help
          - tamperProtection validate XXX ./my_folder my_salt

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
    if args.len() == 2 {
        if &args[1] == "-h" || &args[1] == "--help" || &args[1] == "help" {
            return CliMode::Help;
        }
    } else if args.len() == 3 {
        return CliMode::GetHash;
    } else if args.len() == 5 {
        if &args[1] != "-v" && &args[1] != "--validate" && &args[1] != "validate" {
            return CliMode::Error;
        }
        return CliMode::Validate;
    }
    CliMode::Error
}