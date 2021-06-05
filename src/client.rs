use std::net::UdpSocket;
use crate::messages::build_message;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

pub fn run_test_client(dest: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");
    let mut random = rand::thread_rng();

    let count = Arc::new(Mutex::new(0u64));
    set_ctrlc_handler(&count);
    start_monitoring_thread(&count);

    println!("Sending to {}", dest);
    loop {
        let buf = build_message(&mut random);

        socket.send_to(&buf, dest).expect("Could not send");
        let mut count = count.lock().unwrap();
        *count += 1;
    }
}

fn start_monitoring_thread(count: &Arc<Mutex<u64>>) {
    let our_count = count.clone();
    thread::spawn(move || {
        loop {
            {
                let count = our_count.lock().unwrap();
                println!("Sent {} datagrams", count);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn set_ctrlc_handler(count: &Arc<Mutex<u64>>) {
    let our_count = count.clone();
    ctrlc::set_handler(move || {
        println!("\nQuitting");
        println!("Sent {} datagrams", our_count.lock().unwrap());
        std::process::exit(0);
    }).expect("Could not set ctrl-c handler.");
}



