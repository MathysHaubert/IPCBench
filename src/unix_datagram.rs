use std::{os::unix::net::UnixDatagram, time::Instant};

use crate::{ATTEMPTS, PACKAGE, PACKET_SIZE, ResultRow, SOCKETS_PATH};

const SOCKET_PATH: &str = const_str::concat!(SOCKETS_PATH, "datagram");
const ACK_PATH: &str = const_str::concat!(SOCKETS_PATH, "datagram_ack");

pub fn run_datagram_server() {
    let socket = UnixDatagram::bind(SOCKET_PATH).unwrap();
    let mut buffer = [0u8; PACKET_SIZE];
    let expected = PACKET_SIZE as u64 * ATTEMPTS;
    let mut total = 0u64;

    loop {
        match socket.recv(&mut buffer) {
            Ok(n) => {
                total += n as u64;
                if total >= expected {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }

    // Répond sur le socket d'ACK du client
    let ack_socket = UnixDatagram::unbound().unwrap();
    ack_socket.send_to(&total.to_le_bytes(), ACK_PATH).unwrap();
}

pub fn datagram_socket() -> std::io::Result<ResultRow> {
    // Créer le socket d'ACK AVANT d'envoyer
    let ack = UnixDatagram::bind(ACK_PATH)?;

    let socket = UnixDatagram::unbound().unwrap();
    socket.connect(SOCKET_PATH)?;

    let start = Instant::now();
    let package = PACKAGE;
    for _ in 1..=(ATTEMPTS) {
        socket.send(&package[..])?;
    }

    // Bloquer jusqu'à l'ACK du serveur
    let mut buf = [0u8; 8];
    ack.recv(&mut buf)?;
    let duration = start.elapsed();

    Ok(ResultRow {
        attempt: ATTEMPTS as u32,
        elapsed_time: duration,
        socket_type: String::from("Socket - UnixDatagram"),
    })
}
