use log::{error, info, trace};
use sbs5k_core::chunk::ChunkLoadResult;
use std::{collections::HashMap, io, net};

use tokio::net::UdpSocket;

use sbs5k_core::backend::{c2s, s2c, Client2ServerMsg, PlayerID, Server2ClientMsg};
use sbs5k_core::geometry;

use crate::backing_store::BackingStore;
use crate::player_state::PlayerState;

pub struct RequestProcessor {
    socket: UdpSocket,

    player_states: HashMap<PlayerID, PlayerState>,
    player_username_to_id: HashMap<String, PlayerID>,

    backing_store: Box<dyn BackingStore>,
}

impl RequestProcessor {
    pub async fn new(address: &str, backing_store: Box<dyn BackingStore>) -> Self {
        info!("Listening on {}", address);
        let socket = UdpSocket::bind(address)
            .await
            .expect("Failed to bind UDP socket");
        Self {
            socket,
            player_states: HashMap::new(),
            player_username_to_id: HashMap::new(),
            backing_store,
        }
    }

    pub async fn main_loop(&mut self) -> io::Result<()> {
        // TODO: Set up timer to write to backing store every N seconds, where N is a configurable value

        // TODO: Handle exiting this loop cleanly
        let mut buf = [0; 1500];
        loop {
            let (_, src) = self.socket.recv_from(&mut buf).await?;
            let msg = Client2ServerMsg::from_buf(&buf);
            self.handle_msg(&msg, src).await;
        }
    }

    #[rustfmt::skip]
    async fn handle_msg(&mut self, msg: &Client2ServerMsg, client: net::SocketAddr) {
        trace!("Received from {}: {:?}", client, msg);

        let result = match msg {
            Client2ServerMsg::Login(msg) => self.handle_login(msg, client).await,
            Client2ServerMsg::Logout(msg) => self.handle_logout(msg, client).await,
            Client2ServerMsg::NotifyNewPosition(msg) => self.handle_notify_new_position(msg, client).await,
            Client2ServerMsg::LoadChunksPls(msg) => self.handle_load_chunks_pls(msg, client).await,
        };

        if let Err(err) = result {
            error!("Error while processing request: msg={:?}, err={:?}", msg, err)
        }
    }

    async fn handle_login(
        &mut self,
        msg: &c2s::LoginMsg,
        client: net::SocketAddr,
    ) -> io::Result<()> {
        // Do we already have state for this player?
        let state = if let Some(state) = self.backing_store.load_player_state(&msg.username) {
            info!("Logging in known player {}", msg.username);
            state
        } else {
            info!(
                "Performing first-time setup for new player: {}",
                msg.username
            );
            PlayerState {
                username: msg.username.clone(),
                current_position: geometry::EntityPosition::default(),
            }
        };

        // TODO: Generate a UUID or something
        let player_id = 1234;

        // TODO: How is this thread-safe? Wouldn't this need a lock, assuming Tokio has multiple threads?
        if let Some(v) = self
            .player_username_to_id
            .insert(msg.username.clone(), player_id)
        {
            error!(
                "Player {} requested to log in but is already connected [PlayerID: {}], rejecting",
                msg.username, v
            );
            let response = Server2ClientMsg::OnLoginRejected(s2c::OnLoginRejectedMsg {
                reason: String::from("Username already in use"),
            })
                .to_buf();
            self.socket.send_to(response.as_slice(), client).await?;
            return Ok(());
        }

        info!(
            "Player {} logged in [PlayerID: {}]",
            msg.username, player_id
        );

        let response = Server2ClientMsg::OnLoginSuccess(s2c::OnLoginSuccessMsg {
            token: player_id,
            start_position: state.current_position,
        })
            .to_buf();
        self.player_states.insert(player_id, state); // TODO: Ensure no collisions
        self.socket.send_to(response.as_slice(), client).await?;

        Ok(())
    }

    async fn handle_logout(
        &mut self,
        msg: &c2s::LogoutMsg,
        _client: net::SocketAddr,
    ) -> io::Result<()> {
        if let Some(v) = self.player_states.remove(&msg.player_id) {
            self.player_username_to_id.remove(&v.username);
            self.backing_store.save_player_state(v); // Forcibly flush state since changes are only written periodically
        } else {
            error!("Received Logout for unknown player ID: {}", msg.player_id);
        }
        Ok(())
    }

    async fn handle_notify_new_position(
        &mut self,
        msg: &c2s::NotifyNewPositionMsg,
        _client: net::SocketAddr,
    ) -> io::Result<()> {
        if let Some(v) = self.player_states.get_mut(&msg.player_id) {
            v.current_position = msg.position;
        } else {
            error!(
                "Received NotifyNewPosition for unknown player ID: {}",
                msg.player_id
            );
        }
        Ok(())
    }

    async fn handle_load_chunks_pls(
        &mut self,
        msg: &c2s::LoadChunksPlsMsg,
        client: net::SocketAddr,
    ) -> io::Result<()> {
        let chunk = if let Some(v) = self.backing_store.load_chunk(msg.coordinate) {
            v
        } else {
            todo!()
        };

        let response = Server2ClientMsg::ChunkLoadedUrWelcome(s2c::ChunkLoadedUrWelcomeMsg {
            chunk_load_result: ChunkLoadResult {
                chunk: chunk.clone(), // TODO: avoid needless copy
                coordinate: msg.coordinate,
            },
        })
            .to_buf();
        self.socket.send_to(response.as_slice(), client).await?;

        Ok(())
    }
}
