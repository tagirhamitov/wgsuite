use anyhow::anyhow;
use wglib::{
    actions::{dump_wg_config, restart_wg, start_wg, stop_wg},
    Server,
};

use crate::{
    defaults::{
        prepare_device, prepare_endpoint, prepare_network_interface, prepare_port, prepare_subnet,
    },
    utils::{dump_server, get_config_path_with_sudo, load_server, print_client},
};

pub fn init(
    subnet: Option<String>,
    endpoint: Option<String>,
    port: Option<u16>,
    network_interface: Option<String>,
) -> anyhow::Result<()> {
    let subnet = prepare_subnet(subnet)?;
    let endpoint = prepare_endpoint(endpoint)?;
    let port = prepare_port(port);
    let network_interface = prepare_network_interface(network_interface)?;

    let server = Server::new(subnet, endpoint, port, network_interface);
    dump_server(&server)?;

    Ok(())
}

pub fn add_client(name: String) -> anyhow::Result<()> {
    let mut server = load_server()?;
    let client_id = server.add_client(name)?;
    dump_server(&server)?;
    println!("Created client with id: {client_id}");
    Ok(())
}

pub fn remove_client(id: usize) -> anyhow::Result<()> {
    let mut server = load_server()?;
    server.remove_client(id)?;
    dump_server(&server)?;
    println!("Deleted client with id: {id}");
    Ok(())
}

pub fn server_conf() -> anyhow::Result<()> {
    let server = load_server()?;
    let conf = server.get_wg_config();
    print!("{conf}");
    Ok(())
}

pub fn client_conf(id: usize) -> anyhow::Result<()> {
    let server = load_server()?;
    let conf = server.get_client_wg_config(id)?;
    print!("{conf}");
    Ok(())
}

pub fn list_clients(name: Option<String>) -> anyhow::Result<()> {
    let server = load_server()?;
    match name {
        Some(name) => {
            for client in server.clients.values().filter(|client| client.name == name) {
                print_client(client);
            }
        }
        None => {
            for client in server.clients.values() {
                print_client(client);
            }
        }
    }
    Ok(())
}

pub fn start(device: Option<String>) -> anyhow::Result<()> {
    let device = prepare_device(device);
    let config_path = get_config_path_with_sudo()?;

    let server = Server::load_from_file(&config_path)?;
    dump_wg_config(&server, &device)?;

    start_wg(&device)
}

pub fn stop(device: Option<String>) -> anyhow::Result<()> {
    let device = prepare_device(device);
    sudo::escalate_if_needed().map_err(|err| anyhow!("{err}"))?;

    stop_wg(&device)
}

pub fn restart(device: Option<String>) -> anyhow::Result<()> {
    let device = prepare_device(device);
    let config_path = get_config_path_with_sudo()?;

    let server = Server::load_from_file(&config_path)?;
    dump_wg_config(&server, &device)?;

    restart_wg(&device)
}
