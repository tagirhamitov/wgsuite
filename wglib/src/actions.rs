use ipnet::Ipv4Net;

use crate::Server;

pub fn init(
    subnet: Ipv4Net,
    endpoint: String,
    port: u16,
    network_interface: Option<String>,
    config_path: &str,
) -> anyhow::Result<()> {
    let server = Server::new(subnet, endpoint, port, network_interface.unwrap());
    server.dump_config(config_path)
}

pub fn add_client(config_path: &str, name: String) -> anyhow::Result<usize> {
    let mut server = Server::from_config(config_path)?;
    let client_id = server.add_client(name)?;
    server.dump_config(config_path)?;
    Ok(client_id)
}

pub fn remove_client(config_path: &str, id: usize) -> anyhow::Result<()> {
    let mut server = Server::from_config(config_path)?;
    server.remove_client(id)
}

pub fn list_clients(config_path: &str) -> anyhow::Result<()> {
    let server = Server::from_config(config_path)?;
    for (id, client) in server.clients.iter() {
        println!("{}: {}", id, client.name);
    }
    Ok(())
}

pub fn get_server_wg_config(config_path: &str) -> anyhow::Result<String> {
    let server = Server::from_config(config_path)?;
    Ok(server.get_wg_config())
}

pub fn get_client_wg_config(config_path: &str, id: usize) -> anyhow::Result<String> {
    let server = Server::from_config(config_path)?;
    server.get_client_wg_config(id)
}
