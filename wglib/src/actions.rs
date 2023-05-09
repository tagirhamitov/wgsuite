use std::{io::Write, path::Path};

use anyhow::anyhow;
use sysctl::Sysctl;

use crate::{Client, Server};

#[cfg(target_os = "linux")]
const CTLNAME: &str = "net.ipv4.ip_forward";

pub fn up(device: &str, config_path: &Path) -> anyhow::Result<()> {
    let server = Server::load_from_file(config_path)?;
    dump_wg_config(&server, device)?;
    start_wg(device)?;
    Ok(())
}

pub fn down(device: &str) -> anyhow::Result<()> {
    stop_wg(device)
}

pub fn reboot(device: &str, config_path: &Path) -> anyhow::Result<()> {
    let server = Server::load_from_file(config_path)?;
    dump_wg_config(&server, device)?;
    restart_wg(device)?;
    Ok(())
}

pub fn add_client(device: &str, config_path: &Path, name: String) -> anyhow::Result<usize> {
    let mut server = Server::load_from_file(config_path)?;
    let id = server.add_client(name)?;
    let client = server.get_client(id)?;
    if is_wg_started(device)? {
        wg_update_with_client(device, WgUpdatedClient::Added(client), &server)?;
    }
    server.dump_to_file(config_path)?;
    Ok(id)
}

pub fn remove_client(device: &str, config_path: &Path, id: usize) -> anyhow::Result<()> {
    let mut server = Server::load_from_file(config_path)?;
    let client = server.remove_client(id)?;
    if is_wg_started(device)? {
        wg_update_with_client(device, WgUpdatedClient::Removed(client), &server)?;
    }
    server.dump_to_file(config_path)?;
    Ok(())
}

pub fn list_clients(config_path: &Path) -> anyhow::Result<Vec<Client>> {
    let server = Server::load_from_file(config_path)?;
    Ok(server.clients.into_values().collect())
}

pub fn list_clients_filter(
    config_path: &Path,
    predicate: fn(c: &Client) -> bool,
) -> anyhow::Result<Vec<Client>> {
    let server = Server::load_from_file(config_path)?;
    Ok(server.clients.into_values().filter(predicate).collect())
}

pub fn get_client(config_path: &Path, id: usize) -> anyhow::Result<Client> {
    let server = Server::load_from_file(config_path)?;
    let client = server.get_client(id)?;
    Ok(client)
}

pub fn get_client_wg_config(config_path: &Path, id: usize) -> anyhow::Result<String> {
    let server = Server::load_from_file(config_path)?;
    let config = server.get_client_wg_config(id)?;
    Ok(config)
}

pub fn start_wg(device: &str) -> anyhow::Result<()> {
    allow_ip4_forwarding()?;
    wg_manage(device, WgManageCommand::Up)
}

pub fn stop_wg(device: &str) -> anyhow::Result<()> {
    wg_manage(device, WgManageCommand::Down)
}

pub fn restart_wg(device: &str) -> anyhow::Result<()> {
    stop_wg(device)?;
    start_wg(device)?;
    Ok(())
}

pub fn dump_wg_config(server: &Server, device: &str) -> anyhow::Result<()> {
    let wg_conf = server.get_wg_config();

    let wg_conf_path: std::path::PathBuf = format!("/etc/wireguard/{}.conf", device).into();
    std::fs::create_dir_all(wg_conf_path.parent().unwrap())?;

    let mut file = std::fs::File::create(wg_conf_path)?;
    file.write_all(wg_conf.as_bytes())?;

    Ok(())
}

enum WgManageCommand {
    Up,
    Down,
}

fn wg_manage(device: &str, cmd: WgManageCommand) -> anyhow::Result<()> {
    let status = std::process::Command::new("wg-quick")
        .arg(match cmd {
            WgManageCommand::Up => "up",
            WgManageCommand::Down => "down",
        })
        .arg(device)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("wg-quick failed with exit code: {}", status))
    }
}

enum WgUpdatedClient {
    Added(Client),
    Removed(Client),
}

fn wg_update_with_client(
    device: &str,
    cmd: WgUpdatedClient,
    server: &Server,
) -> anyhow::Result<()> {
    let status = match cmd {
        WgUpdatedClient::Added(client) => std::process::Command::new("wg")
            .arg("set")
            .arg(device)
            .arg("peer")
            .arg(&client.keys.public)
            .arg("allowed-ips")
            .arg(format!("{}/32", client.get_ip_address(&server.subnet)))
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?,
        WgUpdatedClient::Removed(client) => std::process::Command::new("wg")
            .arg("set")
            .arg(device)
            .arg("peer")
            .arg(&client.keys.public)
            .arg("remove")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?,
    };

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("wg failed with exit code: {}", status))
    }
}

fn is_wg_started(device: &str) -> anyhow::Result<bool> {
    let status = std::process::Command::new("wg")
        .arg("show")
        .arg(device)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;
    Ok(status.success())
}

fn allow_ip4_forwarding() -> anyhow::Result<()> {
    let ctl = sysctl::Ctl::new(CTLNAME)?;
    ctl.set_value(sysctl::CtlValue::String("1".to_string()))?;
    Ok(())
}
