use std::path::PathBuf;

use anyhow::anyhow;
use wglib::{
    actions::{dump_wg_config, restart_wg, start_wg, stop_wg},
    Server,
};

use crate::{defaults, utils};

pub fn init(
    subnet: Option<String>,
    endpoint: Option<String>,
    port: Option<u16>,
    network_interface: Option<String>,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let subnet = defaults::prepare_subnet(subnet)?;
    let endpoint = defaults::prepare_endpoint(endpoint)?;
    let port = defaults::prepare_port(port);
    let network_interface = defaults::prepare_network_interface(network_interface)?;
    let output = defaults::prepare_config_path(output)?;

    let server = Server::new(subnet, endpoint, port, network_interface);
    server.dump_to_file(&output)?;

    println!("Created config at: {:?}", output);
    Ok(())
}

pub fn add_client(
    name: String,
    device: Option<String>,
    config_path: Option<PathBuf>,
) -> anyhow::Result<()> {
    let device = defaults::prepare_device(device);
    let config_path = defaults::prepare_config_path(config_path)?;

    let config_path = utils::get_config_path_with_sudo(config_path)?;

    let id = wglib::actions::add_client(&device, &config_path, name)?;

    println!("Created client with id: {id}");
    Ok(())
}

pub fn remove_client(
    id: usize,
    device: Option<String>,
    config_path: Option<PathBuf>,
) -> anyhow::Result<()> {
    let device = defaults::prepare_device(device);
    let config_path = defaults::prepare_config_path(config_path)?;

    let config_path = utils::get_config_path_with_sudo(config_path)?;

    wglib::actions::remove_client(&device, &config_path, id)?;

    println!("Removed client with id: {id}");
    Ok(())
}

pub fn server_conf(config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let config_path = defaults::prepare_config_path(config_path)?;
    let server = Server::load_from_file(&config_path)?;
    let conf = server.get_wg_config();
    print!("{conf}");
    Ok(())
}

pub fn client_conf(id: usize, config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let config_path = defaults::prepare_config_path(config_path)?;
    let server = Server::load_from_file(&config_path)?;
    let conf = server.get_client_wg_config(id)?;
    print!("{conf}");
    Ok(())
}

pub fn list_clients(name: Option<String>, config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let config_path = defaults::prepare_config_path(config_path)?;
    let server = Server::load_from_file(&config_path)?;
    match name {
        Some(name) => {
            for client in server.clients.values().filter(|client| client.name == name) {
                utils::print_client(client);
            }
        }
        None => {
            for client in server.clients.values() {
                utils::print_client(client);
            }
        }
    }
    Ok(())
}

pub fn start(device: Option<String>, config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let device = defaults::prepare_device(device);
    let config_path = defaults::prepare_config_path(config_path)?;

    let config_path = utils::get_config_path_with_sudo(config_path)?;

    let server = Server::load_from_file(&config_path)?;
    dump_wg_config(&server, &device)?;
    start_wg(&device)?;

    println!("Device {} started", device);
    Ok(())
}

pub fn stop(device: Option<String>) -> anyhow::Result<()> {
    let device = defaults::prepare_device(device);
    sudo::escalate_if_needed().map_err(|err| anyhow!("{err}"))?;
    stop_wg(&device)?;

    println!("Device {} stopped", device);
    Ok(())
}

pub fn restart(device: Option<String>, config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let device = defaults::prepare_device(device);
    let config_path = defaults::prepare_config_path(config_path)?;

    let config_path = utils::get_config_path_with_sudo(config_path)?;

    let server = Server::load_from_file(&config_path)?;
    dump_wg_config(&server, &device)?;
    restart_wg(&device)?;

    println!("Device {} restarted", device);
    Ok(())
}
