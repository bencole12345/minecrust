use crate::backing_store::BackingStore;
use crate::server::{GlobalStateHandle, PlayerStateHandle, PlayerStateMapHandle};
use crate::state::PlayerState;
use log::{error, info, trace};
use sbs5k_core::backend_api::{c2s, s2c, Client2ServerMsg, PlayerID, Server2ClientMsg};
use sbs5k_core::chunk::ChunkLoadResult;
use sbs5k_core::geometry;
use std::sync::Arc;
use std::{io, net};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};

pub(crate) struct ClientTask {
    is_logged_in: bool,
    assigned_id: PlayerID,
    socket: TcpStream,

    global_state: GlobalStateHandle,
    player_specific_state: PlayerStateMapHandle,
    // TODO: Rework so that this isn't optional (might require splitting ClientTask into a
    // session-layer and an application-layer component
    player_state: Option<PlayerStateHandle>,
    // TODO: Similarly this should never start off empty, it should be constructed at the same time
    // as the rest of an application-level struct
    username: String,

    backing_store: Arc<Mutex<dyn BackingStore + Sync + Send>>,
}

impl ClientTask {
    pub fn new(
        socket: TcpStream,
        global_state: GlobalStateHandle,
        player_specific_state: PlayerStateMapHandle,
        backing_store: Arc<Mutex<dyn BackingStore + Sync + Send>>,
    ) -> Self {
        Self {
            is_logged_in: false,
            assigned_id: 0,
            socket,
            global_state,
            player_specific_state,
            player_state: None,
            username: String::default(),
            backing_store,
        }
    }

    #[inline(always)]
    pub fn ip(&self) -> net::SocketAddr {
        self.socket.peer_addr().unwrap()
    }

    pub async fn run(&mut self) {
        let mut buf = Vec::new();

        // TODO: Handle exiting this loop cleanly when server is stopped
        loop {
            // Read the size
            match self.socket.read_u16().await {
                Ok(len) => {
                    // TODO: Double-check this isn't causing a load of deallocations and reallocations
                    buf.resize(len as usize, 0);
                }
                Err(_) => {
                    // Let's drop the connection, maybe log and exit this loop?
                    todo!()
                }
            };

            // Read the message
            match self.socket.read_exact(&mut buf).await {
                Ok(0) => {
                    // Connection closed
                    todo!()
                }
                Ok(len) => {
                    assert_eq!(buf.len(), len);
                    let msg = Client2ServerMsg::from_buf(&buf);
                    self.handle_msg(&msg).await;
                }
                Err(_) => {
                    // Handle error
                    todo!()
                }
            }
        }
    }

    #[rustfmt::skip]
    async fn handle_msg(&mut self, msg: &Client2ServerMsg) {
        trace!("Received from {}: {:?}", self.ip(), msg);

        let result = match msg {
            Client2ServerMsg::Login(msg) => self.handle_login(msg).await,
            Client2ServerMsg::Logout(msg) => self.handle_logout(msg).await,
            Client2ServerMsg::NotifyNewPosition(msg) => self.handle_notify_new_position(msg).await,
            Client2ServerMsg::LoadChunksPls(msg) => self.handle_load_chunks_pls(msg).await,
        };

        if let Err(err) = result {
            error!("Error while processing request: msg={:?}, err={:?}", msg, err)
        }
    }

    async fn handle_login(&mut self, msg: &c2s::LoginMsg) -> io::Result<()> {
        if self.is_logged_in {
            error!(
                "Unexpected login message for already logged-in client: [player_id={}, ip={}]",
                self.assigned_id,
                self.ip()
            );
        }

        // Retrieve the player's state if the player with this username has connected previously,
        // or generate new initial state for them if we've not seen them before
        let player_state_arc = if let Some(state) = self
            .backing_store
            .lock()
            .await
            .load_player_state(&msg.username)
        {
            info!("Logging in known player {}", msg.username);
            Arc::new(RwLock::new(state))
        } else {
            info!(
                "Performing first-time setup for new player: {}",
                msg.username
            );
            Arc::new(RwLock::new(PlayerState {
                username: msg.username.clone(),
                current_position: geometry::EntityPosition::default(),
            }))
        };
        self.username = msg.username.clone();

        let response = {
            // Grab all the locks
            let mut global_state = self.global_state.blocking_write();
            let mut player_specific_state_map = self.player_specific_state.blocking_write();
            let player_state = player_state_arc.blocking_read();

            let player_id = global_state.assign_player_id();

            // Update the global map of usernames to PlayerIDs
            if let Some(v) = global_state
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
                self.socket.write_all(response.as_slice()).await?;
                return Ok(());
            }

            // Add this `PlayerState` entry to the global `PlayerID` -> `PlayerState` map
            if let Some(_) = player_specific_state_map.insert(player_id, player_state_arc.clone()) {
                unreachable!()
            }

            info!(
                "Player {} logged in [PlayerID: {}]",
                msg.username, player_id
            );

            let response = Server2ClientMsg::OnLoginSuccess(s2c::OnLoginSuccessMsg {
                start_position: player_state.current_position,
            })
            .to_buf();
            response
        };

        // Setting it here avoids some faff with accessing moved values
        self.player_state = Some(player_state_arc);

        self.socket.write_all(response.as_slice()).await?;
        Ok(())
    }

    async fn handle_logout(&mut self, _msg: &c2s::LogoutMsg) -> io::Result<()> {
        let mut player_states_map = self.player_specific_state.blocking_write();

        if let Some(_) = player_states_map.remove(&self.assigned_id) {
            let mut global_state = self.global_state.blocking_write();
            global_state.player_username_to_id.remove(&self.username);
            // Forcibly flush state since changes are only written periodically
            // TODO: Restore once TCP refactor is complete
            // self.backing_store.blocking_lock().save_player_state(v);
        } else {
            unreachable!()
        }
        Ok(())
    }

    async fn handle_notify_new_position(
        &mut self,
        msg: &c2s::NotifyNewPositionMsg,
    ) -> io::Result<()> {
        // TODO: Get rid of option type by factoring out an application-level struct
        if let Some(state) = &self.player_state {
            let mut player_state = state.blocking_write();
            player_state.current_position = msg.position;
        } else {
            error!("Unexpected NotifyNewPositionMsg from {}", self.ip());
        }

        Ok(())
    }

    async fn handle_load_chunks_pls(&mut self, msg: &c2s::LoadChunksPlsMsg) -> io::Result<()> {
        let response = {
            let backing_store = self.backing_store.blocking_lock();
            let chunk = if let Some(v) = backing_store.load_chunk(msg.coordinate) {
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
            response
        };

        self.socket.write_all(response.as_slice()).await?;
        Ok(())
    }
}
