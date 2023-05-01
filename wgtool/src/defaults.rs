use anyhow::anyhow;
use ipnet::Ipv4Net;
use pnet::datalink::NetworkInterface;

const DEFAULT_DEVICE: &str = "wg0";
const DEFAULT_PORT: u16 = 51820;
const DEFAULT_SUBNET: &str = "10.0.0.0/24";

pub fn prepare_subnet(subnet: Option<String>) -> anyhow::Result<Ipv4Net> {
    Ok(match subnet {
        Some(subnet) => subnet.parse()?,
        None => {
            println!("Using default subnet: {}", DEFAULT_SUBNET);
            DEFAULT_SUBNET.parse()?
        }
    })
}

pub fn prepare_endpoint(endpoint: Option<String>) -> anyhow::Result<String> {
    Ok(match endpoint {
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
    })
}

pub fn prepare_port(port: Option<u16>) -> u16 {
    match port {
        Some(port) => port,
        None => {
            println!("Using default port: {}", DEFAULT_PORT);
            DEFAULT_PORT
        }
    }
}

pub fn prepare_network_interface(network_interface: Option<String>) -> anyhow::Result<String> {
    Ok(match network_interface {
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
    })
}

pub fn prepare_device(device: Option<String>) -> String {
    match device {
        Some(device) => device,
        None => {
            println!("Using default device: {DEFAULT_DEVICE}");
            DEFAULT_DEVICE.to_string()
        }
    }
}

fn get_default_interface() -> Option<NetworkInterface> {
    pnet::datalink::interfaces().into_iter().find(|e| {
        if !e.is_up() || e.is_loopback() || e.mac.is_none() {
            return false;
        }

        e.ips.iter().any(|i| i.is_ipv4())
    })
}
