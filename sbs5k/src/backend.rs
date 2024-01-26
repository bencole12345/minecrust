use crate::event;
use crate::event::Event;
use std::{io, net};

use sbs5k_core::backend;
use sbs5k_core::networking;
use sbs5k_core::networking::OnPacketCallback;

pub(crate) struct BackendConnection {
    conn: networking::AsyncConnection,
}

impl BackendConnection {
    pub(crate) fn new(addr: net::SocketAddr) -> io::Result<Self> {
        // Bind using port 0
        let mut adjusted_addr = addr;
        adjusted_addr.set_port(0);
        let conn = networking::AsyncConnection::new(adjusted_addr);
        Ok(Self { conn })
    }
}

impl OnPacketCallback for BackendConnection {
    fn on_packet(&mut self, data: &[u8]) {
        todo!()
    }
}

impl event::EventListener for BackendConnection {
    fn on_event(&mut self, event: &Event) {
        // TODO: Deal with errors in here better
        // TODO: Try not to heap allocate every time...
        match event {
            Event::PlayerChangedPosition(new_position) => {
                let msg = backend::Client2Server::NotifyNewPosition {
                    token: 0,
                    position: *new_position,
                };
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                println!("Sending PlayerChangedPosition");
                self.conn.send(serialized.as_slice()); // TODO: Move the serialized vec in?
            }

            Event::PlayerEnteredNewChunk(coordinate) => {
                let msg = backend::Client2Server::LoadChunksPls {
                    token: 0,
                    coordinate: *coordinate,
                };
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                self.conn.send(serialized.as_slice());
            }

            Event::EndGame => {
                let msg = backend::Client2Server::Logout { token: 0 };
                let serialized = bincode::serialize(&msg).expect("Failed to serialize");
                self.conn.send(serialized.as_slice());
            }

            _ => (),
        }
    }
}
