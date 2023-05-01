use std::path::PathBuf;

use anyhow::anyhow;
use wglib::{Client, Server};

const CONFIG_FILENAME: &str = ".wg";
const WG_CONFIG_PATH_ENV: &str = "WG_CONFIG_PATH";

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    if let Ok(path) = std::env::var(WG_CONFIG_PATH_ENV) {
        return Ok(path.into());
    }
    let home_path = match dirs::home_dir() {
        Some(path) => path,
        None => {
            return Err(anyhow!(
                "Failed to get home path, make sure the $HOME env variable is set"
            ))
        }
    };
    Ok(home_path.join(CONFIG_FILENAME))
}

pub fn load_server() -> anyhow::Result<Server> {
    let config_path = get_config_path()?;
    Server::load_from_file(&config_path)
}

pub fn dump_server(server: &Server) -> anyhow::Result<()> {
    let config_path = get_config_path()?;
    server.dump_to_file(&config_path)
}

pub fn print_client(client: &Client) {
    println!("{}\t{}", client.id, client.name);
}

pub fn get_config_path_with_sudo() -> anyhow::Result<PathBuf> {
    let config_path = get_config_path()?;
    std::env::set_var(WG_CONFIG_PATH_ENV, &config_path);
    sudo::with_env(&[WG_CONFIG_PATH_ENV]).map_err(|err| anyhow!("{}", err))?;
    Ok(config_path)
}

pub fn run_wg_quick(cmd: &str, device: &str) -> anyhow::Result<()> {
    let status = std::process::Command::new("wg-quick")
        .arg(cmd)
        .arg(device)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("wg-quick failed with exit code: {}", status))
    }
}
