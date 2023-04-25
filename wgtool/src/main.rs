mod actions;
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init {
            subnet,
            endpoint,
            port,
            interface,
        } => actions::init(subnet, endpoint, port, interface)?,
    };
    Ok(())
}
