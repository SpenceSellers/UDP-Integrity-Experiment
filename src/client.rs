use std::net::UdpSocket;
use crate::messages::build_message;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

pub fn run_test_client(dest: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");
    let mut random = rand::thread_rng();

    let state = Arc::new(Mutex::new(ClientState { count: 0 }));
    set_ctrlc_handler(&state);
    start_monitoring_thread(&state);

    println!("Sending to {}", dest);
    loop {
        let buf = build_message(&mut random);

        socket.send_to(&buf, dest).expect("Could not send");
        let mut state = state.lock().unwrap();
        state.count += 1;
    }
}

struct ClientState {
    count: u64,
}

fn start_monitoring_thread(state: &Arc<Mutex<ClientState>>) {
    let our_state = state.clone();
    thread::spawn(move || {
        loop {
            {
                let state = our_state.lock().unwrap();
                println!("Sent {} datagrams", state.count);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn set_ctrlc_handler(state: &Arc<Mutex<ClientState>>) {
    let our_state = state.clone();
    ctrlc::set_handler(move || {
        println!("\nQuitting");
        println!("Sent {} datagrams", our_state.lock().unwrap().count);
        std::process::exit(0);
    }).expect("Could not set ctrl-c handler.");
}



