use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    time::Instant,
};

use crate::ATTEMPTS;
use crate::PACKAGE;
use crate::PACKET_SIZE;
use crate::ResultRow;

pub fn run_unixstream_server() {
    let listener = UnixListener::bind("/tmp/rust_unix_socket").unwrap();
    let mut buffer = [0u8; 4096];
    match listener.accept() {
        Ok((mut sock, _addr)) => {
            for _ in 1..=(ATTEMPTS) {
                match sock.read(&mut buffer) {
                    Ok(bytes_lus) => {
                        if bytes_lus == 0 {
                            break;
                        } else {
                            let message = String::from_utf8_lossy(&buffer[..bytes_lus]);
                            if message.len() != PACKET_SIZE {}
                        }
                    }
                    Err(e) => {
                        println!("Erreur: {e}");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Erreur lors de la connexion: {e}");
        }
    }
}

pub fn unix_socket() -> std::io::Result<ResultRow> {
    let mut stream = UnixStream::connect("/tmp/rust_unix_socket")?;
    let start = Instant::now();
    // On met la constante dans une variable avant, car Rust optimise l'emplacement
    // de la variable si elle se trouve au dessus d'une boucle.
    // On perd en performance si la constante est passée (~31% plus lent)
    let package = PACKAGE;
    for _ in 1..=(ATTEMPTS) {
        stream.write_all(&package)?;
    }
    let duration = start.elapsed();
    Ok(ResultRow {
        attempt: ATTEMPTS as u32,
        elapsed_time: duration,
        socket_type: String::from("UnixStream"),
    })
}
