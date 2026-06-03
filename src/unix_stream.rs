use std::{
    io::{BufWriter, Read, Write},
    net::Shutdown,
    os::unix::net::{UnixListener, UnixStream},
    time::Instant,
};

use crate::ATTEMPTS;
use crate::PACKAGE;
use crate::PACKET_SIZE;
use crate::ResultRow;
use crate::SOCKETS_PATH;

const SOCKET_PATH: &str = const_str::concat!(SOCKETS_PATH, "unixstream");

pub fn run_unixstream_server() {
    let listener = UnixListener::bind(SOCKET_PATH).unwrap();
    let mut buffer = [0u8; PACKET_SIZE / 8];
    let mut total = 0usize;

    match listener.accept() {
        Ok((mut sock, _addr)) => {
            loop {
                match sock.read(&mut buffer) {
                    Ok(0) => {
                        // Le client a fini sa boucle et son flush()
                        break;
                    }
                    Ok(_bytes_lus) => {
                        total += _bytes_lus;
                    }
                    Err(e) => {
                        println!("Erreur: {e}");
                        break;
                    }
                }
            }

            let ack = (total as u64).to_le_bytes();
            sock.write_all(&ack).unwrap();
        }
        Err(e) => {
            println!("Erreur lors de la connexion: {e}");
        }
    }
}

pub fn unix_socket() -> std::io::Result<ResultRow> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    let start = Instant::now();

    {
        let mut buffer = BufWriter::with_capacity(PACKET_SIZE * 8, &stream);
        // On met la constante dans une variable avant, car Rust optimise l'emplacement
        // de la variable si elle se trouve au dessus d'une boucle.
        // On perd en performance si la constante est passée (~31% plus lent)
        let package = PACKAGE;
        for _ in 1..=(ATTEMPTS) {
            buffer.write_all(&package)?;
        }
        buffer.flush()?;
    }

    stream.shutdown(Shutdown::Write)?;

    let mut ack = [0u8; 8];
    stream.read_exact(&mut ack)?;

    let duration = start.elapsed();

    Ok(ResultRow {
        attempt: ATTEMPTS as u32,
        elapsed_time: duration,
        socket_type: String::from("UnixStream"),
    })
}
