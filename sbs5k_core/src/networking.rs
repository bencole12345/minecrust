use std::{io, net};

const BUF_SIZE: usize = 1024;

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
pub struct Connection<Cb: Fn(&[u8])> {
    sock: net::UdpSocket,
    callback: Cb,

    buf: [u8; BUF_SIZE],
    buf_len: usize,
}

// FUTURE BEN: So the idea here is that we'll use some kind of epoll or epoll-like structure, and
// we'll register this socket with it. We'll likely implementing up some kind of EpollListener
// trait for Connection, which will deal with framing and invoking the callback *if we're ready*
// (because we've seen the whole message now).

impl<Cb: Fn(&[u8])> Connection<Cb> {
    fn new(addr: net::SocketAddr, callback: Cb) -> io::Result<Self> {
        Ok(Self {
            sock: net::UdpSocket::bind(addr)?,
            callback,
            buf: [0; BUF_SIZE],
            buf_len: 0,
        })
    }

    fn send(&mut self, payload: &[u8], message_class: Option<u64>) -> io::Result<()> {
        self.sock.send(payload)?;
        Ok(())
    }
}
