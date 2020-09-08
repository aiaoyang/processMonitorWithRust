mod cpu;
mod error;
mod mem;
mod read;

use cpu::*;
use mem::*;

fn main() {
    let pids = std::env::args().collect::<Vec<String>>();
    let pid = pids.get(1).unwrap().as_str();

    println!("pid is : {}", &pid);
    for _ in 0..10 {
        let process_usage = process_cpu_usage(pid);

        let system_cpu_usage = system_cpu_usage();

        let process_mem_usage = process_mem_usage(pid);

        let system_mem_usage = system_mem_usage();

        println!("process_cpu_usage\t->\t{:?}%\nsystem_cpu_usage\t->\t{}%\nprocess_mem_usage\t->\t{}%\nsystem_mem_usage\t->\t{}%", 
					process_usage,
					system_cpu_usage,
					process_mem_usage,
					system_mem_usage,
				);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
