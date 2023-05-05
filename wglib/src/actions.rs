use std::{io::Write, path::PathBuf};

use anyhow::anyhow;
use sysctl::Sysctl;

use crate::{Client, Server};

#[cfg(target_os = "linux")]
const CTLNAME: &str = "net.ipv4.ip_forward";

pub fn up(device: &str, config_path: &PathBuf) -> anyhow::Result<()> {
    let server = Server::load_from_file(config_path)?;
    dump_wg_config(&server, device)?;
    start_wg(device)?;
    Ok(())
}

pub fn down(device: &str) -> anyhow::Result<()> {
    stop_wg(device)
}

pub fn reboot(device: &str, config_path: &PathBuf) -> anyhow::Result<()> {
    let server = Server::load_from_file(config_path)?;
    dump_wg_config(&server, device)?;
    restart_wg(device)?;
    Ok(())
}

pub fn add_client(device: &str, config_path: &PathBuf, name: String) -> anyhow::Result<usize> {
    let mut server = Server::load_from_file(config_path)?;
    let id = server.add_client(name)?;
    server.dump_to_file(config_path)?;
    dump_wg_config(&server, device)?;
    restart_wg(device)?;
    Ok(id)
}

pub fn remove_client(device: &str, config_path: &PathBuf, id: usize) -> anyhow::Result<()> {
    let mut server = Server::load_from_file(config_path)?;
    server.remove_client(id)?;
    server.dump_to_file(config_path)?;
    dump_wg_config(&server, device)?;
    restart_wg(device)?;
    Ok(())
}

pub fn list_clients(config_path: &PathBuf) -> anyhow::Result<Vec<Client>> {
    let server = Server::load_from_file(config_path)?;
    Ok(server.clients.into_values().collect())
}

pub fn list_clients_filter(
    config_path: &PathBuf,
    predicate: fn(c: &Client) -> bool,
) -> anyhow::Result<Vec<Client>> {
    let server = Server::load_from_file(config_path)?;
    Ok(server.clients.into_values().filter(predicate).collect())
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
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("wg-quick failed with exit code: {}", status))
    }
}

fn allow_ip4_forwarding() -> anyhow::Result<()> {
    let ctl = sysctl::Ctl::new(CTLNAME)?;
    ctl.set_value(sysctl::CtlValue::String("1".to_string()))?;
    Ok(())
}
