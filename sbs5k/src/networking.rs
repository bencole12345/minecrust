use crate::updatable::Updatable;
use std::time::Duration;
use std::{net, thread};
use tokio::net::UdpSocket;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc;
use tokio::time;

pub trait OnPacketCallback {
    fn on_packet(&mut self, data: &[u8]);
}

pub type Payload = Vec<u8>;

/// A network thread suitable for use in the game client.
///
/// It sets up a suitable Tokio runtime, with queues into and out from the background thread.
/// There's also a timer, to be used for retransmission when it's needed.
struct NetworkThread {
    dest: net::SocketAddr,
    runtime: Runtime,
}

impl NetworkThread {
    pub fn new(dest: net::SocketAddr) -> Self {
        let runtime = Builder::new_current_thread()
            .thread_name("network_thread")
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        Self { dest, runtime }
    }

    pub fn run(&mut self, mut send_rx: mpsc::Receiver<Payload>, recv_tx: mpsc::Sender<Payload>) {
        let dest = self.dest;

        self.runtime.block_on(async move {
            println!("Running in Tokio background thread");

            // Let the OS pick a port for us
            // TODO: Understand why we need 0.0.0.0, I forgot
            let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
            let mut buf = [0; 1500]; // TODO: Think about size

            let mut interval = time::interval(Duration::from_millis(10));

            loop {
                tokio::select! {
                    // Received a packet from the main thread, let's send it over the network
                    Some(payload) = send_rx.recv() => {
                        socket.send_to(payload.as_slice(), dest).await.expect("Failed to send packet");
                    },

                    // Received a packet from the network, let's pass it back to the main thread
                    Ok((_len, _)) = socket.recv_from(&mut buf) => {
                        recv_tx.send(Vec::from(buf)).await.expect("Failed to write received packed to queue");
                    },

                    // Timer fired, let's check for any packets we need to resend
                    _ = interval.tick() => {
                        // TODO: Revisit this when I'm implementing reliable delivery
                    }
                }
            }
        });
    }
}

pub struct AsyncConnection {
    tx_handle: mpsc::Sender<Payload>,
    rx_handle: mpsc::Receiver<Payload>,
    _thread_handle: thread::JoinHandle<()>,
}

impl AsyncConnection {
    pub fn new(addr: net::SocketAddr) -> Self {
        let (send_tx, send_rx) = mpsc::channel::<Payload>(10);
        let (recv_tx, recv_rx) = mpsc::channel::<Payload>(10);

        let thread_handle = thread::spawn(move || {
            let mut network_thread = NetworkThread::new(addr);

            // Run indefinitely
            network_thread.run(send_rx, recv_tx);
        });

        Self {
            tx_handle: send_tx,
            rx_handle: recv_rx,
            _thread_handle: thread_handle,
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        // TODO: don't allocate
        // TODO: Confirm this won't fail unexpectedly, e.g. during shutdown
        self.tx_handle
            .blocking_send(Vec::from(data))
            .expect("Failed to write to queue");
    }

    pub fn try_recv(&mut self) -> Option<Payload> {
        match self.rx_handle.try_recv() {
            Ok(payload) => Some(payload),
            _ => None,
        }
    }
}

impl Updatable for AsyncConnection {
    fn update(&mut self, _dt: f32) {
        // TODO: Implement properly once we have reliable message delivery
    }
}
