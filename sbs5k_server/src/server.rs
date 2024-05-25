use log::info;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use sbs5k_core::backend_api::PlayerID;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, RwLock};

use crate::backing_store::BackingStore;
use crate::client_task::ClientTask;
use crate::state::{GlobalServerState, PlayerState};

pub(crate) type GlobalStateHandle = Arc<RwLock<GlobalServerState>>;
pub(crate) type PlayerStateHandle = Arc<RwLock<PlayerState>>;
pub(crate) type PlayerStateMapHandle = Arc<RwLock<HashMap<PlayerID, PlayerStateHandle>>>;

pub(crate) struct Server {
    // TODO: Make this a SocketAddr
    address: String,

    /// Global state, containing server-wide state
    global_state: GlobalStateHandle,

    /// Global mapping from PlayerID -> player-specific state
    ///
    /// Separating this from the global state makes it possible for concurrent modification of
    /// different players, while still retaining the possibility to scan across all player state,
    /// which is needed for initial connections.
    global_player_specific_state: PlayerStateMapHandle,

    // TODO: Think about this; ideally we'd use a rw lock so that we can load existing chunks fine
    // and only acquire a write lock to serialiase creation of new chunks
    backing_store: Arc<Mutex<dyn BackingStore + Sync + Send>>,
}

impl Server {
    pub async fn new(
        address: &str,
        backing_store: Arc<Mutex<dyn BackingStore + Sync + Send>>,
    ) -> Self {
        Self {
            address: String::from(address),
            global_state: GlobalStateHandle::default(),
            global_player_specific_state: PlayerStateMapHandle::default(),
            backing_store,
        }
    }

    pub async fn main_loop(&mut self) -> io::Result<()> {
        // TODO: Set up timer to write to backing store every N seconds, where N is a configurable
        // value, probably all inside a tokio::select
        // TODO: Install signal handler to write to store before exit
        // TODO: clean loop exit

        let listener = TcpListener::bind(&self.address)
            .await
            .expect("Failed to bind TCP listener");
        info!("Listening on {}", self.address);

        loop {
            let (socket, addr) = listener.accept().await?;
            info!("New connection from {}", addr);

            let global_state_clone = self.global_state.clone();
            let player_specific_state_clone = self.global_player_specific_state.clone();
            let backing_store_clone = self.backing_store.clone();

            // TODO: Confirm I don't need to save the JoinHandle this returns. Looks like there's
            // also the JoinSet type
            tokio::spawn(async move {
                let mut task = ClientTask::new(
                    socket,
                    global_state_clone,
                    player_specific_state_clone,
                    backing_store_clone,
                );
                task.run().await;
            });
        }
    }
}
