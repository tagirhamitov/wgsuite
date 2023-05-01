use std::io::Write;

use anyhow::anyhow;
use wglib::Server;

use crate::utils::{
    dump_server, get_config_path_with_sudo, get_default_interface, load_server, print_client,
    run_wg_quick,
};

const DEFAULT_DEVICE: &str = "wg0";
const DEFAULT_PORT: u16 = 51820;
const DEFAULT_SUBNET: &str = "10.0.0.0/24";

pub fn init(
    subnet: Option<String>,
    endpoint: Option<String>,
    port: Option<u16>,
    network_interface: Option<String>,
) -> anyhow::Result<()> {
    let subnet = match subnet {
        Some(subnet) => subnet.parse()?,
        None => {
            println!("Using default subnet: {}", DEFAULT_SUBNET);
            DEFAULT_SUBNET.parse()?
        }
    };

    let endpoint = match endpoint {
        Some(endpoint) => endpoint,
        None => match get_default_interface() {
            Some(network_interface) => {
                match network_interface.ips.iter().find(|&addr| addr.is_ipv4()) {
                    Some(net) => {
                        let addr = net.ip().to_string();
                        println!("Using default public ip address: {}", addr);
                        addr
                    }
                    None => return Err(anyhow!("Failed to get public ip address")),
                }
            }
            None => return Err(anyhow!("Failed to get public ip address")),
        },
    };

    let port = match port {
        Some(port) => port,
        None => {
            println!("Using default port: {}", DEFAULT_PORT);
            DEFAULT_PORT
        }
    };

    let network_interface = match network_interface {
        Some(network_interface) => network_interface,
        None => match get_default_interface() {
            Some(network_interface) => {
                println!(
                    "Using default network interface: {}",
                    network_interface.name
                );
                network_interface.name
            }
            None => return Err(anyhow!("Failed to get default network interface")),
        },
    };

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
    let device = match device {
        Some(device) => device,
        None => {
            println!("Using default device: {DEFAULT_DEVICE}");
            DEFAULT_DEVICE.to_string()
        }
    };
    let config_path = get_config_path_with_sudo()?;
    let server = Server::load_from_file(&config_path)?;
    let wg_conf = server.get_wg_config();

    let wg_conf_path: std::path::PathBuf = format!("/etc/wireguard/{}.conf", device).into();
    std::fs::create_dir_all(wg_conf_path.parent().unwrap())?;
    let mut file = std::fs::File::create(&wg_conf_path)?;
    file.write_all(wg_conf.as_bytes())?;

    run_wg_quick("up", &device)
}

pub fn stop(device: Option<String>) -> anyhow::Result<()> {
    let device = match device {
        Some(device) => device,
        None => {
            println!("Using default device: {DEFAULT_DEVICE}");
            DEFAULT_DEVICE.to_string()
        }
    };

    sudo::escalate_if_needed().map_err(|err| anyhow!("{err}"))?;

    run_wg_quick("down", &device)
}

pub fn restart(device: Option<String>) -> anyhow::Result<()> {
    let device = match device {
        Some(device) => device,
        None => {
            println!("Using default device: {DEFAULT_DEVICE}");
            DEFAULT_DEVICE.to_string()
        }
    };

    let config_path = get_config_path_with_sudo()?;
    run_wg_quick("down", &device)?;

    let server = Server::load_from_file(&config_path)?;
    let wg_conf = server.get_wg_config();

    let wg_conf_path: std::path::PathBuf = format!("/etc/wireguard/{}.conf", device).into();
    std::fs::create_dir_all(wg_conf_path.parent().unwrap())?;
    let mut file = std::fs::File::create(&wg_conf_path)?;
    file.write_all(wg_conf.as_bytes())?;

    run_wg_quick("up", &device)
}
