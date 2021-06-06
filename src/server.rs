use std::net::UdpSocket;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use crate::messages::{validate_message, MESSAGE_SIZE, DecodedMessage};
use crate::proc_net;
use std::str::FromStr;

pub fn run_test_server(port: &str) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port)).expect("Could not bind to address");
    println!("Listening on port {}", port);
    let status = Arc::new(Mutex::new(ExperimentStatus::new()));

    let experiment = Experiment { port: u16::from_str(port).expect("Could not parse port")};

    start_monitoring_thread(experiment.clone(), &status);
    loop {
        let mut buf = [0; MESSAGE_SIZE];
        socket.recv_from(&mut buf).expect("Could not receive");
        let decoded_message = validate_message(&buf);

        let mut status = status.lock().unwrap();
        status.encounter_message(&decoded_message);
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct ExperimentStatus {
    good_count: u64,
    bad_count: u64,
    last_sequence_number: Option<u32>,
    out_of_order: u64,
}

impl ExperimentStatus {
    fn new() -> Self {
        ExperimentStatus {
            good_count: 0,
            bad_count: 0,
            last_sequence_number: None,
            out_of_order: 0
        }
    }

    fn encounter_message(&mut self, msg: &Option<DecodedMessage>) {
        if let Some(msg) = msg {
            self.good_count += 1;
            if self.last_sequence_number.is_some() && msg.sequence_number != self.last_sequence_number.unwrap() + 1 {
                self.out_of_order += 1;
            }
            self.last_sequence_number = Some(msg.sequence_number);
        } else {
            self.bad_count += 1;
        }
    }
}

#[derive(Clone)]
struct Experiment {
    port: u16
}

fn start_monitoring_thread(experiment: Experiment, status: &Arc<Mutex<ExperimentStatus>>) {
    let status = Arc::clone(&status);
    thread::spawn(move || {
        let mut last_status = status.lock().unwrap().clone();
        loop {
            {
                let port_stats = proc_net::get_udp_port_stats(experiment.port).unwrap();
                let status = status.lock().unwrap();
                if *status != last_status {
                    println!("Datagrams: {:8} good {:6} bad {:6} out of order {:6} dropped by us {:6} in rx queue",
                             status.good_count,
                             status.bad_count,
                             status.out_of_order,
                             port_stats.drops,
                             port_stats.rx_queue);
                    last_status = status.clone();
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

