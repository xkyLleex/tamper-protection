use std::{
    fs::File, io::{self, Read}, path::Path
};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

/// Calculate the hash of a file.
/// 
/// # Arguments
/// - `path` - The path to the file to hash.
/// 
/// # Returns
/// A `Result` containing the hash bytes or an `io::Error`.
/// 
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
pub fn calculate_folder_hash_with_salt(folder_path: &Path, salt: &[u8]) -> Result<String, io::Error> {
    let mut total_hasher = Sha256::new();

    // Include the salt in the hash
    total_hasher.update(salt);

    // Recursively walk through the directory
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            // Calculate the file hash
            let file_hash = calculate_file_hash(path)?;
            // Update the total hasher with the file and its hash
            total_hasher.update(path.file_name().unwrap().to_string_lossy().as_bytes());
            total_hasher.update(&file_hash);
        }
    }

    // Finalize and return the hex representation of the hash
    Ok(format!("{:x}", total_hasher.finalize()))
}