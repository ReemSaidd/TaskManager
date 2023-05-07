
use sysinfo::{ComponentExt, System, SystemExt, CpuExt, DiskExt};
use std::str;
use ptree::{self, item::StringItem};
use std::collections::HashMap;

pub fn get_system_information(sys: &System) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    vec.push(format!("Name: {}", sys.name().unwrap()));
    vec.push(format!("Kernel version: {}", sys.kernel_version().unwrap()));
    vec.push(format!("OS version: {}", sys.os_version().unwrap()));
    vec.push(format!("Host name: {}", sys.host_name().unwrap()));
    return vec;
}

pub fn get_components_information(sys: &mut System) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for component in sys.components() {
        vec.push(format!("{:?}", component));
    }
    return vec;
}

pub fn get_hddtemp(sys: &mut System, arg: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    match arg.as_str() {
        "" => {
            for component in sys.components_mut() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    vec.push(format!("{}: {:?}°C", component.label(), component.temperature()));
                    component.refresh();
                }
            }            
        },
        "max" => {
            for component in sys.components_mut() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    vec.push(format!("{}: {:?}°C", component.label(), component.max()));
                    component.refresh();
                }
            }
        },
        "crit" => {
            for component in sys.components_mut() {
                if component.label().contains("SSD") || component.label().contains("HDD"){
                    vec.push(format!("{}: {:?}°C", component.label(), component.critical().unwrap()));
                    component.refresh();
                }
            }
        },
        _ => {},
    }   
    return vec;
}

pub fn get_disks_information(sys: &mut System, arg: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let base: u64 = 2;
    let mut power: u32 = 0;
    match arg.as_str() {
        "" => {power = 0;},
        "k" => {power = 10;},
        "m" => {power = 20;},
        _ => {},
    }
    vec.push(format!("{:<50} {:<50} {:<50} {:<50} {:<50} {:<50}", "Name", "Mount Point", "Filesystem", "Total Space", "Available Space", "Used Space"));
    for disk in sys.disks() {
        vec.push(format!("{:<50} {:<50} {:<50} {:<50} {:<50} {:<50}", disk.name().to_str().unwrap(), disk.mount_point().to_str().unwrap(), str::from_utf8(disk.file_system()).unwrap(), disk.total_space()/(base.pow(power)), disk.available_space()/(base.pow(power)), disk.total_space()/(base.pow(power)) - disk.available_space()/(base.pow(power))));
    }
    return vec;
}

pub fn get_cpu_information(sys: &mut System) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    vec.push(format!("{:<50} {:<50} {:<50} {:<50}", "Brand", "Vendor ID", "Name", "Frequency"));
    for cpu in sys.cpus() {
        vec.push(format!("{:<50} {:<50} {:<50} {:<50}", cpu.brand(), cpu.vendor_id(), cpu.name(), cpu.frequency()));
    }
    return vec;
}

pub fn get_gputemp(sys: &mut System, arg: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    match arg.as_str() {
        "" =>  {
            for component in sys.components_mut() {
                if component.label().contains("gpu") {
                    vec.push(format!("{}: {}°C", component.label(), component.temperature()));
                    component.refresh();
                }
            }       
        },
        "max" => {
            for component in sys.components_mut() {
                if component.label().contains("gpu"){
                    vec.push(format!("{}: {}°C", component.label(), component.max()));
                    component.refresh();
                }
            }   
        },
        _ => {}
    }
    return vec;
}

pub fn pstree_new(sys: &mut System) {
    let processes = System::get_process_list();
    let mut sorted_keys: Vec<_> = processes.keys().collect();
    sorted_keys.sort();
    let mut process_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let  mut tree = ptree::TreeBuilder::new("root".to_string());
    let mut muttree = &mut tree;
    let mut resulttree: StringItem = muttree.build();

    for pid in sorted_keys {
        // let new = ptree::TreeBuilder::new("root".to_string()).begin_child(processes[pid].name().to_string());
        // let neww = tree.begin_child(processes[pid].name().to_string()).build();
        let process = &processes[pid];
        match process.parent() {
            Some(parent_pid) => {
                process_map
                    .entry(parent_pid)
                    .or_insert_with(Vec::new)
                    .push(*pid);
            }
            None => {
                // If there is no parent process, assume it is the root process
                process_map.entry(0).or_insert_with(Vec::new).push(*pid);
            }
        }
        //let results = ptree::print_tree(&neww);
    }

    let new_keys: Vec<_> = process_map.keys().collect();
    for pid in new_keys{
        if process_map[pid].len() >= 1 {
            let newleaf = muttree.add_empty_child(process_map[pid][0].to_string());
            muttree = newleaf.add_empty_child(" ".to_string());
            resulttree = muttree.build();
        }

        else{
            let newbranch = muttree.begin_child(process_map[pid][0].to_string());
            
            for i in 1..process_map[pid].len(){
                let newleaf = newbranch.add_empty_child(process_map[pid][i].to_string());
                resulttree = newleaf.build();
            }
        }
    }

    let results = ptree::print_tree(&resulttree);

    

}
