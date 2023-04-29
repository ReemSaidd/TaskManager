use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();



    for (pid, process) in system.processes() {
      let mem= process.memory()/1024;
      let mut diskwritbytes;
      let mut diskreadbytes:u64;
      
      if process.disk_usage().total_written_bytes==0 {
      diskwritbytes=0;
    }
    else {
    diskwritbytes = process.disk_usage().written_bytes/ process.disk_usage().total_written_bytes;}

    if process.disk_usage().total_read_bytes==0 {
        diskreadbytes=0;
      }
      else{
      diskreadbytes = process.disk_usage().read_bytes/ process.disk_usage().total_read_bytes;}

     // let diskreadbytes= process.disk_usage().read_bytes/process.disk_usage().total_read_bytes;
        println!("PID: {}, Name: {}, Status: {:?}, cpu: {:.2}%, Memory: {:.3}Mkibs, written bytes(disk): {}, read bytes(disk): {}", 
                 pid,
                 process.name(),
                 process.status(),
                 process.cpu_usage(),
                 mem,
                 diskwritbytes,
                 diskreadbytes
        );
    }


    println!("System kernel version:   {:?}", system.kernel_version());
    println!("System OS version:       {:?}", system.os_version());
    println!("System host name:        {:?}", system.host_name());

    println!("total memory: {} KB", system.total_memory());
    println!("used memory : {} KB", system.used_memory());
    println!("total swap  : {} KB", system.total_swap());
    println!("used swap   : {} KB", system.used_swap());

 
}