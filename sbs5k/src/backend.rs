use crate::event;
use crate::event::Event;
use std::{io, net};

use bincode;

use sbs5k_core::backend;

pub(crate) struct BackendConnection {
    sock: net::UdpSocket,
    addr: net::SocketAddr,
    buf: [u8; 1024], // TODO: Don't hard-code
    buf_len: usize,
}

impl BackendConnection {
    pub(crate) fn new(addr: net::SocketAddr) -> io::Result<Self> {
        // Bind using port 0
        let mut adjusted_addr = addr;
        adjusted_addr.set_port(0);

        // TODO: Don't hard-code this buffer size
        // let mut buf = Vec::with_capacity(1024);
        // buf.extend([b'\0'].iter().cycle().take(1024));

        let player_id: u32 = 123; // TODO

        let mut buf = [0; 1024];
        buf[0..4].copy_from_slice(&player_id.to_le_bytes());

        Ok(Self {
            sock: net::UdpSocket::bind(adjusted_addr)?,
            addr,
            buf,
            buf_len: 0,
        })
    }

    fn send_buf() {
        todo!()
        // self.sock
    }
}

impl event::EventListener for BackendConnection {
    fn on_event(&mut self, event: &Event) {
        // TODO: Deal with errors in here better
        // TODO: Try not to heap allocate every time...
        // TODO: For obvious reasons this needs to happen from a separate thread
        match event {
            Event::TranslatePlayer(v) => {
                let msg = backend::Client2Server::NotifyNewLocation;
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                self.sock
                    .send_to(serialized.as_slice(), self.addr)
                    .expect("Failed to send");
            }

            Event::RotatePlayer(v) => {
                let msg = backend::Client2Server::NotifyNewLocation;
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                self.sock.send_to(serialized.as_slice(), self.addr).unwrap();
            }

            Event::PlayerEnteredNewChunk(chunk_coord) => {
                let msg = backend::Client2Server::LoadChunksPls;
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                self.sock
                    .send_to(serialized.as_slice(), self.addr)
                    .expect("Failed to send");
            }

            Event::EndGame => {
                let msg = backend::Client2Server::Disconnected;
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                self.sock
                    .send_to(serialized.as_slice(), self.addr)
                    .expect("Failed to send");
            }

            _ => (),
        }
    }
}
