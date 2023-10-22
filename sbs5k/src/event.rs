use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use nalgebra as na;

use sbs5k_core::{chunk, geometry};

use crate::loading;

pub(crate) enum Event {
    EndGame,

    ChunkLoaded(loading::ChunkLoadResult),

    TranslatePlayer(na::Vector3<f32>),
    RotatePlayer(geometry::OrientationDelta),

    PlayerEnteredNewChunk(chunk::ChunkCoordinate),
    // TODO: MoveOtherPlayer, RotateOtherPlayer once we have multiplayer
}

#[derive(Clone)]
pub(crate) struct EventSubmitter {
    tx_handle: mpsc::Sender<Event>,
}

impl EventSubmitter {
    pub fn submit_event(&self, event: Event) {
        self.tx_handle.send(event).expect("Failed to add to queue");
    }
}

pub(crate) trait EventListener {
    fn on_event(&mut self, event: &Event);
}

pub(crate) struct EventQueue {
    tx_handle: mpsc::Sender<Event>,
    rx_handle: mpsc::Receiver<Event>,
    listeners: Vec<Rc<RefCell<dyn EventListener>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        let (tx_handle, rx_handle) = mpsc::channel();
        let listeners = Vec::new();
        Self {
            tx_handle,
            rx_handle,
            listeners,
        }
    }

    pub fn add_listener(&mut self, listener: Rc<RefCell<dyn EventListener>>) {
        self.listeners.push(listener);
    }

    pub fn get_submitter(&self) -> EventSubmitter {
        EventSubmitter {
            tx_handle: Clone::clone(&self.tx_handle),
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
