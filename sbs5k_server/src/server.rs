use std::collections::HashMap;
use std::net::SocketAddr;

use std::io;

use tokio::net::UdpSocket;

use sbs5k_core::player;

use crate::args;

const PACKET_MAX_SIZE: usize = 1500;

pub(crate) struct Server {
    pub listen_addr: SocketAddr,

    running: bool,
    clients_map: HashMap<SocketAddr, player::PlayerId>,
}

impl Server {
    pub(crate) fn new(args: args::Args) -> Self {
        // let addr: SocketAddr = args.endpoint;
        println!("Endpoint: {}", args.endpoint);

        // TODO: don't crash if we fail to parse
        // let addr = SocketAddr::from_str(&args.endpoint).expect("Invalid endpoint");
        let addr = args.endpoint.parse().expect("Invalid endpoint");

        Self {
            listen_addr: addr,
            running: false,
            clients_map: HashMap::new(),
        }
    }

    pub(crate) async fn main_loop(&self) -> io::Result<()> {
        let socket = UdpSocket::bind(self.listen_addr).await?;
        let mut buf = [0; PACKET_MAX_SIZE];

        println!("Listening on {}", self.listen_addr);

        'recv_loop: loop {
            let (size, src) = socket.recv_from(&mut buf).await?;
            println!("Received {} bytes from {}", size, src);

            let should_halt = self.on_datagram(&buf[..size]);

            if should_halt {
                break 'recv_loop;
            }
        }

        Ok(())
    }

    fn on_datagram(&self, data: &[u8]) -> bool {
        println!("Received data: {:?}", data);

        false
    }
}
