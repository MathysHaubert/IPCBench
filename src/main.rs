use comfy_table::Table;
use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    time::{Duration, Instant},
};

#[derive(Debug)]
struct ResultRow {
    attempt: u32,
    elapsed_time: Duration,
    socket_type: String,
}

const PACKET_SIZE: usize = 4096;
const ATTEMPTS: u64 = 1000_000;
const PACKAGE: [u8; PACKET_SIZE] = [0u8; PACKET_SIZE];

impl ResultRow {
    fn calcul_speed(&self) -> f64 {
        let total_bytes = PACKET_SIZE as f64 * self.attempt as f64;

        let total_mo = total_bytes / (1024.0 * 1024.0);

        let secondes = self.elapsed_time.as_secs_f64();

        if secondes > 0.0 {
            total_mo / secondes
        } else {
            0.0
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut result: Vec<ResultRow> = Vec::new();

    // Unix Stream (~= local TCP -> envoie les données en stream (flux continu))
    std::thread::spawn(|| {
        run_unixstream_server();
    });
    std::thread::sleep(std::time::Duration::from_millis(10));
    result.push(unix_socket()?);

    // UnixDatagram (~= local UDP -> envoie les données en packet (batch))

    show_results(result);
    Ok(())
}

fn run_unixstream_server() {
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

fn unix_socket() -> std::io::Result<ResultRow> {
    let mut stream = UnixStream::connect("/tmp/rust_unix_socket")?;
    let start = Instant::now();
    // On met la constante dans une variable avant, car Rust optimise l'emplacement
    // de la variable si elle se trouve au dessus d'une boucle.
    // On perd en performance si la constante est passée (~31% plus lent)
    let mut package = PACKAGE;
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

fn show_results(result: Vec<ResultRow>) {
    let mut table = Table::new();

    table.set_header(vec!["Type", "Tentatives", "Temps", "Vitesse"]);
    for resultat in result {
        table.add_row(vec![
            resultat.socket_type.clone(),
            resultat.attempt.to_string(),
            format!("{:?}", resultat.elapsed_time),
            format!("{:.2} Mo/s", resultat.calcul_speed()),
        ]);
    }

    println!("{table}");
}
