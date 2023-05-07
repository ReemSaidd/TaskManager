use std::process;
use clap::{Arg, App, SubCommand};
use libc::SIGKILL;
use libc::SIGTERM;
use libc::signal;
use psutil::Pid;
use psutil::network::NetConnection;
use sysinfo::{NetworkExt,RefreshKind,NetworkData, ProcessExt, SystemExt, Signal, System};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{ComponentExt};
use std::net::Ipv4Addr;
use std::process::Command;
//extern crate psutil;
//use psutil::{process::Process, Result};

//use std::time::{SystemTime, UNIX_EPOCH};

//use crate::SYSTEM_START_TIME;

//use std::process::Command;
//use procfs::{ProcResult, Process};
use std::io;
//extern crate libc;

use libc::{kill, pid_t};
use psutil::process::processes;
// use psutil::process::Process;
// use psutil::Result;
use psutil::{process::Process, Result};
use prettytable::{Table, Row, Cell};

//use command::{Command, CommandResult};


pub static mut SYSTEM_START_TIME: u64 = 0;


unsafe fn updatesystemstarttime() {
    SYSTEM_START_TIME = (SystemTime::now()).duration_since(UNIX_EPOCH).unwrap().as_secs();
}


pub fn format_time(seconds: u64) -> String {
    let seconds = seconds as u64;
   // let hours = seconds / 3600;
    let minutes = (seconds / 60) % 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}",minutes, seconds)
}

pub fn findbypid(pid: i32) -> Option<sysinfo::Process> {
    let system = System::new();
    for (pid_, process) in system.get_process_list().iter().map(|(pid, p)| (*pid, p)) {
        if pid_ == pid {
            return Some(process.clone());
        }
    }
    None}


 pub  fn sortasc() {
    let mut system = System::new();
   
    system.refresh_all();
  //  let mut system = System::new();
    let mut process_list: Vec<_> = system
    .get_process_list()
    .iter()
    .map(|(pid, process)| (*pid, process))
    .collect();

        process_list.sort_by_key(|&(_, process)| process.pid());
        let mut rows = vec![];
        for (pid, process) in process_list  {

            let systemtime: u64;
        
        
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                unsafe{systemtime = SYSTEM_START_TIME;}
              
                let process_duration = format_time(now - systemtime + process.start_time());
                let name = process.name();
                let mem = pretty_bytes::converter::convert((process.memory() as f64) * 1000.0);
                let cpu=process.cpu_usage();
                let status=process.status();
         
        
                if name.is_empty() {
                    continue;
                }
        
                for name_part in name.split_whitespace() {
                    rows.push(format!(" {:<8}  {:<100}  {:>10}  {:>20}%  {:>20}  {:<70} ", pid, name_part, mem, cpu, process_duration, status));
                }
            }
        
          
            println!("| {:<8} | {:<100} | {:<10} | {:<20} | {:<20} |{:<70}", "PID", "Name", "Memory", "cpu", "process_duration", "status");
          
        
            for row in rows {
                println!("{}", row);
            }
    }
    






  pub  fn sortdesc() {
    let mut system = System::new();
   
    system.refresh_all();
    //let mut system = System::new();
    let mut process_list: Vec<_> = system
    .get_process_list()
    .iter()
    .map(|(pid, process)| (*pid, process))
    .collect();

        process_list.sort_by_key(|&(_, process)| !process.pid());
        let mut rows = vec![];

for (pid, process) in process_list  {

    let systemtime: u64;


        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        unsafe{systemtime = SYSTEM_START_TIME;}
      
        let process_duration = format_time(now - systemtime + process.start_time());
        let name = process.name();
        let mem = pretty_bytes::converter::convert((process.memory() as f64) * 1000.0);
        let cpu=process.cpu_usage();
        let status=process.status();

        if name.is_empty() {
            continue;
        }

        for name_part in name.split_whitespace() {
            rows.push(format!(" {:<8}  {:<100}  {:>10}  {:>20}%  {:>20}  {:<70} ", pid, name_part, mem, cpu, process_duration, status));
        }
    }

  
    println!("| {:<8} | {:<100} | {:<10} | {:<20} | {:<20} |{:<70}", "PID", "Name", "Memory", "cpu", "process_duration", "status");
  

    for row in rows {
        println!("{}", row);
    }
    }

pub fn findbyname(name: &str) -> Vec<Process> {
    let mut matchingname = Vec::new();
    for process in psutil::process::processes().unwrap() {
        if let Ok(process) = process {
            if let Ok(process_name) = process.name() {
                if process_name.to_lowercase() == name.to_lowercase() {
                    matchingname.push(process);
                }
            }
        }
    }
    matchingname
}



