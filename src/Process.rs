use sysinfo::{ProcessExt};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(PartialEq, Clone)]
pub struct Process {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
    pub start_time: u64,
    pub elapsed_time: u64,
    pub parent: Option<sysinfo::Pid> 
}

impl Process {
    pub fn new(process: &sysinfo::Process) -> Process {
        Process {
            pid: process.pid(),
            name: process.name().to_string(),
            cpu: process.cpu_usage(),
            mem: process.memory(),
            start_time: process.start_time(),
            elapsed_time: 0,
            parent: process.parent()
        }
    }

    pub fn format(&self) -> Vec<String> {


        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format_time(self.elapsed_time),
            parent_string
        ]
    }

    fn format_time(seconds: u64) -> String {
        let seconds = seconds as u64;
        let hours = seconds / 3600;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}


