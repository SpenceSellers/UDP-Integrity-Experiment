mod client;
mod server;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).unwrap_or(&"client".to_owned()) == "client" {
        println!("Client starting.");
        client::run_test_client();
    } else {
        println!("Server starting.");
        server::run_test_server();
    }
}

