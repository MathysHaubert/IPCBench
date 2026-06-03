use std::{os::unix::net::UnixDatagram, time::Instant};

use crate::{ATTEMPTS, PACKAGE, PACKET_SIZE, ResultRow, SOCKETS_PATH};

const SOCKET_PATH: &str = const_str::concat!(SOCKETS_PATH, "datagram");

pub fn run_datagram_server() {
    let socket = UnixDatagram::bind(SOCKET_PATH).unwrap();
    let mut buffer = [0u8; PACKET_SIZE]; // Parce que le socket est "UDP", le packet doit etre de la meme taille.
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((count, _)) => {
                if count != PACKET_SIZE {
                    eprintln!("Packet lost !");
                    continue;
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

pub fn datagram_socket() -> std::io::Result<ResultRow> {
    let socket = UnixDatagram::unbound().unwrap();
    socket.connect(SOCKET_PATH)?;

    let start = Instant::now();
    // On met la constante dans une variable avant, car Rust optimise l'emplacement
    // de la variable si elle se trouve au dessus d'une boucle.
    // On perd en performance si la constante est passée (~31% plus lent)
    let package = PACKAGE;
    for _ in 1..=(ATTEMPTS) {
        socket.send(&package[..])?;
    }
    let duration = start.elapsed();
    Ok(ResultRow {
        attempt: ATTEMPTS as u32,
        elapsed_time: duration,
        socket_type: String::from("UnixDatagram"),
    })
}
