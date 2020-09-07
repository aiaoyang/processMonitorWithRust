use std::error::Error;
use std::{io, num};
#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Parse(num::ParseFloatError),
}

impl From<num::ParseFloatError> for MyError {
    fn from(err: num::ParseFloatError) -> MyError {
        MyError::Parse(err)
    }
}
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

pub fn total_mem() -> Result<f64, MyError> {
    return Ok(read_from_file("/proc/meminfo".to_string(), 0, 1)?);
}

pub fn mem_usage(pid: &str) -> Result<f64, MyError> {
    let file_name = format!("/proc/{}/statm", pid);
    return Ok(read_from_file(file_name, 0, 1)? / total_mem()?);
}

pub fn read_from_file(file: String, line: usize, position: usize) -> Result<f64, MyError> {
    let content = std::fs::read(file)?;
    let num = String::from_utf8_lossy(&content)
        .lines()
        .collect::<Vec<&str>>()
        .get(line)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(position)
        .unwrap()
        .parse::<f64>()?;

    Ok(num)
}
