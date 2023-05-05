use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

use axum::{http::StatusCode, routing::get, Router};
use clap::Parser;
use wglib::Server;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    device: String,
    #[arg(long)]
    config_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    sudo::escalate_if_needed()?;
    let cli = Cli::parse();

    let app = Router::new().route("/metrics", get(|| get_metrics(cli.device, cli.config_path)));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn get_metrics(device: String, config_path: PathBuf) -> (StatusCode, String) {
    match get_metrics_impl(&device, &config_path) {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

fn get_metrics_impl(device: &str, config_path: &Path) -> anyhow::Result<String> {
    let server = Server::load_from_file(config_path)?;
    let metrics = wglib::metrics::get_metrics(device, &server)?;
    let serialized_metrics = serde_json::to_string(&metrics)?;
    Ok(serialized_metrics)
}
