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

//use std::time::{SystemTime, UNIX_EPOCH};

//use crate::SYSTEM_START_TIME;

//use std::process::Command;
//use procfs::{ProcResult, Process};
use std::io;
//extern crate libc;

use libc::{kill, pid_t};
use psutil::process::processes;
use psutil::process::Process;
use psutil::Result;
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


//extern crate psutil;
//use psutil::{process::Process, Result};

// pub fn find_process_by_name(name: &str) -> Result<Option<Process>> {
//   let mut found: Option<Process> = None;

//   for process_result in processes()? {
//       if let Ok(process) = process_result {
//           if process.name() == name {
//               found = Some(process);
//               break;
//           }
//       }
//   }

//   Ok(found)
// }
fn main() {

    let mut system = System::new();
   
    system.refresh_all();
 


///////////////////////////kill process////////

  

//////////////////////////////////////////////////////////////////////////////////////
/// //////////////////////////////search by id ////////////////////
// let pidsearch = 1234; 

//     if let Some(process) = system.get_process_by_pid(pidsearch) {
//         println!("Process found: {}", process.name());
//         println!("| {:<8}  {:<100}  {:<10}  {:<20}  {:<70}", pidsearch, process.name(), process.memory(), process.cpu_usage(),process.status());
//     } else {
//         println!("Process not found");
//     } 


/// /////////////////////////////////////////////
//////////////////////////sorted process table ////////////////////////////
// let mut sortedprocess_list: Vec<_> = system
//     .get_process_list()
//     .iter()
//     .map(|(pid, process)| (*pid, process))
//     .collect();

// sortedprocess_list.sort_by_key(|&(_, process)| process.pid());

// let mut rows = vec![];

// for (pid, process) in sortedprocess_list  {

//     let systemtime: u64;


//         let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
//         unsafe{systemtime = SYSTEM_START_TIME;}
      
//         let process_duration = format_time(now - systemtime + process.start_time());
//         let name = process.name();
//         let mem = pretty_bytes::converter::convert((process.memory() as f64) * 1000.0);
//         let cpu=process.cpu_usage();
//         let status=process.status();
//  //  let disk= process.disk_usage();

//         if name.is_empty() {
//             continue;
//         }

//         for name_part in name.split_whitespace() {
//             rows.push(format!(" {:<8}  {:<100}  {:>10}  {:>20}%  {:>20}  {:<70} ", pid, name_part, mem, cpu, process_duration, status));
//         }
//     }

  
//     println!("| {:<8} | {:<100} | {:<10} | {:<20} | {:<20} |{:<70}", "PID", "Name", "Memory", "cpu", "process_duration", "status");
  

//     for row in rows {
//         println!("{}", row);
//     }
//////////////////////////////////////////////////////////////////////////////////////////

// regular process table ///////////////////////////////////

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

  
    println!("| {:<8} | {:<100} | {:<10} | {:<10}  | {:<20} | {:<70}", "PID", "Name", "Memory", "cpu", "process_duration", "status");
   
    for row in rows2 {
        println!("{}", row);
    }

////////////////////////////////////////////////////////////////////////
system.refresh_network();
    println!("total memory: {}", pretty_bytes::converter::convert((system.get_total_memory() as f64) * 1000.0));
    println!("used  memory : {}", pretty_bytes::converter::convert((system.get_used_memory() as f64) * 1000.0));
    println!("{:?}", system.get_network());
    
    
    let network = system.get_network();

      //  println!("Interface name: {}", network.get_name());
        println!("Input: {:.2} B/s", network.get_income());
        println!("Output: {:.2} B/s", network.get_outcome());


	loop{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to get input.");
		match input.as_str().trim(){
			// "sysname" => {println!("{}", system.name().unwrap())},
			// "uname" => {println!("{}", system.kernel_version().unwrap())},
			// "release" => {println!("{}", system.os_version().unwrap())},
			// "hostname" => {println!("{}", system.hostname().unwrap())},
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
    "kill" => { 
        let pidkill = 18143; //replace with command arg
        if let Some(process) = system.get_process(pidkill) {
            println!("Killing PID {}: {}", pidkill, process.name());
            process.kill(sysinfo::Signal::Kill); // SIGKILL signal to the process
        } else {
            println!("Process {} not found.", pidkill);
        }
       

    },
    "sort" => { 
        let mut sortedprocess_list: Vec<_> = system
    .get_process_list()
    .iter()
    .map(|(pid, process)| (*pid, process))
    .collect();

sortedprocess_list.sort_by_key(|&(_, process)| process.pid());

let mut rows = vec![];

for (pid, process) in sortedprocess_list  {

    let systemtime: u64;


        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        unsafe{systemtime = SYSTEM_START_TIME;}
      
        let process_duration = format_time(now - systemtime + process.start_time());
        let name = process.name();
        let mem = pretty_bytes::converter::convert((process.memory() as f64) * 1000.0);
        let cpu=process.cpu_usage();
        let status=process.status();
 //  let disk= process.disk_usage();

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