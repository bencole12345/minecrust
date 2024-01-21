use std::net;
use std::net::ToSocketAddrs;
use sbs5k_core::backend::Client2Server;

struct RequestProcessor {
    socket: net::UdpSocket,
}

impl RequestProcessor {
    pub fn new(address: impl ToSocketAddrs) -> Self{
        let socket = net::UdpSocket::bind(address).expect("Failed to bind UDP socket");
        Self { socket }
    }

    pub fn main_loop(&self) {
        let mut buf = [0; 1500];
        while let Ok((_, src)) = self.socket.recv_from(&mut buf) {
            // TODO: Reuse the buffer
            let msg: Client2Server = bincode::deserialize(&buf).expect("Failed to deserialize");

            self.handle_msg(&msg,  src);
        }
    }

    fn handle_msg(&self, msg: &Client2Server, client: net::SocketAddr) {
        println!("Received from {}: {:?}", client, msg);

        match msg {
            Client2Server::Login { .. } => {}
            Client2Server::Logout { .. } => {}
            Client2Server::Heartbeat { .. } => {}
            Client2Server::NotifyNewPosition { .. } => {}
            Client2Server::LoadChunksPls { .. } => {}
        }
    }
}
