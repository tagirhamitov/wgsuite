mod commands;
mod defaults;
mod utils;

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
    },
    AddClient {
        #[arg(long)]
        name: String,
    },
    RemoveClient {
        #[arg(long)]
        id: usize,
    },
    ServerConf,
    ClientConf {
        #[arg(long)]
        id: usize,
    },
    ListClients {
        #[arg(long)]
        name: Option<String>,
    },
    Start {
        #[arg(long)]
        device: Option<String>,
    },
    Stop {
        #[arg(long)]
        device: Option<String>,
    },
    Restart {
        #[arg(long)]
        device: Option<String>,
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
        } => commands::init(subnet, endpoint, port, interface)?,
        Commands::AddClient { name } => commands::add_client(name)?,
        Commands::RemoveClient { id } => commands::remove_client(id)?,
        Commands::ServerConf => commands::server_conf()?,
        Commands::ClientConf { id } => commands::client_conf(id)?,
        Commands::ListClients { name } => commands::list_clients(name)?,
        Commands::Start { device } => commands::start(device)?,
        Commands::Stop { device } => commands::stop(device)?,
        Commands::Restart { device } => commands::restart(device)?,
    };
    Ok(())
}
