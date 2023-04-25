use anyhow::anyhow;
use wglib::Server;

use crate::utils::{self, get_config_path};

const DEFAULT_SUBNET: &str = "10.0.0.0/24";
const DEFAULT_PORT: u16 = 51820;

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
        None => match utils::get_default_interface() {
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
        None => match utils::get_default_interface() {
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
    let config_path = get_config_path()?;
    server.dump_to_file(&config_path)?;

    Ok(())
}
