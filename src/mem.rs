use crate::error::*;
use crate::read::read_file_line_column;

pub fn system_mem() -> Result<f64, MyError> {
    return Ok(read_file_line_column("/proc/meminfo".to_string(), 0, 1)?);
}

pub fn system_mem_used() -> Result<f64, MyError> {
    return Ok(read_file_line_column("/proc/meminfo".to_string(), 1, 1)?);
}

pub fn process_mem(pid: &str) -> Result<f64, MyError> {
    let file_name = format!("/proc/{}/statm", pid);
    return Ok(read_file_line_column(file_name, 0, 1)? / system_mem()?);
}

pub fn process_mem_usage(pid: &str) -> f64 {
    process_mem(pid).unwrap()
}
pub fn system_mem_usage() -> f64 {
    (system_mem().unwrap() - system_mem_used().unwrap()) / system_mem().unwrap()
}
