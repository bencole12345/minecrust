use std::{io, net};

use tokio::net::UdpSocket;

use sbs5k_core::backend::Client2Server;

pub struct RequestProcessor {
    socket: UdpSocket,
}

impl RequestProcessor {
    pub async fn new(address: &str) -> Self {
        println!("Listening on {}", address);
        let socket = UdpSocket::bind(address)
            .await
            .expect("Failed to bind UDP socket");
        Self { socket }
    }

    pub async fn main_loop(&self) -> io::Result<()> {
        // TODO: Handle exiting this loop cleanly
        let mut buf = [0; 1500];
        loop {
            let (_, src) = self.socket.recv_from(&mut buf).await?;

            // TODO: Reuse the buffer
            let msg: Client2Server = bincode::deserialize(&buf).expect("Failed to deserialize");

            self.handle_msg(&msg, src);
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
