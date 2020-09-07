use std::time::Duration;

mod cpu;
mod mem;

use cpu::*;
use mem::*;

// }
fn main() {
    println!(
        "process: {}\ntotal {}\n",
        process_cpu_usage("423"),
        total_cpu_usage(),
    );
    // println!("c: {:?}", res);

    println!(
        "total : {:?}\ncurrent : {:?}",
        total_mem().unwrap(),
        mem_usage("1").unwrap()
    );
}
