use crate::error::*;
use crate::read::read_file_line_column;

pub fn total_mem() -> Result<f64, MyError> {
    return Ok(read_file_line_column("/proc/meminfo".to_string(), 0, 1)?);
}

pub fn mem_usage(pid: &str) -> Result<f64, MyError> {
    let file_name = format!("/proc/{}/statm", pid);
    return Ok(read_file_line_column(file_name, 0, 1)? / total_mem()?);
}
