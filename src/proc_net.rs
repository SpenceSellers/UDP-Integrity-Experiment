use std::fs;
use std::str::FromStr;

#[derive(Debug)]
pub struct UdpPortStats {
    pub drops: u64,
    pub rx_queue: u64
}

pub fn get_udp_port_stats(port: u16) -> Result<UdpPortStats, ()> {
    let udp_file = fs::read_to_string("/proc/net/udp").expect("Could not read /proc/net/udp");
    return parse_udp_stats(port, &udp_file);
}

fn parse_udp_stats(port: u16, udp_file: &str) -> Result<UdpPortStats, ()> {
    for line in udp_file.lines().skip(1) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        let port_hex = columns[1].split(":").nth(1).unwrap();
        let line_port = u16::from_str_radix(port_hex, 16).expect("Invalid port format");
        if line_port != port {
            continue;
        }

        let rx = columns[4].split(":").nth(1).unwrap();

        return Ok(UdpPortStats {
            drops: u64::from_str(columns[12]).unwrap(),
            rx_queue: u64::from_str_radix(rx, 16).unwrap()
        });
    }

    return Err(());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_udp_stats_file() {
        let file = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops
 1680: 00000000:DF9E 00000000:0000 07 00000000:00000000 00:00000000 00000000   115        0 28970 2 0000000000000000 0
 1831: 3500007F:0035 00000000:0000 07 00000000:00000000 00:00000000 00000000   101        0 23396 2 0000000000000000 0
 1846: 2358A8C0:0044 0158A8C0:0043 01 00000000:00000000 00:00000000 00000000     0        0 1983485 2 0000000000000000 0
 1237: 00000000:0DE3 00000000:0000 07 00000000:00000FF3 00:00000000 00000000  1000        0 2090741 2 0000000000000000 1890
 2058: 00000000:2118 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 70563 2 0000000000000000 0
 2409: 00000000:0277 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 1935475 2 0000000000000000 0
 3035: 00000000:14E9 00000000:0000 07 00000000:00000000 00:00000000 00000000   115        0 28968 2 0000000000000000 0";

        let result = parse_udp_stats(3555, file).expect("Parsing failed");

        assert_eq!(result.drops, 1890);
        assert_eq!(result.rx_queue, 4083);
    }
}