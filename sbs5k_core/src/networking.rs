use serde::Serialize;
use std::os::fd::RawFd;
use std::sync::mpsc;
use std::{io, net, thread, time};

#[cfg(windows)]
use std::os::io::AsRawSocket;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

const BUF_SIZE: usize = 1024;

pub trait OnPacketCallback {
    fn on_packet(&mut self, data: &[u8]);
}

/// A UDP-based transport protocol that supports both fast individual packet delivery and reliable
/// in-order delivery of multi-frame messages.
///
/// There are two modes of operation:
///
/// *Reliable*: Messages are sent reliably using acks. If we don't get an ack, we'll keep retrying
/// until we do.
///
/// *Replaceable*: Messages are still sent reliably, but may be replaced by a later "version" of
/// that message. This is intended for "update"-type messages: for example, there's no point trying
/// to get an ack for an earlier "player moved" message if it's already been superseded by a later
/// location update for the same player.
pub struct Connection {
    sock: net::UdpSocket,
    buf: [u8; BUF_SIZE],
    // TODO: Make sending reliable
    // TODO: Support replaceable messages (make sure later versions can't replace earlier versions)
    // TODO: Handle disconnection and reconnection
}

// FUTURE BEN: So the idea here is that we'll use some kind of epoll or epoll-like structure, and
// we'll register this socket with it. We'll likely implementing up some kind of EpollListener
// trait for Connection, which will deal with framing and invoking the callback *if we're ready*n
// (because we've seen the whole message now).

impl Connection {
    fn new(addr: net::SocketAddr) -> io::Result<Self> {
        Ok(Self {
            sock: net::UdpSocket::bind(addr)?,
            buf: [0; BUF_SIZE],
        })
    }

    fn send(&mut self, payload: &impl Serialize) -> io::Result<()> {
        let serialized = bincode::serialize(payload).expect("Failed to serialize");
        self.sock.send(serialized.as_slice())?;
        Ok(())
    }

    fn on_tick(&mut self, cb: &mut impl OnPacketCallback) {
        while let Ok(size) = self.sock.recv(&mut self.buf) {
            cb.on_packet(&self.buf[..size]);
        }
    }
}

struct NetworkThread {
    event_base: libevent::Base,
    socket: net::UdpSocket,
    dest: net::SocketAddr,
    send_queue_rx: mpsc::Receiver<Vec<u8>>,
    recv_queue_tx: mpsc::Sender<Vec<u8>>,
}

impl NetworkThread {
    fn new(
        dest: net::SocketAddr,
        send_queue_rx: mpsc::Receiver<Vec<u8>>,
        recv_queue_tx: mpsc::Sender<Vec<u8>>,
    ) -> Self {
        let mut event_base = libevent::Base::new()
            .expect("Failed to create libevent base - are you running on a funky OS?");

        // Set up the socket
        let socket = net::UdpSocket::bind("0.0.0.0:0").expect("Failed to create UDP socket");
        // TODO: Why do we need nonblocking? can't remember why I wrote this
        // socket
        //     .set_nonblocking(true)
        //     .expect("Failed to set nonblocking");

        Self {
            event_base,
            socket,
            dest,
            send_queue_rx,
            recv_queue_tx,
        }
    }

    fn run(&mut self) {
        // Register the socket with the event base
        let socket_fd = get_socket_fd(&self.socket);
        let socket_event = libevent::Event::new(socket_fd, libevent::EventFlags::READ, None);
        self.event_base
            .spawn(socket_event, |_, _, _| {
                // self.on_socket_event();
            })
            .expect("Failed to add socket");

        // Set up a timer with the event base
        let timer = libevent::Interval::new(time::Duration::from_millis(10)); // Approximately 60 Hz
        self.event_base
            .spawn(timer, |_| {
                while let Ok(msg) = self.send_queue_rx.try_recv() {
                    // TODO: Handle passing through the address better
                    // TODO: Handle failures better
                    println!("{}", self.dest);
                    // self.socket.send_to(msg.as_slice(), self.dest).expect("Failed to send packet");
                    self.socket
                        .send_to(msg.as_slice(), "127.0.0.1:12345")
                        .expect("Failed to send packet");
                }
            })
            .expect("Failed to add timer");

        // Kick it off
        println!("Starting networking thread");
        self.event_base.run();
    }

    fn on_socket_event(&self) {

    }

    fn on_timer(&self) {

    }
}

fn get_socket_fd(socket: &net::UdpSocket) -> RawFd {
    #[cfg(unix)]
    let fd = socket.as_raw_fd();
    #[cfg(windows)]
    let fd = socket.as_raw_socket();
    fd
}

pub struct AsyncConnection {
    tx_handle: mpsc::Sender<Vec<u8>>,
    rx_handle: mpsc::Receiver<Vec<u8>>,
    thread_handle: thread::JoinHandle<()>,
}

impl AsyncConnection {
    pub fn new(addr: net::SocketAddr) -> Self {
        let (send_tx_handle, send_rx_handle) = mpsc::channel();
        let (recv_tx_handle, recv_rx_handle) = mpsc::channel();
        let thread_handle = thread::spawn(move || {
            let mut network_thread = NetworkThread::new(addr, send_rx_handle, recv_tx_handle);
            network_thread.run();
        });
        Self {
            tx_handle: send_tx_handle,
            rx_handle: recv_rx_handle,
            thread_handle,
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        // TODO: don't allocate
        // TODO: Confirm this won't fail unexpectedly, e.g. during shutdown
        self.tx_handle
            .send(Vec::from(data))
            .expect("Failed to write to queue");
    }
}
