use std::fs::read_to_string;
use std::io::Error as StdIOError;

#[derive(Debug)]
pub enum Error {
    General(String),
    IOError(StdIOError)
}

pub fn read_lines(input_path: &str) -> Result<Vec<String>, Error> {
    match read_to_string(input_path) {
        Err(e) => Err(Error::IOError(e)),
        Ok(s) => {
            Ok(s.replace("\r", "")
                .split("\n")
                .into_iter()
                .map(|s| s.to_string())
                .collect()
            )
        }
    }
}

pub fn read_non_empty_lines(input_path: &str) -> Result<Vec<String>, Error> {
    let mut result: Vec<String> = Vec::new();
    for line in read_lines(input_path)? {
        if line.len() > 0 {
            result.push(line);
        }
    }
    Ok(result)
}  