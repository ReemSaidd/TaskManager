use sysinfo;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(PartialEq, Clone)]
pub struct Process {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
    pub start_time: u64,
    pub parent: Option<sysinfo::Pid> 
}

pub fn format(&self) -> Vec<String> {
    let parent_string = match self.parent {
        Some(pid) => pid.to_string(),
        None => String::from("N/A")
    };
    let systemtime: u64;


    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    unsafe{systemtime = SYSTEM_START_TIME;}
    let time = now - systemtime + self.time;
    let process_duration = format_time(time);
    
    fn format_time(seconds: u64) -> String {
        let seconds = seconds as u64;
        let hours = seconds / 3600;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    let systemtime: u64;

    unsafe{systemtime = SYSTEM_START_TIME;}

    vec![
        self.pid.to_string(),
        self.name.clone(),
        format!("{:.2}%", self.cpu),
        pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
        process_duration,
        parent_string
    ]

    
}


