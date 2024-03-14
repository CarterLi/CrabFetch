use crate::{config_manager::Configuration, memory::MemoryInfo, cpu::CPUInfo};

mod cpu;
mod memory;
mod config_manager;

trait Fetchable {
    fn new() -> Self;
    fn format(&self, format: &str) -> String;
}

fn main() {
    let config: Configuration = config_manager::parse();
    let cpu: CPUInfo = cpu::get_cpu();
    let memory: MemoryInfo = memory::get_memory();

    if config.enable_cpu {
        let mut str = config.cpu_title;
        str.push_str(&config.seperator);
        str.push_str(&cpu.format(&config.cpu_format));
        println!("{}", str);
    }
    if config.enable_memory {
        let mut str = config.memory_title;
        str.push_str(&config.seperator);
        str.push_str(&memory.format(&config.memory_format));
        println!("{}", str);
    }
}
