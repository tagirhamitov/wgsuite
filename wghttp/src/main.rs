mod boot;
mod clients;

use std::{net::SocketAddr, path::PathBuf};

use axum::{
    http::Method,
    routing::{delete, get, post},
    Router,
};
use clap::Parser;

use boot::*;
use clients::*;

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
    Cli::parse();

    let app = Router::new()
        .route("/up", post(up))
        .route("/down", post(down))
        .route("/reboot", post(reboot))
        .route("/clients", post(add_client))
        .route("/clients/:id", delete(remove_client))
        .route("/clients", get(get_clients))
        .route("/config/:id", get(get_config))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers(tower_http::cors::Any)
                .allow_origin(tower_http::cors::Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
