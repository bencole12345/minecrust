use std::sync::mpsc;

/// Type for a bidirectional stream with an associated request and response type.
pub type BiDirectionalStream<Request, Response> = (mpsc::Sender<Request>, mpsc::Receiver<Response>);
