use std::{
    env, path::Path
};

mod cli_handler;
mod hash_calc;

/// Program function to get the hash of a folder with a given salt.
/// 
/// # Arguments
/// - `args` - [tamperProtection <folder_path> <salt>]
/// 
/// # Returns
/// A `String` containing the hash code or an empty string in case of error.
/// 
fn program_get_hash(args : &Vec<String>) -> String {
    // Start preset hash code calculation
    let folder_to_hash: &Path = Path::new(&args[1]);

    if !folder_to_hash.is_dir() {
        eprintln!("[Error] Folder {:?} not existing or not a folder.", folder_to_hash);
        return "".to_string();
    }

    cli_handler::print(&format!("Use Salt: \"{}\"", &args[2]), cli_handler::CliColor::Default);
    cli_handler::print("--- Start Calc Hash Code ---", cli_handler::CliColor::Default);

    // Calculating the folder hash with the specified text salt
    match hash_calc::calculate_folder_hash_with_salt(folder_to_hash, &args[2].as_bytes()) {
        Ok(total_hash) => {
            cli_handler::print("--- Calc Done ---", cli_handler::CliColor::Default);
            cli_handler::print(&format!("\n Folder: {:?} hash code is:", folder_to_hash), cli_handler::CliColor::Default);
            cli_handler::print(&total_hash, cli_handler::CliColor::Default);
            return total_hash;
        }
        Err(e) => {
            cli_handler::print(&format!("[Error] Calc Hash code Error!!!: {}", e), cli_handler::CliColor::Red);
        }
    }

    "".to_string()
}

/// Program function to validate the hash of a folder with a given salt.
/// 
/// # Arguments
/// - `args` - [tamperProtection <-v | --validate | validate> <hash_code> <folder_path> <salt>]
/// 
fn program_validate(args : &Vec<String>) {
    let hash_to_validate = &args[2];
    let vec_for_program_get_hash = vec!["".to_string(), args[3].clone(), args[4].clone()];
    let hast_folder =  program_get_hash(vec_for_program_get_hash.as_ref());

    if hash_to_validate == &hast_folder {
        cli_handler::print("\n[Success] Hash code match, folder is valid.", cli_handler::CliColor::Default);
    } else {
        cli_handler::print("\n[Error] Hash code not match, folder may be tampered.", cli_handler::CliColor::Default);
        cli_handler::print(&format!("  - Given Hash Code: {}", hash_to_validate), cli_handler::CliColor::Blue);
        cli_handler::print(&format!("  - Calc  Hash Code: {}", hast_folder), cli_handler::CliColor::Yellow);
    }
}

/// Main Entry Point
fn main() {
    let args: Vec<String> = env::args().collect();

    match cli_handler::args_handler(&args) {
        cli_handler::CliMode::Help => {
            cli_handler::help();
        }
        cli_handler::CliMode::GetHash => {
            program_get_hash(&args);
        }
        cli_handler::CliMode::Validate => {
            program_validate(&args);
        }
        cli_handler::CliMode::Error => {
            cli_handler::print("[Error] Invalid arguments!", cli_handler::CliColor::Red);
            cli_handler::help();
        }
    }
}