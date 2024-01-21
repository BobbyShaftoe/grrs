use clap::Parser;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

/// Main function to summarize a directory and its members
fn main() {
    let args = Cli::parse();
    let path = Path::new(&args.path);

    println!("\nDirectory to summarize: {}\n", path.display());

    // Get directory members by calling function that returns a Result with a vector of DirEntry
    let dir_members = match get_dir_members(&path) {
        Ok(dir_members) => dir_members,
        Err(error) => panic!("Problem opening directory: {:?}", error),
    };

    // Create hashmap for filename and attributes
    let mut file_attribs_map = HashMap::new();

    // Add filename and attributes to hashmap
    for member in &dir_members {
        let file_path = match normalize_path(member.path().as_path()) {
            Ok(file_path) => file_path,
            Err(error) => panic!("Problem normalizing path: {:?}", error),
        };

        let file_size = match get_file_size(&member.path()) {
            Ok(file_size) => file_size,
            Err(error) => panic!("Problem getting file size: {:?}", error),
        };

        file_attribs_map.insert(file_path, file_size);
    }

    // Print filename and attributes from hashmap
    for (file_name, file_size) in file_attribs_map {
        println!("{}: {} bytes", file_name.display(), file_size);
    }

    // Print total number of items in directory
    println!("\nTotal items: {}", dir_members.len());
}

/// Get file size in bytes for a given path and return a Result with a u64
fn get_file_size(path: &PathBuf) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Get directory members for a given path and return a Result with a vector of DirEntry
fn get_dir_members(path: &Path) -> io::Result<Vec<DirEntry>> {
    let mut members = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        members.push(entry);
    }
    Ok(members)
}

/// Normalize a path and return a Result with a PathBuf
fn normalize_path(path: &Path) -> io::Result<PathBuf> {
    let path_components: Vec<_> = path.components().map(|c| c.as_os_str()).collect();

    // Join the components back into a path
    let normalized_path = path_components.join("/".as_ref());
    return Ok(PathBuf::from(normalized_path));
}
