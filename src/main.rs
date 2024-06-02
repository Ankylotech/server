use std::env;
mod server;
mod client;
mod game;
use server::Server;
use game::hearts::Hearts;
use crate::game::tictactoe::TicTacToe;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut server = Server::start(TicTacToe::new()).expect("Failed to initialize server");
    while server.is_ongoing() {
        server.play_turn();
    }
    server.print_result();
}
