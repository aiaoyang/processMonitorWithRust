mod cpu;
mod error;
mod mem;
mod read;

use cpu::*;
use mem::*;

// }
fn main() {
    cpu::core_num().unwrap();
    print!("{:?}", process_cpu_count("1"));
}
