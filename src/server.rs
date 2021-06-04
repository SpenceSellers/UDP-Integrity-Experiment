use std::net::UdpSocket;
use sha2::{Digest, Sha256};
use crate::{MESSAGE_SIZE, HASH_SIZE};

pub fn run_test_server(port: &str) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port)).expect("Could not bind to address");
    let mut good_count: u64 = 0;
    let mut bad_count: u64 = 0;
    loop {
        let mut buf = [0; MESSAGE_SIZE];
        socket.recv_from(&mut buf).expect("Could not receive");
        let hash = &buf[MESSAGE_SIZE-HASH_SIZE..];
        let data = &buf[..MESSAGE_SIZE-HASH_SIZE];

        let mut digest = Sha256::new();
        digest.update(&data);
        let hash_result = digest.finalize();
        if &hash_result[..] != hash {
            bad_count += 1;
            println!("BAD HASH!");
        } else {
            good_count += 1;
            if (good_count + bad_count) % 100_000 == 0 {
                println!("{} good messages, {} bad messages", good_count, bad_count);
            }
        }

        if (good_count + bad_count) == 100_000_000 {
            println!("Fully complete.");
            break;
        }
    }
}
