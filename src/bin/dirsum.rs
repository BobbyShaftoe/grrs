use chrono::prelude::*;
use clap::{Parser, ValueEnum};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tabled::{settings::Style, Table, Tabled};

#[derive(Parser, Debug)]
#[command(name = "dirsum", version = "0.1.0", about = "A simple to use directory summarizer", long_about = None)]
struct Cli {
    /// Path to directory to summarize
    #[arg(value_name = "DIRECTORY")]
    path: PathBuf,
    /// Size format to use for file and directory sizes
    #[arg(value_enum, long, value_name = "SIZE UNIT")]
    size: Option<FileSizeFormat>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
enum FileSizeFormat {
    /// "Bytes"
    B,
    /// "KiloBytes"
    KB,
    /// "Megabytes"
    MB,
    /// "Gigabytes"
    GB,
}

#[derive(Debug, Tabled)]
struct FileAttribs {
    filename: String,
    file_type: String,
    size: f32,
    creation_time: String,
    modified_time: String,
}

/// Main function to summarize a directory and its members
fn main() {
    let args: Cli = Cli::parse();
    let path: &Path = Path::new(&args.path);

    let mut file_format_map: HashMap<FileSizeFormat, String> = HashMap::new();
    file_format_map.insert(FileSizeFormat::B, "Bytes".to_string());
    file_format_map.insert(FileSizeFormat::KB, "KiloBytes".to_string());
    file_format_map.insert(FileSizeFormat::MB, "Megabytes".to_string());
    file_format_map.insert(FileSizeFormat::GB, "Gigabytes".to_string());

    println!("\nDirectory to summarize: {}", path.display());
    print!("Size format: ");
    match args.size {
        Some(size) => println!("{}\n", file_format_map.get(&size).unwrap()),
        None => println!("Bytes\n"),
    }

    let file_size_modifier: f32 = match args.size {
        Some(size_format) => get_file_size_modifier(size_format),
        None => 1.0,
    };

    // Get directory members by calling function that returns a Result with a vector of DirEntry
    let dir_members: Vec<DirEntry> = match get_dir_members(path) {
        Ok(dir_members) => dir_members,
        Err(error) => panic!("Problem opening directory: {:?}", error),
    };

    // Vector that will hold file attributes structs
    let mut file_attribs_vec: Vec<FileAttribs> = Vec::new();

    for member in &dir_members {
        // Get file attributes for each member of the directory
        let file_path: PathBuf = match normalize_path(member.path().as_path()) {
            Ok(file_path) => file_path,
            Err(error) => panic!("Problem normalizing path: {:?}", error),
        };

        // Get attributes for each file as struct and add to vector
        match get_all_file_attributes(&member.path(), &file_path, file_size_modifier) {
            Ok(FileAttribs {
                filename,
                file_type,
                size,
                creation_time,
                modified_time,
            }) => file_attribs_vec.push(FileAttribs {
                filename,
                file_type,
                size,
                creation_time,
                modified_time,
            }),
            Err(error) => panic!("Problem getting file attributes: {:?}", error),
        };
    }

    // Creation of table from vector of structs
    let mut table: Table = Table::new(&file_attribs_vec);
    table.with(Style::psql());
    println!("{}", table);

    // Total number of items within the directory
    println!("\nTotal items: {}", dir_members.len());
}

/// Get all file attributes for a given path and return a Result with a FileAttribs struct
fn get_all_file_attributes(
    path: &PathBuf,
    file_path: &Path,
    file_size_modifier: f32,
) -> io::Result<FileAttribs> {
    let metadata: Metadata = fs::metadata(path)?;

    let file_type: &str = match metadata.file_type().is_dir() {
        true => "Directory",
        false => "File",
    };

    let file_size: u64 = metadata.len();

    let creation_time = metadata
        .created()?
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let modified_time = metadata
        .modified()?
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let file_attribs = FileAttribs {
        filename: file_path.to_str().unwrap().to_string(),
        file_type: file_type.to_string(),
        size: format!("{:.2}", file_size as f32 / file_size_modifier)
            .parse()
            .unwrap(),
        creation_time: Local
            .timestamp_opt(creation_time as i64, 0)
            .unwrap()
            .to_string(),
        modified_time: Local
            .timestamp_opt(modified_time as i64, 0)
            .unwrap()
            .to_string(),
    };

    Ok(file_attribs)
}

/// Get directory members for a given path and return a Result with a vector of DirEntry
fn get_dir_members(path: &Path) -> io::Result<Vec<DirEntry>> {
    let mut members: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry: DirEntry = entry?;
        members.push(entry);
    }
    Ok(members)
}

/// Normalize a path and return a Result with a PathBuf
fn normalize_path(path: &Path) -> io::Result<PathBuf> {
    // Get the components of the path and convert them to OsStr and collect them into a vector
    let path_components: Vec<_> = path.components().map(|c| c.as_os_str()).collect();

    // Join the components back into a path
    let normalized_path: OsString = path_components.join("/".as_ref());
    Ok(PathBuf::from(normalized_path))
}

/// Get the file size modifier based on the FileSizeFormat enum
fn get_file_size_modifier(size_format: FileSizeFormat) -> f32 {
    match size_format {
        FileSizeFormat::B => 1.0,
        FileSizeFormat::KB => 1024.0,
        FileSizeFormat::MB => 1024.0 * 1024.0,
        FileSizeFormat::GB => 1024.0 * 1024.0 * 1024.0,
    }
}
