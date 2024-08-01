use std::env;
use std::net::{Ipv4Addr, UdpSocket};

mod ais;
mod client;
mod game;
mod server;

use crate::ais::test::TicTacToeAI;
use crate::ais::AI;
use crate::client::Client;
use crate::game::tictactoe::TicTacToe;
use crate::game::Game;
use crate::server::Player;
use crate::server::PlayerType;
use server::Server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"client".to_string()) {
        let mut socket =
            UdpSocket::bind("255.255.255.255:34255").expect("failed to bind to localhost");
        socket.set_nonblocking(false).expect("nonblocking Error");
        let mut buf = [0; 10];
        let mut addr;
        println!("Wait for connection");
        match socket.recv_from(&mut buf) {
            Ok((received, address)) => addr = address,
            Err(err) => return,
        }
        println!("successfully connected");
        if buf == TicTacToe::identifier() {
            let mut client = Client::new(TicTacToe::new(), TicTacToeAI, addr).unwrap();
            client.listen();
        } else {
            println!("Identifier does not have a matching AI");
        }
    } else {
        let ai = TicTacToeAI;
        let locals = vec![Player::new(ai.get_name(), PlayerType::Local(Box::new(ai)))];
        let mut server =
            Server::start(TicTacToe::new(), Vec::new()).expect("Failed to initialize server");
        while server.is_ongoing() {
            server.play_turn();
        }
        server.notify_all();
        server.print_result();
    }
}
