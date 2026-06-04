use std::{
    fs::{create_dir, remove_dir_all},
    path::Path,
    time::Duration,
};

use crate::core::ResultRow;

mod core;
mod unix_datagram;
mod unix_stream;

pub const PACKET_SIZE: usize = 65_536;
pub const ATTEMPTS: u64 = 1000_000;
pub const PACKAGE: [u8; PACKET_SIZE] = [0u8; PACKET_SIZE];
pub const SOCKETS_PATH: &'static str = "/tmp/sockets/";

fn main() -> std::io::Result<()> {
    let mut result: Vec<core::ResultRow> = Vec::new();

    if !Path::new(SOCKETS_PATH).exists() {
        create_dir(SOCKETS_PATH)?;
    }

    // Unix Stream (~= local TCP -> envoie les données en stream (flux continu))
    match start_bench(unix_stream::run_unixstream_server, unix_stream::unix_socket) {
        Ok(result_row) => result.push(result_row),
        Err(_) => eprintln!("Error on UnixStream bench"),
    };

    // UnixDatagram (~= local UDP -> envoie les données en packet (batch))
    match start_bench(
        unix_datagram::run_datagram_server,
        unix_datagram::datagram_socket,
    ) {
        Ok(result_row) => result.push(result_row),
        Err(_) => eprintln!("Error on UnixDatagram bench"),
    };

    // Show data + clean sockets
    core::show_results(result);
    remove_dir_all(SOCKETS_PATH)?;
    Ok(())
}

fn start_bench<S, C>(s: S, c: C) -> std::io::Result<ResultRow>
where
    S: Fn() + Send + 'static,
    C: Fn() -> std::io::Result<ResultRow>,
{
    std::thread::spawn(s);
    std::thread::sleep(Duration::from_millis(10));
    c()
}
