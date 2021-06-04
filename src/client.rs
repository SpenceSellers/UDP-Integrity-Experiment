use rand::RngCore;
use sha2::{Digest, Sha256};
use std::net::UdpSocket;

pub fn run_test_client() {
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

