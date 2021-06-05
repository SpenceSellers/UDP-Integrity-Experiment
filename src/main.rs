mod client;
mod server;
mod messages;
mod proc_net;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    //"192.168.88.35:3555";
    if args.get(1).unwrap_or(&"server".to_owned()) == "client" {
        println!("Client starting.");
        client::run_test_client(args.get(2).expect("Must provide a destination IP and port, such as 192.168.1.35:3555"));
    } else {
        println!("Server starting.");
        server::run_test_server(args.get(2).unwrap_or(&"3555".to_owned()));
    }
}

