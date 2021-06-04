use std::net::UdpSocket;
use sha2::{Digest, Sha256};

pub fn run_test_server() {
    let socket = UdpSocket::bind("0.0.0.0:3555").expect("Could not bind to address");
    let mut good_count: u64 = 0;
    let mut bad_count: u64 = 0;
    loop {
        let mut buf = [0; 256];
        socket.recv_from(&mut buf).expect("Could not receive");
        let hash = &buf[224..];
        let data = &buf[..224];

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