pub fn processtable(){

    let mut system = System::new();
   
    system.refresh_all();
    let mut process_list: Vec<_> = system
        .get_process_list()
        .iter()
        .map(|(pid, process)| (*pid, process))
        .collect();
    
    let mut rows2 = vec![];
    
    for (pid, process) in process_list  {
    
        let systemtime: u64;
    
    
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            unsafe{systemtime = SYSTEM_START_TIME;}
          
            let process_duration = format_time(now - systemtime + process.start_time());
            let name = process.name();
            let mem =   pretty_bytes::converter::convert((process.memory() as f64) * 1000.0);
            let cpu=process.cpu_usage();
            let status=process.status();
     //  let disk= process.disk_usage();
    
            if name.is_empty() {
                continue;
            }
    
            for name_part in name.split_whitespace() {
                rows2.push(format!(" {:<8}  {:<100}  {:>10}  {:>10}%  {:>20}  {:<70} ", pid, name_part, mem, cpu, process_duration, status));
            }
        }
    
      
        println!("| {:<8} | {:<95} | {:<10} | {:<10}  | {:<20} | {:<70}", "PID", "Name", "Memory", "cpu", "process_duration", "status");
       
        for row in rows2 {
            println!("{}", row);
        }
    


}

fn main() {

// regular process table ///////////////////////////////////
let mut system = System::new();
   
system.refresh_all();

processtable();
////////////////////////////////////////////////////////////////////////
system.refresh_network();
    println!("total memory: {}", pretty_bytes::converter::convert((system.get_total_memory() as f64) * 1000.0));
    println!("used  memory : {}", pretty_bytes::converter::convert((system.get_used_memory() as f64) * 1000.0));
    println!("{:?}", system.get_network());
    
    
    // let network = system.get_network();

    //   //  println!("Interface name: {}", network.get_name());
    //     println!("Input: {:.2} B/s", network.get_income());
    //     println!("Output: {:.2} B/s", network.get_outcome());


	loop{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to get input.");
		match input.as_str().trim(){
            "memory info "=> {
            println!("total memory: {}", pretty_bytes::converter::convert((system.get_total_memory() as f64) * 1000.0));
            println!("{}", pretty_bytes::converter::convert((system.get_used_memory() as f64) * 1000.0))},
            "Network info" => {println!("{:?}", system.get_network());},
			"sysinfo" => {},
			"hddtemp" => {},
			"gputemp" => {},
			"sensors" => {},
      "help" => { 
        println!("Commands: ");
        println!("sort --> sort process table ASC relative to pid");
        println!("kill (process pid) --> kills process by pid");
        println!("pstree --> prints process tree");
        println!("search--> searches process by pid");
        println!("     ");

    },
    "findbyid" => { 

          let mut iter = input.split_whitespace();
    // Skip the first string
    let parsed = iter.nth(1).and_then(|num_str| num_str.trim().parse::<i32>().ok());
let parsedpid= parsed.unwrap();


let pid = 30515;

match findbypid(pid) {
    Some(process) => println!("Found process {} with PID {}", process.name(), pid),
    None => println!("No process found with PID {}", pid),
}
   
    },
    "kill" => { 
        let pidkill = 18143; //replace with command arg
        if let Some(process) = system.get_process(pidkill) {
            println!("Killing PID {}: {}", pidkill, process.name());
            process.kill(sysinfo::Signal::Kill); // SIGKILL signal to the process
        } else {
            println!("Process {} not found.", pidkill);
        }
       

    },
    "finsbyname" => { 
        let name = "firefox";

        let matching_processes = findbyname(name);
        if matching_processes.len() > 0 {
            println!("Found {} processes with name '{}':", matching_processes.len(), name);
            for process in matching_processes {
                println!("PID: {}, Name: {}", process.pid(), process.name().unwrap_or("unknown process".to_string()));
            }
        } else {
            println!("No processes found with name '{}'", name);
        }
    },
    "desc" => { 
     sortasc();
     

    },
    "PT" => { 
        processtable();
        
   
       },
    "asc" => { 
        sortdesc();

    println!("     ");
    },
			"quit" => {
				println!("Exiting program");
				break;
			},
			_ => {println!("Commands: ");
            println!("sort --> sort process table ASC relative to pid");
            println!("kill (process pid) --> kills process by pid");
            println!("pstree --> prints process tree");
            println!("search--> searches process by pid");
            println!("     ");}
		}
	}

}