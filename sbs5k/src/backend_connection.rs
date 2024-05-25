use crate::event;
use crate::event::Event;
use crate::networking;
use crate::networking::OnPacketCallback;
use crate::updatable::Updatable;
use std::{io, net};

use sbs5k_core::backend_api;
use sbs5k_core::backend_api::{c2s, s2c, Client2ServerMsg, Server2ClientMsg};

pub(crate) struct BackendConnection {
    conn: networking::AsyncConnection,
    event_submitter: event::EventSubmitter,
    our_player_id: Option<backend_api::PlayerID>,
}

impl BackendConnection {
    pub(crate) fn new(
        addr: net::SocketAddr,
        username: String,
        event_submitter: event::EventSubmitter,
    ) -> io::Result<Self> {
        let mut conn = networking::AsyncConnection::new(addr);

        // Send the login message
        let buf = Client2ServerMsg::Login(c2s::LoginMsg { username }).to_buf();
        conn.send(buf.as_slice());

        // TODO: Delay startup until login is complete, we need to know our player ID before we can do anything meaningful
        // (maybe make it non-optional)

        Ok(Self {
            conn,
            our_player_id: None,
            event_submitter,
        })
    }

    fn handle_on_login_success(&mut self, msg: s2c::OnLoginSuccessMsg) {
        self.our_player_id = Some(msg.token);
        // TODO: Improve API for setting initial position; ideally the driver would wait until we've had this response
        self.event_submitter
            .submit_event(Event::PlayerChangedPosition(msg.start_position));
    }

    fn handle_on_login_rejected(&mut self, msg: s2c::OnLoginRejectedMsg) {
        panic!("Login rejected with: {}", msg.reason);
    }

    fn handle_chunk_loaded(&mut self, msg: s2c::ChunkLoadedUrWelcomeMsg) {
        self.event_submitter
            .submit_event(Event::ChunkLoaded(msg.chunk_load_result));
    }
}

impl event::EventListener for BackendConnection {
    fn on_event(&mut self, event: &Event) {
        // TODO: Deal with errors in here better
        // TODO: Try not to heap allocate every time...
        match event {
            Event::PlayerChangedPosition(new_position) => {
                let buf = Client2ServerMsg::NotifyNewPosition(c2s::NotifyNewPositionMsg {
                    player_id: self.our_player_id.unwrap_or(0),
                    position: *new_position,
                })
                .to_buf();
                self.conn.send(buf.as_slice()); // TODO: Move the serialized vec in?
            }

            Event::PlayerEnteredNewChunk(coordinate) => {
                // TODO: This is going to need work, it needs to work out the range of chunks to
                // request just like the single-process loader does
                let buf = Client2ServerMsg::LoadChunksPls(c2s::LoadChunksPlsMsg {
                    token: 0,
                    coordinate: *coordinate,
                })
                .to_buf();
                self.conn.send(buf.as_slice());
            }

            Event::EndGame => {
                let buf = Client2ServerMsg::Logout(c2s::LogoutMsg {
                    player_id: self.our_player_id.unwrap_or(0),
                })
                .to_buf();
                self.conn.send(buf.as_slice());
            }

            _ => (),
        }
    }
}

impl Updatable for BackendConnection {
    fn update(&mut self, dt: f32) {
        // TODO: It would be nice to use the on_packet callback directly for this. One for when I
        // have an internet connection again
        while let Some(payload) = self.conn.try_recv() {
            self.on_packet(payload.as_slice());
        }
        self.conn.update(dt);
    }
}

impl OnPacketCallback for BackendConnection {
    fn on_packet(&mut self, data: &[u8]) {
        let msg = Server2ClientMsg::from_buf(data);
        println!("Received {:?}", msg);

        match msg {
            Server2ClientMsg::OnLoginSuccess(msg) => self.handle_on_login_success(msg),
            Server2ClientMsg::OnLoginRejected(msg) => self.handle_on_login_rejected(msg),
            Server2ClientMsg::ChunkLoadedUrWelcome(msg) => self.handle_chunk_loaded(msg),
        }
    }
}
