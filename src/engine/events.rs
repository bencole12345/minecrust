use crate::engine::inputs;

/// An event from the runtime environment
pub enum Event {
    KeyPress(inputs::Key),
    KeyRelease(inputs::Key),
    // TODO: MouseEvent
}

/// A source of events to be processed
pub trait EventSource {
    // TODO: Make this return an iterator over Events
    fn poll_events(&mut self) -> Vec<Event>;
}
