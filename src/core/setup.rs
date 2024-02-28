use sysinfo::{Disk, Disks, System};
use std::hash::{Hash, Hasher};

fn display_sysinfo() -> (String, String, String, String, String, String) {
        println!("Hello! Welcome to Meshi. This setup procedure will help you get started.\n");
    println!("You only need to run this command once. \n");
    let mut sys = System::new_all();
    sys.refresh_all();
    let name = System::name().unwrap();
    let kv = System::kernel_version().unwrap();
    let osv = System::os_version().unwrap();
    let hn = System::host_name().unwrap();
    let tm = sys.total_memory();
    let ncpus = sys.cpus().len();

    let disks: Disks = Disks::new_with_refreshed_list();
    let mut c: usize = 0;
    let mut new_disks: Vec<&sysinfo::Disk> = vec![];
    for disk in &disks {
        if Disk::name(disk).to_str().unwrap().contains("loop")
            || Disk::name(disk).to_str().unwrap().contains("root")
            || Disk::name(disk).to_str().unwrap().contains("overlay")
        {
            continue;
        } else {
            new_disks.push(disk);
            c += 1;
        }
    }

    println!("=> System:");
    println!("- Name: \t\t{}", name);
    println!("- Kernel Version: \t{}", kv);
    println!("- OS version: \t\t{}", osv);
    println!("- Hostname: \t\t{}", hn);
    println!("- Total Memory: \t{} B", tm);
    println!("- CPUs: \t\t{}", ncpus);
    println!("- Storage Devices ({}):", c);
    for disk in new_disks {
        println!(
            "\t\t\t{} \t({} B Available)",
            Disk::name(disk).to_str().unwrap(),
            Disk::available_space(disk)
        );
    }

    return (name, kv, osv, hn, tm.to_string(), ncpus.to_string())
}

fn generate_id(info: (String, String, String, String, String, String)) -> u64{
    let mut nh = std::hash::DefaultHasher::new();
    info.hash(&mut nh);
    nh.finish()

}

pub fn setup() {
    let info = display_sysinfo();
    let id = generate_id(info);
    println!("{}", id);
}
