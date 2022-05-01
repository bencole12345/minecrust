use crate::inputs;

/// An event from the runtime environment
pub enum Event {
    /// A key was pressed
    KeyPress(inputs::Key),

    /// A key was released
    KeyRelease(inputs::Key),

    /// The mouse was moved by (dx, dy) proportion of the screen's dimensions
    MouseMove(f64, f64),
}

/// A source of events to be processed
pub trait EventSource {
    fn poll_events(&mut self) -> Vec<Event>;
}
