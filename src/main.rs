use std::env;
mod server;
mod client;
mod game;
use server::Server;
use game::hearts::Hearts;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"server".to_string()) {
        Server::start(Hearts::init());
    } else {
        println!("Starting locally");
    }
}
