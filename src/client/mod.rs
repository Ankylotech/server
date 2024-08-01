use crate::ais::AI;
use crate::game::{Game, GameState};
use std::net::{SocketAddr, UdpSocket};

pub struct Client<T: Game, U: AI<T>> {
    game: T,
    ai: U,
    socket: UdpSocket,
}

impl<T: Game, U: AI<T>> Client<T, U> {
    pub fn new(game: T, ai: U, addr: SocketAddr) -> std::io::Result<Client<T, U>> {
        let mut socket = UdpSocket::bind("127.0.0.1:34255")?;
        socket.set_nonblocking(false)?;
        socket.set_multicast_loop_v4(false)?;
        socket.connect(addr)?;
        socket.send(ai.get_name().as_bytes())?;
        Ok(Client { game, ai, socket })
    }

    pub fn listen(&mut self) {
        loop {
            let mut update = [0; 32];
            match self.socket.recv(&mut update) {
                Ok(_) => (),
                Err(_) => continue,
            }
            self.game.apply_update(update);
            let next = self.ai.get_next_move(&self.game);
            self.socket
                .send(&T::move_to_network(&self.game, next))
                .expect("Failed to send move");
            if self.game.get_gamestate() != GameState::ONGOING {
                break;
            }
        }
    }
}
