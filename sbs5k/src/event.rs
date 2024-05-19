use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{mpsc, Arc, RwLock};

use sbs5k_core::{chunk, geometry};

pub(crate) enum Event {
    EndGame,

    ChunkLoaded(chunk::ChunkLoadResult),

    PlayerChangedPosition(geometry::EntityPosition),
    PlayerEnteredNewChunk(chunk::ChunkCoordinate), // TODO: Get rid, this can be inferred from listening to PlayerChangedPosition messages

                                                   // TODO: MoveOtherPlayer, RotateOtherPlayer once we have multiplayer
}

#[derive(Clone)]
pub(crate) struct EventSubmitter {
    tx_handle: mpsc::Sender<Event>,
    is_live: Arc<RwLock<bool>>,
}

impl EventSubmitter {
    pub fn submit_event(&self, event: Event) {
        let res = self.tx_handle.send(event);
        if res.is_err() {
            let live = *self.is_live.read().unwrap();
            if live {
                panic!("Failed to write event while game is live");
            }
        }
    }
}

pub(crate) trait EventListener {
    fn on_event(&mut self, event: &Event);
}

pub(crate) struct EventQueue {
    tx_handle: mpsc::Sender<Event>,
    rx_handle: mpsc::Receiver<Event>,
    listeners: Vec<Rc<RefCell<dyn EventListener>>>,
    is_live: Arc<RwLock<bool>>,
}

impl EventQueue {
    pub fn new(is_live: Arc<RwLock<bool>>) -> Self {
        let (tx_handle, rx_handle) = mpsc::channel();
        let listeners = Vec::new();
        Self {
            tx_handle,
            rx_handle,
            listeners,
            is_live,
        }
    }

    pub fn add_listener(&mut self, listener: Rc<RefCell<dyn EventListener>>) {
        self.listeners.push(listener);
    }

    pub fn get_submitter(&self) -> EventSubmitter {
        EventSubmitter {
            tx_handle: Clone::clone(&self.tx_handle),
            is_live: Clone::clone(&self.is_live),
        }
    }

    pub fn dispatch_all_events(&mut self) {
        while let Ok(event) = self.rx_handle.try_recv() {
            for listener in &mut self.listeners {
                listener.borrow_mut().on_event(&event);
            }
        }
    }
}
