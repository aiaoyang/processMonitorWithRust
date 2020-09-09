use crate::error::*;
use crate::read::read_file_line_column;

// 初始化系统总内存大小
lazy_static! {
    static ref MEM_TOTAL: f64 = system_mem();
}

// 系统总内存
pub fn system_mem() -> f64 {
    read_file_line_column("/proc/meminfo".to_string(), 0, 1).unwrap()
}

// 系统空闲内存
pub fn system_mem_free() -> Result<f64, MyError> {
    return Ok(read_file_line_column("/proc/meminfo".to_string(), 1, 1)?);
}

// 进程内存使用量
pub fn process_mem(pid: &str) -> Result<f64, MyError> {
    let file_name = format!("/proc/{}/statm", pid);
    return Ok(read_file_line_column(file_name, 0, 1)? / system_mem());
}

// 进程内存使用率
pub fn process_mem_usage(pid: &str) -> f64 {
    process_mem(pid).unwrap()
}
pub fn system_mem_usage() -> f64 {
    (*MEM_TOTAL - system_mem_free().unwrap()) / *MEM_TOTAL
}
