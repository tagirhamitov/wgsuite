use std::{collections::HashMap, net::Ipv4Addr};

use axum::{extract::Path, http::StatusCode, Json};
use clap::Parser;
use serde::{Deserialize, Serialize};
use wglib::Server;

use crate::Cli;

#[derive(Deserialize)]
pub struct AddClient {
    name: String,
}

pub async fn add_client(
    Json(payload): Json<AddClient>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let cli = Cli::parse();
    match wglib::actions::add_client(&cli.device, &cli.config_path, payload.name) {
        Ok(id) => Ok(Json(id)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Serialize)]
pub struct Client {
    id: usize,
    name: String,
    ip: Ipv4Addr,
    last_connected: u64,
    uploaded: u128,
    downloaded: u128,
}

pub async fn get_clients() -> Result<Json<Vec<Client>>, (StatusCode, String)> {
    let cli = Cli::parse();
    match get_clients_impl(&cli.device, &cli.config_path) {
        Ok(clients) => Ok(Json(clients)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

fn get_clients_impl(device: &str, config_path: &std::path::Path) -> anyhow::Result<Vec<Client>> {
    let server = Server::load_from_file(config_path)?;
    let clients = server.clients;
    let metrics = wglib::metrics::get_metrics(device)?;

    let mut clients: HashMap<String, (usize, String, Ipv4Addr)> = clients
        .into_values()
        .map(|client| {
            let ip = client.get_ip_address(&server.subnet);
            (client.keys.public, (client.id, client.name, ip))
        })
        .collect();
    Ok(metrics
        .into_iter()
        .filter_map(move |metric| {
            clients
                .remove(&metric.public_key)
                .map(|(id, name, ip)| Client {
                    id,
                    name,
                    ip,
                    last_connected: metric.latest_handshake,
                    uploaded: metric.received_bytes,
                    downloaded: metric.sent_bytes,
                })
        })
        .collect())
}

pub async fn get_config(Path(id): Path<usize>) -> Result<String, (StatusCode, String)> {
    let cli = Cli::parse();
    match wglib::actions::get_client_wg_config(&cli.config_path, id) {
        Ok(config) => Ok(config),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
