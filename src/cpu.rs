use std::time::Duration;

// HZ 系统时钟
// const HZ: i32 = 100;

#[derive(Debug)]
pub struct SysCPU {
    user: usize,
    nice: usize,
    system: usize,
    idle: usize,
    iowait: usize,
    irq: usize,
    softirq: usize,
    steal: usize,
    used: usize,
    total: usize,
}

impl std::ops::Sub<SysCPU> for SysCPU {
    type Output = SysCPU;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        return SysCPU {
            user: self.user - rhs.user,
            nice: self.nice - rhs.nice,
            system: self.system - rhs.system,
            idle: self.idle - rhs.idle,
            iowait: self.iowait - rhs.iowait,
            irq: self.irq - rhs.irq,
            softirq: self.softirq - rhs.softirq,
            steal: self.steal - rhs.steal,
            used: self.used - rhs.used,
            total: self.total - rhs.total,
        };
    }
}
impl SysCPU {
    fn new() -> Self {
        return SysCPU {
            user: 0,
            nice: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            irq: 0,
            softirq: 0,
            steal: 0,
            used: 0,
            total: 0,
        };
    }
}
// 进程cpu使用率
pub fn process_cpu_usage_with_duration(pid: &str, duration: Duration) -> f64 {
    let c1 = process_cpu_count(pid).unwrap_or(0);

    let t1 = std::time::SystemTime::now();

    std::thread::sleep(duration);

    let c2 = process_cpu_count(pid).unwrap_or(0);

    let duration = std::time::SystemTime::now()
        .duration_since(t1)
        .unwrap()
        .as_secs_f64();

    println!("time to f64: {}", duration);
    ((c2 as f64) - (c1 as f64)) / (duration * core_num() as f64)
}
//
pub fn process_cpu_usage(pid: &str) -> f64 {
    process_cpu_usage_with_duration(pid, Duration::from_millis(300))
}

// 进程快照
fn process_cpu_count(pid: &str) -> Result<usize, std::io::Error> {
    let fname = String::from("/proc/") + pid + "/stat";
    std::fs::read(&fname).map(|f| {
        String::from_utf8(f)
            .map(|content| {
                content
                    .lines()
                    .next()
                    .map(|first_line| {
                        let iter = first_line.split_whitespace().collect::<Vec<&str>>();

                        let mut tmp: usize = 0;

                        for v in vec![14, 15, 16, 17] {
                            tmp += iter.get(v).unwrap().parse::<usize>().unwrap_or(0);
                        }
                        tmp
                    })
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    })
    // .unwrap()
}

// 总cpu使用率
pub fn total_cpu_usage() -> f64 {
    let pre = cpu_stat_file_to_struct().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(300));
    let now = cpu_stat_file_to_struct().unwrap();
    let delta = now - pre;
    println!("delta : {:?}", delta);
    delta.used as f64 / delta.total as f64
}

// 从文件读取cpu状态
fn cpu_stat_file_to_struct() -> Result<SysCPU, ()> {
    let f = std::fs::read("/proc/stat").unwrap_or(Vec::new());
    let content = String::from_utf8(f).unwrap();
    let mut slice = content.lines().next().unwrap().split_whitespace();
    slice.next();

    let user = slice.next().unwrap().parse::<usize>().unwrap();
    let nice = slice.next().unwrap().parse::<usize>().unwrap();
    let system = slice.next().unwrap().parse::<usize>().unwrap();
    let idle = slice.next().unwrap().parse::<usize>().unwrap();
    let iowait = slice.next().unwrap().parse::<usize>().unwrap();
    let irq = slice.next().unwrap().parse::<usize>().unwrap();
    let softirq = slice.next().unwrap().parse::<usize>().unwrap();
    let steal = slice.next().unwrap().parse::<usize>().unwrap();
    let total = user + nice + system + idle + iowait + irq + softirq + steal;
    let used = total - idle;

    let mut c = SysCPU::new();

    c.user = user;
    c.nice = nice;
    c.system = system;
    c.idle = idle;
    c.iowait = iowait;
    c.irq = irq;
    c.softirq = softirq;
    c.steal = steal;
    c.used = used;
    c.total = total;

    // println!("c : {:?}", &c);
    Ok(c)
}

pub fn core_num() -> i8 {
    let lines = std::fs::read("/proc/cpuinfo")
        .map(|vec_content| String::from_utf8(vec_content))
        .unwrap()
        .unwrap();
    let mut num = 0;
    for line in lines.lines() {
        if line.contains("processor") {
            num += 1;
        }
    }
    println!("num of core : {}", num);
    num
}
