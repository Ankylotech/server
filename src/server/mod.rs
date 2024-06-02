use std::net::UdpSocket;
use std::net::SocketAddr;
use std::{thread,time};
use std::cmp::PartialEq;
use std::io;
use crate::game::{Game, GameState};
use crate::server::PlayerType::Console;

const TURN_TIME: time::Duration = time::Duration::from_millis(100);

pub struct Server<T: Game> {
    players: Vec<Player>,
    game: T,
    socket: UdpSocket
}

impl<T: Game> Server<T> {
    pub fn start(game: T) -> std::io::Result<Server<T>>{
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("Failed to bind to address");
        socket.set_nonblocking(true)?;
        socket.set_multicast_loop_v4(false)?;
        socket.set_broadcast(true)?;
        socket.send_to(&T::identifier(), "224.0.0.0:4").expect("Failed to send message");
        thread::sleep(time::Duration::from_secs(1));
        let mut players = Vec::new();
        loop {
            let mut data = [0;30];
            match socket.recv_from(&mut data) {
                Ok((received, addr)) => players.push(Player {name: String::from_utf8_lossy(&data[..received]).to_string(), player_type: PlayerType::Network(addr)}),
                _ => break
            }
        }
        let mut input_string = String::new();
        for i in players.len()..T::num_players() {
            println!("Input the name of the next player:");
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            players.push(Player {name: input_string.trim().to_string(), player_type: Console});
        }
        println!("{:?}", players);
        Ok(Server {players, game, socket})
    }

    pub fn is_ongoing(&self) -> bool {
        return self.game.get_gamestate() == GameState::ONGOING;
    }

    pub fn print_result(&self) {
        match self.game.get_gamestate() {
            GameState::ONGOING => (),
            GameState::DRAW => println!("The game was a draw"),
            GameState::WINNER(index) => println!("Player {index}: {} won!", self.players[index].name),
        }
    }

    pub fn play_turn(&mut self) {
        if !self.is_ongoing() {
            return;
        }
        let notify = self.game.players_to_notify();
        let mut networks = Vec::new();
        let mut console = Vec::new();
        let mut local = Vec::new();
        for i in notify {
            match self.players[i].player_type {
                PlayerType::Network(addr) => {
                    networks.push((self.players[i].name.clone(), addr));
                },
                PlayerType::Console => console.push(self.players[i].name.clone()),
                PlayerType::Local => local.push(self.players[i].name.clone()),
            }
        }
        for (_, addr) in networks {
            self.socket.send_to(&self.game.update(),addr).expect("Error while sending update to players");
        }
        for (name) in &console {
            self.game.console_move(name);
        }
        if (console.len() == 0) {
            thread::sleep(TURN_TIME);
        }
        loop {
            let mut data = [0; 30];
            match self.socket.recv_from(&mut data) {
                Ok((received, addr)) => {
                    match self.players.iter().position(|p| p.player_type == PlayerType::Network(addr)) {
                        Some(index) => self.game.network_move(data, received, index),
                        None => ()
                    }
                }
                _ => break,
            }
        }
        // TODO: local moves
    }
}
#[derive(Debug, PartialEq)]
enum PlayerType {
    Console,
    Network(SocketAddr),
    Local // TODO: relevant data for local players
}
#[derive(Debug)]
struct Player {
    name: String,
    player_type: PlayerType,
}