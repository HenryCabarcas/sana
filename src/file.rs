use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

pub fn read_file(filename: &String) -> Result<&String, Error> {
    let path = Path::new(filename.as_str());
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    let lines = lines?; // Handle the Result<Vec<String>, io::Error>
    return Ok(&lines.join("\n"));
}

pub fn read_file_lines(filename: &String) -> Result<&Vec<String>, Error> {
    let path = Path::new(filename.as_str());
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    let lines = lines?; // Handle the Result<Vec<String>, io::Error>
    return Ok(&lines);
}
