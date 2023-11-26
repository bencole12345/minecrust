use core::backend::Client2Server;
use serde::Deserialize;
use std::collections::HashMap;
use std::{io, net};

use bincode;

use sbs5k_core as core;

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:12345";

    println!("Listening on {}", addr);
    let sock = net::UdpSocket::bind(addr)?;

    let mut outbound_socks = HashMap::new();

    let mut buf = [0; 1500];
    while let Ok((amt, src)) = sock.recv_from(&mut buf) {
        let outbound_sock = outbound_socks
            .entry(amt)
            .or_insert_with(|| net::UdpSocket::bind(src));

        // let mut deserializer = postcard::Deserializer::from_bytes(&buf);
        // let msg = Message::deserialize(&mut deserializer).unwrap();

        // TODO: Reuse the buffer
        let msg: Client2Server = bincode::deserialize(&buf).expect("Failed to deserialize");

        // outbound_socks.
        println!("Received: {:?}", msg);
    }

    Ok(())
}
