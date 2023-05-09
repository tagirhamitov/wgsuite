mod commands;
mod defaults;
mod utils;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(long)]
        subnet: Option<String>,
        #[arg(long)]
        endpoint: Option<String>,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        interface: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    AddClient {
        name: String,
        #[arg(long)]
        device: Option<String>,
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
    RemoveClient {
        id: usize,
        #[arg(long)]
        device: Option<String>,
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
    ServerConf {
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
    ClientConf {
        id: usize,
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
    ListClients {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
    Start {
        #[arg(long)]
        device: Option<String>,
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
    Stop {
        #[arg(long)]
        device: Option<String>,
    },
    Restart {
        #[arg(long)]
        device: Option<String>,
        #[arg(long)]
        config_path: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init {
            subnet,
            endpoint,
            port,
            interface,
            output,
        } => commands::init(subnet, endpoint, port, interface, output)?,
        Commands::AddClient {
            name,
            device,
            config_path,
        } => commands::add_client(name, device, config_path)?,
        Commands::RemoveClient {
            id,
            device,
            config_path,
        } => commands::remove_client(id, device, config_path)?,
        Commands::ServerConf { config_path } => commands::server_conf(config_path)?,
        Commands::ClientConf { id, config_path } => commands::client_conf(id, config_path)?,
        Commands::ListClients { name, config_path } => commands::list_clients(name, config_path)?,
        Commands::Start {
            device,
            config_path,
        } => commands::start(device, config_path)?,
        Commands::Stop { device } => commands::stop(device)?,
        Commands::Restart {
            device,
            config_path,
        } => commands::restart(device, config_path)?,
    };
    Ok(())
}
