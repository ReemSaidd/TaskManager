// use std::process::Command;

extern crate sysinfo;


// use std::io::Write;
// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

use std::process;

// use tui::backend::TermionBackend;
// use tui::layout::{Constraint, Direction};
// use tui::Terminal;
// use termion::raw::IntoRawMode;
// use termion::cursor::Goto;
// use termion::input::MouseTerminal;
// use termion::screen::AlternateScreen;
// use termion::event::Key;
use sysinfo::{NetworkExt, NetworkData, System, SystemExt};

fn main(){
let mut sys = System::new();
let num = num_cpus::get();
println!("cpucoutn: {} ", num);
println!("=> system:");
// RAM and swap information:
println!("total memory: {} bytes", sys.get_total_memory());
println!("used memory : {} bytes", sys.get_used_memory());
println!("total swap  : {} bytes", sys.get_total_swap());
println!("used swap   : {} bytes", sys.get_used_swap());




// println!("cpu usage : {}", sys.getcpu_usage());
// println!("network: {}", sys.get_network());
    // loop {
    //     sys.refresh_cpu(); // Refreshing CPU information.
    //     for cpu in sys.cpus() {
    //         print!("{}% ", cpu.getcpu_usage());
    //     }
    //     // Sleeping for 500 ms to let time for the system to run for long
    //     // enough to have useful information.
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    // }
}

//     let mut sys = System::new();

//     // First we update all information of our `System` struct.
//     sys.refresh_all();
//     // println!("");

  
    
// }
    
//     let mut sys = System::new_all();
//     let me = procfs::process::Process::myself().unwrap();
// let me_stat = me.stat().unwrap();
// let tps = procfs::ticks_per_second();

// println!("{: >10} {: <8} {: >8} {}", "PID", "TTY", "TIME", "CMD");

// let tty = format!("pty/{}", me_stat.tty_nr().1);
// for prc in procfs::process::all_processes().unwrap() {
//     if let Ok(stat) = prc.unwrap().stat() {
//         if stat.tty_nr == me_stat.tty_nr {
//             // total_time is in seconds
//             let total_time =
//                 (stat.utime + stat.stime) as f32 / (tps as f32);
//             println!(
//                 "{: >10} {: <8} {: >8} {}",
//                 stat.pid, tty, total_time, stat.comm
//             );
//         }
//     }
// }
//     println!("Hello, world!");
