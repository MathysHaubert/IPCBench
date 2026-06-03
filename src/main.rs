use comfy_table::Table;
use std::time::Duration;

mod unix_stream;

#[derive(Debug)]
struct ResultRow {
    attempt: u32,
    elapsed_time: Duration,
    socket_type: String,
}

pub const PACKET_SIZE: usize = 4096;
pub const ATTEMPTS: u64 = 1000_000;
pub const PACKAGE: [u8; PACKET_SIZE] = [0u8; PACKET_SIZE];

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
        unix_stream::run_unixstream_server();
    });
    std::thread::sleep(std::time::Duration::from_millis(10));
    result.push(unix_stream::unix_socket()?);

    // UnixDatagram (~= local UDP -> envoie les données en packet (batch))

    show_results(result);
    Ok(())
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
