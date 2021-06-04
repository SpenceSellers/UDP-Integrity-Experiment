use std::net::UdpSocket;
use sha2::{Digest, Sha256};
use crate::{MESSAGE_SIZE, HASH_SIZE};

pub fn run_test_server(port: &str) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port)).expect("Could not bind to address");
    let mut status = ExperimentStatus { good_count: 0, bad_count: 0};
    loop {
        let mut buf = [0; MESSAGE_SIZE];
        socket.recv_from(&mut buf).expect("Could not receive");
        let hash = &buf[MESSAGE_SIZE-HASH_SIZE..];
        let data = &buf[..MESSAGE_SIZE-HASH_SIZE];

        let mut digest = Sha256::new();
        digest.update(&data);
        let hash_result = digest.finalize();
        if &hash_result[..] != hash {
            status.bad_count += 1;
            println!("BAD HASH!");
        } else {
            status.good_count += 1;
            if status.total_received() % 100_000 == 0 {
                println!("{} good messages, {} bad messages", status.good_count, status.bad_count);
            }
        }

        if status.total_received() == 100_000_000 {
            println!("Fully complete.");
            break;
        }
    }
}

struct ExperimentStatus {
    good_count: u64,
    bad_count: u64
}

impl ExperimentStatus {
    pub fn total_received(&self) -> u64 { self.good_count + self.bad_count }
}
