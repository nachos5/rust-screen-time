use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use csv;
use serde::{Deserialize, Serialize};

use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    class: String,
    name: String,
    time: u64,
}

const FILENAME: &str = "info.csv";

// gets the csv filepath - if it doesn't exist then a new file is created
pub fn get_csv_filepath() -> PathBuf {
    let proj_dirs =
        ProjectDirs::from("", "nachos5", "rust screen time").expect("could not get project dirs");
    let cache_dir = proj_dirs.cache_dir();
    // dbg!(cache_dir);
    if !cache_dir.join(FILENAME).exists() {
        fs::create_dir_all(cache_dir).expect("failed to create directory");
        fs::File::create(cache_dir.join(FILENAME)).expect("failed to create file");
    }
    cache_dir.join(FILENAME)
}

pub fn get_csv_file_contents() -> String {
    let filepath = get_csv_filepath();
    fs::read_to_string(filepath).expect("could not read csv file")
}

// deserializes the csv file and returns a Record struct vector
pub fn get_records() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut records = vec![];
    let file_contents = get_csv_file_contents();
    let mut reader = csv::Reader::from_reader(file_contents.as_bytes());
    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    Ok(records)
}
