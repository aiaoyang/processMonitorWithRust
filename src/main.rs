// #![feature(const_fn)]
mod cpu;
mod error;
mod mem;
mod read;

use cpu::*;
use mem::*;

// }
fn main() {
    let pids = std::env::args().collect::<Vec<String>>();

    for _ in 0..10 {
        let usage = process_cpu_usage(pids.get(1).unwrap().as_str());
        println!("process cpu usage : {:?}", usage);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
