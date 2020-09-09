use crate::error as iError;
use std::time::Duration;
// HZ 系统时钟
// const HZ: i32 = 100;

lazy_static! {
        static ref DURATION_NUMBER:usize=300;
        static ref CORE_NUM: f64 = core_num() as f64;// CPU核数
                static ref DURATION: Duration = Duration::from_millis(300);// cpu采集间隔

    static ref CPU_TOTAL: f64 = *CORE_NUM * (*DURATION).as_secs_f64();// 采集间隔内的 CPU总时间
}

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

/// 进程cpu使用率 需要自定义采集间隔时间
pub fn process_cpu_usage(pid: &str) -> f64 {
    let c1 = process_cpu_count(pid).unwrap();

    // let t1 = std::time::SystemTime::now();

    std::thread::sleep(*DURATION);

    let c2 = process_cpu_count(pid).unwrap();

    // let duration = std::time::SystemTime::now()
    //     .duration_since(t1)
    //     .unwrap()
    //     .as_secs_f64();

    (c2 - c1) / *CPU_TOTAL
}

// 进程cpu使用率快照
pub fn process_cpu_count(pid: &str) -> Result<f64, iError::MyError> {
    use crate::read;

    let file = format!("/proc/{}/stat", pid);

    let line = read::read_file_line(file, 0)?;

    let column = line.split_whitespace().into_iter().collect::<Vec<&str>>();

    let mut count: f64 = 0.0;

    for v in 13..=16 {
        // 如果 slice.get(index)存在，则将该数据转换成f64格式
        if let Some(tmp) = column.get(v).and_then(|column| {
            // 如果转换成f64格式成功，则返回转换后的数据
            if let Ok(value) = column.parse::<f64>() {
                Some(value)
            } else {
                panic!("error, some column does not exist");
            }
        }) {
            count += tmp;
        }
    }

    // println!("here  count -> {:?}", count);
    Ok(count)
}

// 系统cpu使用率
pub fn system_cpu_usage() -> f64 {
    let pre = cpu_stat_file_to_struct().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(300));
    let now = cpu_stat_file_to_struct().unwrap();
    let delta = now - pre;
    // println!("delta : {:?}", delta);
    delta.used as f64 / delta.total as f64
}

// 从文件读取cpu状态
fn cpu_stat_file_to_struct() -> Result<SysCPU, iError::MyError> {
    let content = std::fs::read_to_string("/proc/stat")?;

    let mut slice = content.lines().next().unwrap().split_whitespace();
    // 跳过 首列 cpu 字段
    // cpu  352783 915 125099 70973099 25194 0 6192 0 0 0
    slice.next();

    let user = slice.next().unwrap().parse::<usize>()?;
    let nice = slice.next().unwrap().parse::<usize>()?;
    let system = slice.next().unwrap().parse::<usize>()?;
    let idle = slice.next().unwrap().parse::<usize>()?;
    let iowait = slice.next().unwrap().parse::<usize>()?;
    let irq = slice.next().unwrap().parse::<usize>()?;
    let softirq = slice.next().unwrap().parse::<usize>()?;
    let steal = slice.next().unwrap().parse::<usize>()?;
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

pub fn core_num() -> usize {
    let content = std::fs::read_to_string("/proc/cpuinfo").unwrap();
    content
        .lines()
        .filter(|line| line.contains("processor"))
        .into_iter()
        .collect::<Vec<_>>()
        .len()
}

mod test {
    // use crate::cpu::process_cpu_count;
    #[cfg(test)]
    fn test() {
        print!("{:?}", process_cpu_count("1"));
    }
}
