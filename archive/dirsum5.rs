use clap::Parser;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tabled::{settings::Style, Table, Tabled};
// use winapi::um::minwinbase::SYSTEMTIME;
use chrono::format::Fixed::TimezoneOffsetZ;
use chrono::*;

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
    let args: Cli = Cli::parse();
    let path: &Path = Path::new(&args.path);

    println!("\nDirectory to summarize: {}\n", path.display());

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
        match get_all_file_attributes(&member.path(), &file_path) {
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

    // let date_1601 = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0);
    let date_1601 = NaiveDate::from_ymd_opt(1601, 1, 1).unwrap();
    println!("Date 1601: {:#?}", date_1601);

    let date_unix_epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    println!("Date 1970: {:#?}", date_unix_epoch);

    let duration_1601_1970 = date_unix_epoch.signed_duration_since(date_1601);
    println!("Duration between 1601 and 1970: {:#?}", duration_1601_1970);

    let duration_1601_1970_seconds = duration_1601_1970.num_seconds();
    println!(
        "Duration between 1601 and 1970 in seconds: {:#?}",
        duration_1601_1970_seconds
    );

    let file_years_since_1970 = duration_1601_1970.num_days() / 365;
    println!("File years since 1970: {:#?}", file_years_since_1970);

    let file_time_since_1601_as_sec = 1042307722094 / 10000000 + duration_1601_1970_seconds;
    println!(
        "File time since 1601 as seconds: {:#?}",
        file_time_since_1601_as_sec / 60 / 60 / 24 / 365
    );

    let file_time_as_date = Utc
        .timestamp_opt(file_time_since_1601_as_sec, 0)
        .unwrap()
        .to_string();
    println!("File time: {:#?}", file_time_as_date);

    let unix_epoch = SystemTime::UNIX_EPOCH;
    println!("Unix epoch: {:#?}", unix_epoch);


}

/// Get all file attributes for a given path and return a Result with a FileAttribs struct
fn get_all_file_attributes(path: &PathBuf, file_path: &Path) -> io::Result<FileAttribs> {
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
        size: file_size,
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
