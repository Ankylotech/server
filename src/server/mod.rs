use std::net::UdpSocket;
use std::net::SocketAddr;
use std::{thread,time};

struct Server<T: Game> {
    players: Vec<Player>,
    game: T,
    socket: UdpSocket
}

impl Server<T: Game> {
    pub fn init(game: T) -> Result<(Server<T>,)> {
        let socket = UdpSocket::bind("127.0.0.1")?;
        socket.set_nonblocking(true)?;
        socket.set_multicast_loop_v4(false)?;
        socket.set_broadcast(true)?;
        socket.send_to([127;10], "255.255.255.255");
        thread::sleep(time::Duration::from_seconds(1));
        let mut players = Vec::new();
        loop {
            let mut data = [0;30];
            match socket.recv_from(&mut data) {
                Ok((received, addr)) => players.append(Player {name: String::from_utf8_lossy(data[..received]), ip: addr}),
                _ => break
            }
        }
        println("{}", players);
    }



}


struct Player {
    name: String,
    ip: SocketAddr,
}