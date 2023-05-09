use axum::http::StatusCode;
use clap::Parser;

use crate::Cli;

pub async fn up() -> Result<(), (StatusCode, String)> {
    let cli = Cli::parse();
    match wglib::actions::up(&cli.device, &cli.config_path) {
        Ok(()) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn down() -> Result<(), (StatusCode, String)> {
    let cli = Cli::parse();
    match wglib::actions::down(&cli.device) {
        Ok(()) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn reboot() -> Result<(), (StatusCode, String)> {
    let cli = Cli::parse();
    match wglib::actions::reboot(&cli.device, &cli.config_path) {
        Ok(()) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
