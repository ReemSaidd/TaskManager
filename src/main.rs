use std::io;
use std::thread;
use std::time::Duration;
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt, CpuRefreshKind, RefreshKind};
use termion::{color, cursor, style};
use std::io::Write;

fn display_system_information(sys: &System) {
	println!("System name: {}", sys.name().unwrap());
	println!("System kernel version: {}", sys.kernel_version().unwrap());
	println!("System OS version: {}", sys.os_version().unwrap());
	println!("System host name: {}", sys.host_name().unwrap());
}

fn display_components(sys: &mut System) {
	for component in sys.components() {
		println!("{:?}", component);
	}
}

// fn monitor_components(sys: &mut System) {
// 	loop {
// 		for component in sys.components() {
// 			println!("{:?}", component);
// 		}
// 	}
// }

fn monitor_hddtemp(sys: &mut System) {
	let mut stdout = io::stdout();
	loop {
		write!(stdout, "{}", cursor::Left(100)).unwrap();
		for component in sys.components_mut() {
			if component.label().contains("SSD") || component.label().contains("HDD"){
				write!(stdout, "{}{}{}: {:?}°C{}", termion::clear::CurrentLine, color::Fg(color::Red), component.label(), component.temperature(), color::Fg(color::Reset),).unwrap();
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
				write!(stdout, "{}: {:?}°C", component.label(), component.temperature()).unwrap();
				component.refresh();
				stdout.flush().unwrap();
				thread::sleep(Duration::from_millis(200));
			}
		}	
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
			"sensors" => {display_components(&mut sys)},
			"quit" => {
				println!("Exiting program");
				break;
			},
			// "cpu_usage" => {loop{println!("{}%", sys.global_cpu_info().cpu_usage()); sys.refresh_cpu()}},
			//"lscpu" => {display_cpu_info(&sys)},
			_ => println!("Got some other input")
		}
	}
}
