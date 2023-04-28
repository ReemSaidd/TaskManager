use std::io;
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt, CpuRefreshKind, RefreshKind};

fn display_system_information(sys: &System){
	println!("System name: {}", sys.name().unwrap());
	println!("System kernel version: {}", sys.kernel_version().unwrap());
	println!("System OS version: {}", sys.os_version().unwrap());
	println!("System host name: {}", sys.host_name().unwrap());
}

fn display_components(sys: &System){
	for component in sys.components(){
		println!("{:?}", component);
	}
}

fn display_hddtemp(sys: &System){
	for component in sys.components() {
		if component.label().contains("SSD") || component.label().contains("HDD"){
			println!("{}: {:?}°C", component.label(), component.temperature());
		}
	}	
}

fn display_gputemp(sys: &System){
	for component in sys.components() {
		if component.label().contains("gpu"){
			println!("{}: {}°C", component.label(), component.temperature());
		}
	}
}

fn main() {
	let mut sys = System::new_all();
	//let networks = sys.networks();
	loop{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to get input.");
		match input.as_str().trim(){
			"sysname" => {println!("{}", sys.name().unwrap())},
			"uname" => {println!("{}", sys.kernel_version().unwrap())},
			"release" => {println!("{}", sys.os_version().unwrap())},
			"hostname" => {println!("{}", sys.host_name().unwrap())},
			"sysinfo" => {display_system_information(&sys)},
			"hddtemp" => {display_hddtemp(&sys)},
			"gputemp" => {display_gputemp(&sys)},
			"sensors" => {display_components(&sys)},
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

