use std::net::UdpSocket;
use sha2::{Sha256, Digest};
use rand;
use rand::RngCore;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).unwrap_or(&"client".to_owned()) == "client" {
        run_test_client();
    } else {
        run_test_server();
    }
}

fn run_test_client() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");
    let dest = "192.168.88.35:3555";
    let mut random = rand::thread_rng();

    for i in 0..=100_000_000 {
        let mut data = [0u8;224];
        random.fill_bytes(&mut data);

        let mut digest = Sha256::new();
        digest.update(&data);
        let result = digest.finalize();

        let buf: Vec<u8> = data.iter()
            .chain(&result)
            .cloned()
            .collect();
        assert_eq!(buf.len(), 256);
        socket.send_to(&buf, dest).expect("Could not send");
    }
}

fn run_test_server() {
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
