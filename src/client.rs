use rand::RngCore;
use sha2::{Digest, Sha256};
use std::net::UdpSocket;
use rand::prelude::ThreadRng;
use crate::{MESSAGE_SIZE, HASH_SIZE};

pub fn run_test_client(dest: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");
    let mut random = rand::thread_rng();

    let mut count = 0u64;
    println!("Sending to {}", dest);
    loop {
        count += 1;
        let buf = build_message(&mut random);

        socket.send_to(&buf, dest).expect("Could not send");
        if count % 10_000 == 0 {
            println!("Sent {} datagrams", count);
        }
    }
}

fn build_message(random: &mut ThreadRng) -> Vec<u8> {
    let mut data = [0u8; MESSAGE_SIZE-HASH_SIZE];
    random.fill_bytes(&mut data);

    let mut digest = Sha256::new();
    digest.update(&data);
    let result = digest.finalize();

    let buf: Vec<u8> = data.iter()
        .chain(&result)
        .cloned()
        .collect();
    debug_assert_eq!(buf.len(), MESSAGE_SIZE);
    buf
}

