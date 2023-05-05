use std::io;
use std::thread;
use std::time::Duration;
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt, CpuRefreshKind, RefreshKind, DiskExt};
use termion::{color, cursor, style};
use std::io::Write;

fn display_system_information(sys: &System) {
    println!("System name: {}", sys.name().unwrap());
    println!("System kernel version: {}", sys.kernel_version().unwrap());
    println!("System OS version: {}", sys.os_version().unwrap());
    println!("System host name: {}", sys.host_name().unwrap());
}

fn display_components_information(sys: &mut System) {
    for component in sys.components() {
        println!("{:?}", component);
    }
}

fn display_disks_information(sys: &mut System) {
    let base: u64 = 2;
    let power: u32 = 20;
    for disk in sys.disks() {
        println!("Disk name: {}", disk.name().to_str().unwrap());
        println!("Disk type: {}", disk.name().to_str().unwrap());
        println!("Total space: {} MB",  disk.total_space()/(base.pow(power)));
        println!("Available space: {} MB", disk.available_space()/(base.pow(power)));

    }
}

fn monitor_hddtemp(sys: &mut System) {
    let mut stdout = io::stdout();
    loop {
        write!(stdout, "{}", cursor::Left(100)).unwrap();
        for component in sys.components_mut() {
            if component.label().contains("SSD") || component.label().contains("HDD"){
                if component.temperature() < component.critical().unwrap(){
                    write!(stdout, "{}{}{}{}: {:?}°C{}", cursor::Hide, termion::clear::CurrentLine, color::Fg(color::Green), component.label(), component.temperature(), color::Fg(color::Reset)).unwrap();    
                }
                else {
                    write!(stdout, "{}{}{}{}: {:?}°C{}", cursor::Hide, termion::clear::CurrentLine, color::Fg(color::Red), component.label(), component.temperature(), color::Fg(color::Reset)).unwrap();    
                }
                component.refresh();
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}

fn display_hddtemp(sys: &mut System, arg: &str) {
    if arg == "" {
        for component in sys.components_mut() {
            if component.label().contains("SSD") || component.label().contains("HDD"){
                if component.temperature() < component.critical().unwrap() {
                    //println!("{}{}This is a {}colored{} line", color::Fg(color::Red), style::Bold, color::Fg(color::Yellow), style::Reset);
                    println!("{}{}: {:?}°C{}", color::Fg(color::Green), component.label(), component.temperature(), style::Reset);
                }
                else {
                    println!("{}{}: {:?}°C{}", color::Fg(color::Red), component.label(), component.temperature(), style::Reset);
                }
                component.refresh();
                
            }
        }

    }
    else if arg == "max" {
        for component in sys.components_mut() {
            if component.max() < component.critical().unwrap() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    println!("{}{}: {:?}°C{}", color::Fg(color::Green), component.label(), component.max(), style::Reset);
                }
                else {
                    println!("{}{}: {:?}°C{}", color::Fg(color::Red), component.label(), component.max(), style::Reset);
                }
                component.refresh();    
            }
            
            
        }
    }
    else if arg == "crit"{
        for component in sys.components_mut() {
            if component.label().contains("SSD") || component.label().contains("HDD"){
                println!("{}{}: {:?}°C{}", color::Fg(color::Red), component.label(), component.temperature(), style::Reset);
                component.refresh();
            }   
        }
    }
    
}

fn display_gputemp(sys: &mut System, arg: &str) {
    if arg == "" {
        for component in sys.components_mut() {
            if component.label().contains("gpu") {
                println!("{}: {}°C", component.label(), component.temperature());
                component.refresh();
            }
        }       
    }
    else if arg == "max" {
        for component in sys.components_mut() {
            if component.label().contains("gpu"){
                println!("{}: {}°C", component.label(), component.max());
                component.refresh();
            }
        }       
    }
}

fn monitor_gputemp(sys: &mut System) {
    let mut stdout = io::stdout();
    loop {
        write!(stdout, "{}", cursor::Left(100)).unwrap();
        for component in sys.components_mut() {
            if component.label().contains("gpu") {
                write!(stdout, "{}{}: {:?}°C", cursor::Hide, component.label(), component.temperature()).unwrap();
                component.refresh();
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        }   
    }
}

fn display_cpu_information(sys: &mut System) {
    for cpu in sys.cpus() {
        println!("Vendor ID: {}", cpu.vendor_id());
        println!("Model name: {}", cpu.name());
        println!("Brand: {}", cpu.brand());
        println!("Frequency: {}", cpu.frequency());
        break;
    }
}


fn main() {
    let mut sys = System::new_all();
    //let networks = sys.networks();
    loop{
        let mut input = String::new();
        let mut arg: &str = "";
        io::stdin().read_line(&mut input).expect("Failed to get input.");

        match input.as_str().trim() {
            "sysname" => {println!("{}", sys.name().unwrap())},
            "uname" => {println!("{}", sys.kernel_version().unwrap())},
            "release" => {println!("{}", sys.os_version().unwrap())},
            "hostname" => {println!("{}", sys.host_name().unwrap())},
            "sysinfo" => {display_system_information(&sys)},
            "hddtemp" => {
                display_hddtemp(&mut sys, arg);
            },
            "hddtemp -max" => {
                arg = "max";
                display_hddtemp(&mut sys, arg);
            },
            "hddtemp -crit" => {
                arg = "crit";
                display_hddtemp(&mut sys, arg);
            },
            "hddtemp -mon" => {
                monitor_hddtemp(&mut sys);
            },
            "gputemp" => {
                display_gputemp(&mut sys, arg)
            },
            "gputemp -mon" => {
                monitor_gputemp(&mut sys);
            },
            "gputemp -max" => {
                arg = "max";
                display_gputemp(&mut sys, arg);
            },
            "sensors" => {display_components_information(&mut sys)},
            "lscpu" => {display_cpu_information(&mut sys)},
            "df" => {display_disks_information(&mut sys)},
            "quit" => {
                println!("Exiting program");
                break;
            },
            _ => println!("Got some other input")
        }
    }
}
