use std::net::{ SocketAddr, UdpSocket};
use crate::game::Game;

pub struct Client<T:Game, U:AI<T>> {
    game: T,
    ai: U,
    socket: UdpSocket,
}

impl<T:Game, U:AI<T>> Client<T,U> {
    pub fn new(game: T, ai: U, addr: SocketAddr) -> Client<T,U> {
        let mut socket = UdpSocket::new("localhost");
        socket.set_nonblocking(false)?;
        socket.set_multicast_loop_v4(false)?;
        socket.connect(addr)?;
        Client {game, ai, socket}
    }

    pub fn listen(&mut self) {
        loop {
            let mut update = [0;32];
            match self.socket.recv(&mut update) {
                Ok(_) => (),
                Err(_) => continue,
            }
            self.game.apply_update(update);
            let next = self.ai.get_next_move(&self.game);
            self.socket.send(self.game.move_to_network(next)).expect("Failed to send move");
        }
    }

}

pub trait AI<T:Game> {
    fn get_next_move(self: &Self, state: &T) -> T::Move;
}