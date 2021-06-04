use std::net::UdpSocket;
use crate::messages::build_message;

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



