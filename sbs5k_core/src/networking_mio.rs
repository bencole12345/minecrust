use std::{io, net};
use std::sync::mpsc;
use serde::Serialize;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::io::AsRawSocket;
use std::ptr::NonNull;
use std::time::Duration;
use mio::Interest;

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
    // TODO: Support replaceable messages (but make sure older versions never overtake newer versions)
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
    poll: mio::Poll,
    events: mio::Events,
    socket: mio::net::UdpSocket,
    // TODO: Use a different type to cancel the timer?
    timer: mio_extras::timer::Timer<()>,
    // TODO: Look into the mio_extras channel options (think they're incompatible with the latest mio but worth double-checking)
    send_queue_rx: mpsc::Receiver<Vec<u8>>,
    recv_queue_tx: mpsc::Sender<Vec<u8>>,
}

const SEND_QUEUE: mio::Token = mio::Token(0);
const NETWORK: mio::Token = mio::Token(1);
const TIMER: mio::Token = mio::Token(2);

impl NetworkThread {
    fn new(dest: net::SocketAddr, send_queue_rx: mpsc::Receiver<Vec<u8>>, recv_queue_tx: mpsc::Sender<Vec<u8>>) -> Self {
        // let event_base = libevent::Base::new().expect("Failed to create libevent base - are you running on a funky OS?");
        //
        // // Set up the socket
        // let socket = net::UdpSocket::bind(dest).expect("Failed to create UDP socket");
        // socket.set_nonblocking(true).expect("Failed to set nonblocking");
        // #[cfg(unix)]
        //     let socket_fd = socket.as_raw_fd();
        // #[cfg(windows)]
        //     let socket_fd = socket.as_raw_socket();
        // let socket_event = libevent::Event::new(socket_fd, libevent::EventFlags::READ, None);
        // event_base.event_add(NonNull::from(&socket_event), None);
        //
        // // Set up the timer
        // let timer = libevent::Interval::new(Duration::from_millis(10));  // Approximately 60 Hz
        // event_base.event_add(NonNull::from(timer), None);

        let mut scheduler = mio_misc::scheduler::Scheduler::new(Some(String::from("network-thread-scheduler")));
        // scheduler.schedule(mio_misc::scheduler::NotificationScheduler::notify_with_fixed_interval())

        mio_misc::scheduler::NotificationScheduler::new()

        Self {
            poll: mio::Poll::new().expect("Failed to create Poll - are you running on a funky OS?"),
            events: mio::Events::with_capacity(64),
            socket: mio::net::UdpSocket::bind(dest).expect("Failed to create socket"),
            timer: ,
            send_queue_rx,
            recv_queue_tx,
        }
    }

    fn run(&mut self) {
        self.poll.registry().register(&mut self.socket, NETWORK, Interest::READABLE).unwrap();
        // self.poll.registry().register(&mut self.send_queue_rx, SEND_QUEUE, Interest::READABLE).unwrap();
        self.poll.registry().register(&mut self.timer, TIMER, Interest::READABLE).unwrap();

        loop {
            self.poll.poll(&mut self.events, None).expect("failed to poll");
            for event in &self.events {
                match event.token() {
                    // SEND_QUEUE => {
                    //     // TODO: Read from the queue and send it to the socket
                    // }
                    NETWORK => {
                        // TODO: Dispatch to the socket handler
                    }
                    TIMER => {
                        // TODO: Check the send queue
                        // TODO: Handle resends
                    }
                    _ => unreachable!()
                }
            }
        }
    }
}

pub struct AsyncConnection {
    tx_handle: mpsc::Sender<Vec<u8>>,
    rx_handle: mpsc::Receiver<Vec<u8>>,
}

impl AsyncConnection {
    pub fn new(addr: net::SocketAddr) -> io::Result<Self> {
        // let (send_tx_handle, send_rx_handle) = mpsc::channel();
        // let (recv_tx_handle, recv_rx_handle) = mpsc::channel();
        todo!()
    }

    pub fn send(data: &[u8]) {
        todo!()
    }
}
