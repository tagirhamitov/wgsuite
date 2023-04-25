use std::path::PathBuf;

use anyhow::anyhow;
use pnet::datalink::NetworkInterface;

const CONFIG_FILENAME: &str = ".wg";

pub fn get_default_interface() -> Option<NetworkInterface> {
    pnet::datalink::interfaces().into_iter().find(|e| {
        if !e.is_up() || e.is_loopback() || e.mac.is_none() {
            return false;
        }

        e.ips.iter().any(|i| i.is_ipv4())
    })
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
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
