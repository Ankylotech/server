use std::net::UdpSocket;
use std::net::SocketAddr;
use std::{thread,time};
use crate::game::Game;

pub struct Server<T: Game> {
    players: Vec<Player>,
    game: T,
    socket: UdpSocket
}

impl<T: Game> Server<T> {
    pub fn start(game: T) -> std::io::Result<()>{
        let socket = UdpSocket::bind("127.0.0.1")?;
        socket.set_nonblocking(true)?;
        socket.set_multicast_loop_v4(false)?;
        socket.set_broadcast(true)?;
        socket.send_to(&T::identifier(), "255.255.255.255")?;
        thread::sleep(time::Duration::from_secs(1));
        let mut players = Vec::new();
        loop {
            let mut data = [0;30];
            match socket.recv_from(&mut data) {
                Ok((received, addr)) => players.push(Player {name: String::from_utf8_lossy(&data[..received]).to_string(), ip: addr}),
                _ => break
            }
        }
        println!("{:?}", players);
        Ok(())
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    ip: SocketAddr,
}