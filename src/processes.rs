use std::fs::{read_dir, ReadDir};

use serde::Deserialize;

use crate::{formatter::CrabFetchColor, config_manager::Configuration, Module, ModuleError};

pub struct ProcessesInfo {
    count: u32 // god forbid someone manages to hit this limit
}
#[derive(Deserialize)]
pub struct ProcessesConfiguration {
    pub title: String,
    pub title_color: Option<CrabFetchColor>,
    pub title_bold: Option<bool>,
    pub title_italic: Option<bool>,
    pub seperator: Option<String>,
    pub format: Option<String>,
}
impl Module for ProcessesInfo {
    fn new() -> ProcessesInfo {
        ProcessesInfo {
            count: 0
        }
    }

    fn style(&self, config: &Configuration, max_title_size: u64) -> String {
        let title_color: &CrabFetchColor = config.processes.title_color.as_ref().unwrap_or(&config.title_color);
        let title_bold: bool = config.processes.title_bold.unwrap_or(config.title_bold);
        let title_italic: bool = config.processes.title_italic.unwrap_or(config.title_italic);
        let seperator: &str = config.processes.seperator.as_ref().unwrap_or(&config.seperator);

        let value: String = self.replace_color_placeholders(&self.replace_placeholders(config));

        Self::default_style(config, max_title_size, &config.processes.title, title_color, title_bold, title_italic, seperator, &value)
    }
    fn unknown_output(config: &Configuration, max_title_size: u64) -> String { 
        let title_color: &CrabFetchColor = config.processes.title_color.as_ref().unwrap_or(&config.title_color);
        let title_bold: bool = config.processes.title_bold.unwrap_or(config.title_bold);
        let title_italic: bool = config.processes.title_italic.unwrap_or(config.title_italic);
        let seperator: &str = config.processes.seperator.as_ref().unwrap_or(&config.seperator);

        Self::default_style(config, max_title_size, &config.processes.title, title_color, title_bold, title_italic, seperator, "Unknown")
    }

    fn replace_placeholders(&self, config: &Configuration) -> String {
        let format: String = config.processes.format.clone().unwrap_or("{count}".to_string());
        format.replace("{count}", &self.count.to_string())
    }
}

pub fn get_process_count() -> Result<ProcessesInfo, ModuleError> {
    let mut process_info: ProcessesInfo = ProcessesInfo::new();

    // Scans /proc and simply checks if it's a number 
    let dir: ReadDir = match read_dir("/proc") {
        Ok(r) => r,
        Err(e) => return Err(ModuleError::new("Processes", format!("Failed to read /proc: {}", e)))
    };

    for x in dir {
        let x = x.unwrap().file_name().into_string().unwrap();
        if x.parse::<u64>().is_ok() {
            process_info.count += 1;
        }
    }

    Ok(process_info)
}
