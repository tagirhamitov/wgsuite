use std::time::UNIX_EPOCH;

use serde::Serialize;

#[derive(Serialize)]
pub struct ClientMetrics {
    pub public_key: String,
    pub latest_handshake: u64,
    pub received_bytes: u128,
    pub sent_bytes: u128,
}

pub fn get_metrics(device: &str) -> anyhow::Result<Vec<ClientMetrics>> {
    let dump = get_dump_from_wg(device)?;

    let mut metrics = Vec::new();
    for line in dump.lines().skip(1) {
        let (public_key, latest_handshake, received_bytes, sent_bytes) = parse_line(line)?;
        let latest_handshake = get_seconds_from_now(latest_handshake)?;

        metrics.push(ClientMetrics {
            public_key: public_key.to_string(),
            latest_handshake,
            received_bytes,
            sent_bytes,
        });
    }
    Ok(metrics)
}

fn get_dump_from_wg(device: &str) -> anyhow::Result<String> {
    let output = std::process::Command::new("wg")
        .arg("show")
        .arg(device)
        .arg("dump")
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

fn parse_line(line: &str) -> anyhow::Result<(&str, u64, u128, u128)> {
    let items: Vec<&str> = line.split_whitespace().collect();
    Ok((
        items[0],
        items[4].parse()?,
        items[5].parse()?,
        items[6].parse()?,
    ))
}

fn get_seconds_from_now(timestamp: u64) -> anyhow::Result<u64> {
    let now = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_secs() - timestamp)
}
