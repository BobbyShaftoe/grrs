use clap::Parser;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io;
use std::io::Bytes;
use std::path::{Path, PathBuf};
use clap::error::ContextValue::String;
use tabled::{Tabled, Table, settings::Style};

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

#[derive(Tabled)]
struct FileAttribs {
    filename: &'static str,
    file_type: &'static str,
    size: u64,
    creation_time: &'static str,
    modified_time: &'static str,
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
    let mut file_attribs_vec = Vec::new();



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

        let file_attribs = match get_all_file_attributes(&member.path()) {
            Ok(file_attribs) => file_attribs,
            Err(error) => panic!("Problem getting file attributes: {:?}", error),
        };

        file_attribs_map.insert(file_path, file_size);
        file_attribs_vec.push(file_attribs);

    }

    // Print filename and attributes from hashmap
    // for (file_name, file_size) in file_attribs_map {
    //     println!("{}: {} bytes", file_name.display(), file_size);
    // }

    let mut table = Table::new(&file_attribs_vec);
    table.with(Style::psql());
    println!("{}", table);

    // Print total number of items in directory
    println!("\nTotal items: {}", dir_members.len());
}

/// Get file size in bytes for a given path and return a Result with a u64
fn get_file_size(path: &PathBuf) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn get_all_file_attributes(path: &PathBuf) -> io::Result<FileAttribs> {
    let metadata = fs::metadata(&path)?;
    let m = &path.clone();
    let file_type = match metadata.file_type().is_dir() {
        true => "Directory",
        false => "File",
    };
    let file_size = metadata.len();
    let creation_time = metadata.created()?;
    let modified_time = metadata.modified()?;

    let file_attribs = FileAttribs {
        filename: "aaa",
        file_type: file_type,
        size: file_size,
        creation_time: "aaa",
        modified_time: "aaa",
    };

    Ok(file_attribs)
}

// fn get_all_file_attributes(path: &'static PathBuf) -> io::Result<FileAttribs> {
//     let new_path = &path.to_str().unwrap();
//     let metadata = fs::metadata(&new_path)?;
//     let file_type = match metadata.file_type().is_dir() {
//         true => "Directory",
//         false => "File",
//     };
//     let file_size = metadata.len();
//     let creation_time = metadata.created()?;
//     let modified_time = metadata.modified()?;
//
//     let file_attribs = FileAttribs {
//         filename: &new_path,
//         file_type: file_type,
//         size: &file_size,
//         creation_time: format!("{:?}", creation_time).as_str(),
//         modified_time: format!("{:?}", modified_time).as_str(),
//     };
//
//     Ok(file_attribs)
// }

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
