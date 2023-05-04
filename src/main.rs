use std::process;
use clap::{Arg, App, SubCommand};
use sysinfo::{RefreshKind,NetworkData,NetworksExt, ProcessExt, SystemExt, Signal, ProcessRefreshKind, System};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{ComponentExt, CpuExt, CpuRefreshKind};
use std::net::Ipv4Addr;
use std::process::Command;
//use std::process::Command;
//use procfs::{ProcResult, Process};
use std::io;
//extern crate libc;

use libc::{kill, pid_t};
use psutil::process::processes;
use psutil::process::Process;
use psutil::Result;
use prettytable::{Table, Row, Cell};
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
    let mut system = System::new_all();
    //let network = system.networks();
    //let networks = Networks::new();
    system.refresh_all();
    system.refresh_cpu();

    system.refresh_networks();
    system.refresh_networks_list();
   

    let mut table = Table::new();
    
    table.add_row(
        Row::new(vec![
            Cell::new("PID"),
            Cell::new("NAME"),
            Cell::new("STATUS"),
            Cell::new("CPU USAGE %"),
            Cell::new("MEMORY USAGE (GB)"),
            Cell::new("DISK WRITE BYTES"),
            Cell::new("DISK READ BYTES"),
          
        ])
    );
    
    for (pid, process) in system.processes() {
      
    
      
     
      let  diskwritbytes;
      let  diskreadbytes:u64;
    if process.disk_usage().total_written_bytes == 0 {
      diskwritbytes = 0;  
    }
    else 
    {
    diskwritbytes = process.disk_usage().written_bytes / process.disk_usage().total_written_bytes;
    }

    if process.disk_usage().total_read_bytes==0 
    {
        diskreadbytes=0;
    }
    else
    {
      diskreadbytes = process.disk_usage().read_bytes/ process.disk_usage().total_read_bytes;
    }
        table.add_row(
            Row::new(vec![
                Cell::new(&pid.to_string()),
                Cell::new(process.name()),
                Cell::new(&format!("{:?}", process.status())),
                Cell::new(&format!("{:.2} %", process.cpu_usage())),
                Cell::new(&format!("{:.2} Gbs", process.memory() as f64 / 1024.0 / 1024.0 / 1024.0)),
                Cell::new(&diskwritbytes.to_string()),
                Cell::new(&diskreadbytes.to_string()),
            
            ])
        );
    }
    
    
    table.printstd();

    // let process_id = 1234; // Replace with actual process ID
    // let pid = process::id();

    // let result = unsafe { kill(process_id as pid_t, libc::SIGTERM) };

    // match result {
    //     0 => println!("Successfully sent SIGTERM to process {}", process_id),
    //     _ => {
    //         let err_output = Command::new("kill")
    //             .arg("-TERM")
    //             .arg(process_id.to_string())
    //             .output()
    //             .expect("failed to execute process");
    //         println!("Error sending SIGTERM to process {} ({})", process_id,
    //         String::from_utf8_lossy(&err_output.stderr));
    //     }
    // }
  //   for process in procfs::all_processes()? {
  //     let pid = process.pid;
  //     let net = process.net()?;
  //     println!("PID: {}\n", pid);

  //     if net.is_empty() {
  //         println!("No network info available for this process.");
  //         continue;
  //     }

  //     for connection in net {
  //         println!("Remote address: {}", connection.get_peer_address().unwrap());
  //         println!("Local address: {}", connection.get_local_address().unwrap());
  //         println!("Status: {}", connection.get_status());
  //         println!("");
  //     }
  // }
  // Ok(())}


    // println!("System kernel version:   {:?}", system.kernel_version());
    // println!("System OS version:       {:?}", system.os_version());
    // println!("System host name:        {:?}", system.host_name());

    println!("total memory: {} GB", system.total_memory()as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("used  memory : {} GB", system.used_memory()as f64 / 1024.0 / 1024.0 / 1024.0);
    // println!("total swap  : {} KB", system.total_swap());
    // println!("used  swap   : {} KB", system.used_swap());

 

	loop{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to get input.");
		match input.as_str().trim(){
			"sysname" => {println!("{}", system.name().unwrap())},
			"uname" => {println!("{}", system.kernel_version().unwrap())},
			"release" => {println!("{}", system.os_version().unwrap())},
			"hostname" => {println!("{}", system.host_name().unwrap())},
      "mem "=> {println!("{} KB", system.total_memory());
    
    },
      "Used Memory"=> {println!("{} KB", system.used_memory())},
			"sysinfo" => {},
			"hddtemp" => {},
			"gputemp" => {},
			"sensors" => {},
      "help" => { 
        println!("Commands: ");
        println!("print --> prints process table   ");
        println!("kill (process pid) --> kills process by pid");
        println!("pstree --> prints process tree");
        // println!("");

    },
    
			"quit" => {
				println!("Exiting program");
				break;
			},
			_ => println!("Got some other input")
		}
	}

}