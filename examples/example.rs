extern crate blkid;
use blkid::probe::Probe;
use std::fs;

fn main() {
    let partitions = fs::read_to_string("/proc/partitions").expect("failed to get list of partitions");
    let partitions = partitions.lines().skip(2)
        .filter_map(|line| line.split_whitespace().nth(3));

    println!("DEVICE, FSTYPE, UUID, PARTUUID");
    for partition in partitions {
        let path = ["/dev/", partition].concat();
        let probe = Probe::new_from_filename(&path).expect("failed to probe device");
        probe.do_probe();

        let dm_name = ["/sys/class/block/", partition, "/dm/name"].concat();
        let dm_;
        let path = match fs::read_to_string(&dm_name) {
            Ok(device_map) => {
                dm_ = ["/dev/mapper/", device_map.trim_right()].concat();
                &dm_
            }
            Err(_) => &path
        };

        println!(
            "{}, {}, {}, {}",
            path,
            probe.lookup_value("TYPE").unwrap_or("N/A"),
            probe.lookup_value("UUID").unwrap_or("N/A"),
            probe.lookup_value("PARTUUID").unwrap_or("N/A")
        );
    }
}
