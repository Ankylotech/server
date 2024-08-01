use crate::ais::AI;
use crate::game::{Game, GameState};
use crate::server::PlayerType::{Console, Network};
use std::io;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::{thread, time};

const TURN_TIME: time::Duration = time::Duration::from_millis(500);

pub struct Server<T: Game> {
    players: Vec<Player<T>>,
    game: T,
    socket: UdpSocket,
}

impl<T: Game> Server<T> {
    pub fn start(game: T, mut players: Vec<Player<T>>) -> std::io::Result<Server<T>> {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("Failed to bind to address");
        socket.set_nonblocking(true)?;
        socket.set_multicast_loop_v4(false)?;
        socket.set_broadcast(true)?;
        socket
            .send_to(&T::identifier(), "255.255.255.255:34255")
            .expect("Failed to send message");
        thread::sleep(time::Duration::from_secs(1));
        loop {
            let mut data = [0; 30];
            match socket.recv_from(&mut data) {
                Ok((received, addr)) => players.push(Player {
                    name: String::from_utf8_lossy(&data[..received]).to_string(),
                    player_type: PlayerType::Network(addr),
                }),
                _ => break,
            }
        }
        let mut input_string = String::new();
        for i in players.len()..T::num_players() {
            println!("Input the name of the next player:");
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            players.push(Player {
                name: input_string.trim().to_string(),
                player_type: Console,
            });
        }
        println!(
            "{:?}",
            players
                .iter()
                .map(|p| { &p.name })
                .collect::<Vec<&String>>()
        );
        Ok(Server {
            players,
            game,
            socket,
        })
    }

    pub fn is_ongoing(&self) -> bool {
        return self.game.get_gamestate() == GameState::ONGOING;
    }

    pub fn print_result(&self) {
        self.game.print_state();
        match self.game.get_gamestate() {
            GameState::ONGOING => (),
            GameState::DRAW => println!("The game was a draw"),
            GameState::WINNER(index) => {
                println!("Player {index}: {} won!", self.players[index].name)
            }
        }
    }

    pub fn notify_all(&self) {
        for p in &self.players {
            match p.player_type {
                Network(addr) => {
                    self.socket
                        .send_to(&self.game.update(), addr)
                        .expect("Error while sending update to players");
                }
                _ => (),
            }
        }
    }

    pub fn play_turn(&mut self) {
        if !self.is_ongoing() {
            return;
        }
        self.game.print_state();
        let notify = self.game.players_to_notify();
        println!("getting the next move from {:?}", notify);
        let mut networks = Vec::new();
        let mut console = Vec::new();
        let mut local = Vec::new();
        for i in notify {
            match &self.players[i].player_type {
                PlayerType::Network(addr) => {
                    networks.push((self.players[i].name.clone(), addr, i));
                }
                PlayerType::Console => console.push(self.players[i].name.clone()),
                PlayerType::Local(ai) => local.push((self.players[i].name.clone(), ai)),
            }
        }
        for (_, addr, _) in &networks {
            self.socket
                .send_to(&self.game.update(), addr)
                .expect("Error while sending update to players");
        }
        for (name) in &console {
            self.game.console_move(name);
        }
        for (_, ai) in local {
            self.game.make_move(ai.get_next_move(&self.game));
        }
        if networks.len() > 0 {
            thread::sleep(TURN_TIME);
        }
        let mut recv = vec![false; self.players.len()];
        let mut totals = 0;
        loop {
            let mut data = [0; 30];
            if totals >= networks.len() {
                break;
            }
            match self.socket.recv_from(&mut data) {
                Ok((received, addr)) => {
                    match self.players.iter().position(|p| match p.player_type {
                        Network(address) => address == addr,
                        _ => false,
                    }) {
                        Some(index) => {
                            if !recv[index] {
                                totals += 1;
                                recv[index] = true;
                                println!("received message from player");
                                self.game.network_move(data, received, index)
                            }
                        }
                        None => println!("address did not match"),
                    }
                }
                _ => break,
            }
        }
        loop {
            let mut data = [0; 30];
            match self.socket.recv_from(&mut data) {
                Ok(_) => (),
                _ => break,
            }
        }
        for (name, _, index) in networks {
            if !recv[index] {
                println!("{} took too long an will make the default move", name);
                self.game.default_move(index);
            }
        }
    }
}

pub enum PlayerType<T: Game> {
    Console,
    Network(SocketAddr),
    Local(Box<dyn AI<T>>),
}
pub struct Player<T: Game> {
    name: String,
    player_type: PlayerType<T>,
}

impl<T: Game> Player<T> {
    pub fn new(name: String, player_type: PlayerType<T>) -> Player<T> {
        Player { name, player_type }
    }
}
