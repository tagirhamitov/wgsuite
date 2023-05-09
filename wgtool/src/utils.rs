use std::path::PathBuf;

use anyhow::anyhow;
use wglib::Client;

const WG_CONFIG_PATH_ENV: &str = "WG_CONFIG_PATH";

pub fn print_client(client: &Client) {
    println!("{}\t{}", client.id, client.name);
}

pub fn get_config_path_with_sudo(mut config_path: PathBuf) -> anyhow::Result<PathBuf> {
    if let Ok(path) = std::env::var(WG_CONFIG_PATH_ENV) {
        config_path = path.into();
    } else {
        std::env::set_var(WG_CONFIG_PATH_ENV, &config_path);
        sudo::with_env(&[WG_CONFIG_PATH_ENV]).map_err(|err| anyhow!("{}", err))?;
    }
    Ok(config_path)
}
