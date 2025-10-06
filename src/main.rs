use std::{
    env, fs::File, io::{self, Read}, path::Path
};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;

/// Calculate the hash of a folder with a given salt.
/// 
/// # Arguments
/// - `folder_path` - The path to the folder to hash.
/// - `salt` - The salt to use in the hash calculation.
/// 
/// # Returns
/// A `Result` containing the hexadecimal string of the hash or an `io::Error`.
/// 
/// # Example
/// ```
/// let folder_hash = calculate_folder_hash_with_salt(Path::new("my_folder"), b"my_salt").unwrap();
/// println!("Folder hash: {}", folder_hash);
/// ```
/// 
fn calculate_folder_hash_with_salt(folder_path: &Path, salt: &[u8]) -> Result<String, io::Error> {
    let mut total_hasher = Sha256::new();

    total_hasher.update(salt);

    // Start walking through the directory, only hasking files, not folders
    for entry in WalkDir::new(folder_path).sort_by_file_name().into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // Skip entries that cannot be read
        };

        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            if let Ok(relative_path) = path.strip_prefix(folder_path) {
                total_hasher.update(relative_path.to_string_lossy().as_bytes());
            }
            total_hasher.update(salt);
            let file_hash = calculate_file_hash(path)?;
            total_hasher.update(&file_hash);

            println!("[Hash] File: {:?} -> {}", path, hex::encode(&file_hash));
        }
    }

    let final_hash = total_hasher.finalize();
    Ok(hex::encode(final_hash))
}


fn calculate_file_hash(path: &Path) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 65536]; // 64KB buffer

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize().to_vec())
}

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

    println!("Use Salt: \"{}\"", &args[2]);
    println!("--- Start Calc Hash Code ---");

    // Calculating the folder hash with the specified text salt
    match calculate_folder_hash_with_salt(folder_to_hash, &args[2].as_bytes()) {
        Ok(total_hash) => {
            println!("--- Calc Done ---");
            println!("\n Folder: {:?} hash code is:", folder_to_hash);
            println!("{}", total_hash);
            return total_hash;
        }
        Err(e) => {
            eprintln!("[Error] Calc Hash code Error!!!: {}", e);
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
        println!("\n[Success] Hash code match, folder is valid.");
    } else {
        println!("\n[Error] Hash code not match, folder may be tampered.");
        println!("  - Given Hash Code: {}", hash_to_validate);
        println!("  - Calc  Hash Code: {}", hast_folder);
    }

}

fn program_help() {
    println!("Tamper Protection - Folder Hash Code Calculator");
    println!("- Usage:");
    println!("  - tamperProtection <folder_path> <salt>");
    println!("  - tamperProtection <-h | --help | help>");
    println!("  - tamperProtection <-v | --validate | validate> <hash_code> <folder_path> <salt>");
    println!();
    println!("- Example:");
    println!("  - tamperProtection ./my_folder my_salt");
    println!("  - tamperProtection help");
    println!("  - tamperProtection validate XXX ./my_folder my_salt");
    println!();
}

/// Main Entry Point
fn main() {
    let args_count = env::args().len();
    let args: Vec<String> = env::args().collect();

    if args_count == 2 {
        if &args[1] == "-h" || &args[1] == "--help" || &args[1] == "help" {
            program_help();
        }
    } else if args_count == 3 {
        program_get_hash(&args);
    } else if args_count == 5 {
        if &args[1] != "-v" && &args[1] != "--validate" && &args[1] != "validate" {
            eprintln!("[Error] This program arguments error!!!");
            program_help();
            return;
        }
        program_validate(&args);
    } else {
        eprintln!("[Error] This program arguments error!!!");
        program_help();
    }
}