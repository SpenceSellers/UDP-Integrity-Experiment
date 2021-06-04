use std::net::UdpSocket;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use crate::messages::{validate_message, MESSAGE_SIZE};

pub fn run_test_server(port: &str) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port)).expect("Could not bind to address");
    println!("Listening on port {}", port);
    let status = Arc::new(Mutex::new(ExperimentStatus { good_count: 0, bad_count: 0}));

    start_monitoring_thread(&status);
    loop {
        let mut buf = [0; MESSAGE_SIZE];
        socket.recv_from(&mut buf).expect("Could not receive");
        let message_was_valid = validate_message(&buf);

        let mut status = status.lock().unwrap();
        if message_was_valid {
            status.good_count += 1;
        } else {
            status.bad_count += 1;
        }
    }
}


#[derive(Eq, PartialEq, Hash, Clone)]
struct ExperimentStatus {
    good_count: u64,
    bad_count: u64
}

fn start_monitoring_thread(status: &Arc<Mutex<ExperimentStatus>>) {
    let status = Arc::clone(&status);
    thread::spawn(move || {
        let mut last_status = status.lock().unwrap().clone();
        loop {
            {
                let status = status.lock().unwrap();
                if *status != last_status {
                    println!("{} good datagrams, {} bad datagrams", status.good_count, status.bad_count);
                    last_status = status.clone();
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

