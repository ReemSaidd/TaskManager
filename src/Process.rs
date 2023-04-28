use sysinfo;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::system::SYSTEM_START_TIME;


use sysinfo::ProcessExt;
use sysinfo::NetworkExt;
use sysinfo::NetworkData;

#[derive(PartialEq, Clone)]
pub struct Process {
    pub pid: i32,
    pub name: String,
    //pub status:ProcessStatus,
    pub cpu: f32,
    pub mem: u64,
    pub start_time: u64,
    pub elapsed_time: u64,
    pub parent: Option<sysinfo::Pid> ,
    //pub network: u64
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
                parent: process.parent(),
                //network:process.network()
            }
        }

        pub fn pstableformat(& self) -> Vec<String> {
            let parent_string = match self.parent {
                Some(pid) => pid.to_string(),
                None => String::from("N/A")
            };

            vec![
                self.pid.to_string(),
                self.name.clone(),
                Process::format_time(self.start_time),
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

        pub fn format(& self) -> Vec<String> {
            let parent_string = match self.parent {
                Some(pid) => pid.to_string(),
                None => String::from("N/A")
            };

            let systemtime: u64;


            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            unsafe{systemtime = SYSTEM_START_TIME;}
            //self.elapsed_time = now - systemtime + self.start_time;
            let process_duration = Process::format_time(now - systemtime + self.start_time);
            //let process_duration = format_time(self.elapsed_time);
            



            vec![
                self.pid.to_string(),
                self.name.clone(),
                //self.status,
                format!("{:.2}%", self.cpu),
                pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
                Process::format_time(self.start_time),
                parent_string
            ]

        }
}


