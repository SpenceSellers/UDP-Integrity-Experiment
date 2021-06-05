use std::fs;
use std::str::FromStr;

#[derive(Debug)]
pub struct UdpPortStats {
    pub drops: u64,
    pub rx_queue: u64
}

pub fn get_udp_port_stats(port: u16) -> Result<UdpPortStats, ()> {
    let udp_file = fs::read_to_string("/proc/net/udp").expect("Could not read /proc/net/udp");
    for line in udp_file.lines().skip(1) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        let port_hex = columns[1].split(":").nth(1).unwrap();
        let line_port = u16::from_str_radix(port_hex, 16).expect("Invalid port format");
        if line_port != port {
            continue;
        }

        return Ok(UdpPortStats {
            drops: u64::from_str(columns[12]).unwrap(),
            rx_queue: 0
        });
    }

    return Err(());
}