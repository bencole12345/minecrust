mod request_processor;

use core::backend::Client2Server;
use std::{io, net};

use sbs5k_core as core;

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:12345";

    println!("Listening on {}", addr);
    let sock = net::UdpSocket::bind(addr)?;

    let mut buf = [0; 1500];
    while let Ok((_, src)) = sock.recv_from(&mut buf) {
        // TODO: Reuse the buffer
        let msg: Client2Server = bincode::deserialize(&buf).expect("Failed to deserialize");

        handle_msg(&msg, &sock, src);
    }

    Ok(())
}

