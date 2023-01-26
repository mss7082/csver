use csv;
use serde::Deserialize;
use std::error::Error;
use std::{io, process};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Users {
    username: String,
    identifier: String,
    #[serde(rename = "First name")]
    first_name: String,
    #[serde(rename = "Last name")]
    last_name: String,
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut records: Vec<Users> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;
    for result in rdr.deserialize() {
        let record: Users = result?;
        records.push(record);
    }

    println!("{:#?}", records);
    Ok(())
}

fn main() {
    if let Err(e) = run("./uspop.csv") {
        // if let Err(e) = read_from_file("./username.csv") {
        eprintln!("{}", e);
        process::exit(1);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let mut records: Vec<Record> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        records.push(record);
    }

    println!("{:#?}", records);
    Ok(())
}
