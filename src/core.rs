use comfy_table::Table;

use crate::PACKET_SIZE;
use std::time::Duration;

#[derive(Debug)]
pub struct ResultRow {
    pub attempt: u32,
    pub elapsed_time: Duration,
    pub socket_type: String,
}

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

pub fn show_results(result: Vec<ResultRow>) {
    let mut table = Table::new();

    table.set_header(vec!["Type", "Attemps", "Time", "Speed", "Packet size"]);
    for resultat in result {
        table.add_row(vec![
            resultat.socket_type.clone(),
            resultat.attempt.to_string(),
            format!("{:?}", resultat.elapsed_time),
            format!("{:.2} Mo/s", resultat.calcul_speed()),
            PACKET_SIZE.to_string(),
        ]);
    }

    println!("{table}");
}
