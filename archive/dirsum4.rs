use clap::Parser;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tabled::{settings::Style, Table, Tabled};

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

#[derive(Debug, Tabled)]
struct FileAttribs {
    filename: String,
    file_type: String,
    size: u64,
    creation_time: String,
    modified_time: String,
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

    // Vector that will hold file attributes structs
    let mut file_attribs_vec = Vec::new();

    for member in &dir_members {
        // Get file attributes for each member of the directory
        let file_path = match normalize_path(member.path().as_path()) {
            Ok(file_path) => file_path,
            Err(error) => panic!("Problem normalizing path: {:?}", error),
        };

        // Get attributes for each file as struct and add to vector
        let file_attribs = match get_all_file_attributes(&member.path(), &file_path) {
            Ok(file_attribs) => file_attribs_vec.push(file_attribs),
            Err(error) => panic!("Problem getting file attributes: {:?}", error),
        };
    }

    // Creation of table from vector of structs
    let mut table = Table::new(&file_attribs_vec);
    table.with(Style::psql());
    println!("{}", table);

    // Total number of items within the directory
    println!("\nTotal items: {}", dir_members.len());
}

/// Get file size in bytes for a given path and return a Result with a u64
fn get_file_size(path: &PathBuf) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Get all file attributes for a given path and return a Result with a FileAttribs struct
fn get_all_file_attributes(path: &PathBuf, file_path: &Path) -> io::Result<FileAttribs> {
    let metadata = fs::metadata(&path)?;

    let file_type = match metadata.file_type().is_dir() {
        true => "Directory",
        false => "File",
    };

    let file_size = metadata.len();
    let creation_time: SystemTime = metadata.created()?;
    let modified_time: SystemTime = metadata.modified()?;

    let file_attribs = FileAttribs {
        filename: file_path.to_str().unwrap().to_string(),
        file_type: file_type.to_string(),
        size: file_size,
        creation_time: creation_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
        modified_time: "1042307722094".to_string(),
    };

    Ok(file_attribs)
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
    // Get the components of the path and convert them to OsStr and collect them into a vector
    let path_components: Vec<_> = path.components().map(|c| c.as_os_str()).collect();

    // Join the components back into a path
    let normalized_path = path_components.join("/".as_ref());
    Ok(PathBuf::from(normalized_path))
}
