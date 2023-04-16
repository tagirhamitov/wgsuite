use super::client::Client;
use crate::crypto::KeyPair;

use anyhow::anyhow;
use ipnet::Ipv4Net;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
pub struct Server {
    pub subnet: Ipv4Net,
    pub endpoint: String,
    pub port: u16,
    pub network_interface: String,
    pub keys: KeyPair,
    pub clients: HashMap<usize, Client>,
}

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    subnet_cidr: String,
    endpoint: String,
    port: u16,
    network_interface: String,
    keys: KeyPair,
    clients: HashMap<usize, Client>,
}

impl Server {
    pub fn new(subnet: Ipv4Net, endpoint: String, port: u16, network_interface: String) -> Self {
        Self {
            subnet,
            endpoint,
            port,
            network_interface,
            keys: KeyPair::generate(),
            clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, name: String) -> anyhow::Result<usize> {
        let id = match self.find_free_id() {
            Some(v) => v,
            None => return Err(anyhow!("no enough space for new clients")),
        };
        assert!(
            self.clients.insert(id, Client::new(id, name)).is_none(),
            "find_free_id() returned invalid id"
        );
        Ok(id)
    }

    pub fn remove_client(&mut self, id: usize) -> anyhow::Result<()> {
        match self.clients.remove(&id) {
            Some(_) => Ok(()),
            None => Err(anyhow!("client with id {} doesn't exist", id)),
        }
    }

    pub fn from_config(config_path: &str) -> anyhow::Result<Self> {
        let file = std::fs::File::open(config_path)?;
        let config = serde_json::from_reader(file)?;
        Self::from_server_config(config)
    }

    pub fn dump_config(&self, config_path: &str) -> anyhow::Result<()> {
        let config = ServerConfig {
            subnet_cidr: self.subnet.to_string(),
            endpoint: self.endpoint.clone(),
            port: self.port,
            network_interface: self.network_interface.clone(),
            keys: self.keys.clone(),
            clients: self.clients.clone(),
        };

        let file = std::fs::File::create(config_path)?;
        serde_json::to_writer(file, &config)?;
        Ok(())
    }

    pub fn max_number_of_clients(&self) -> usize {
        let prefix_len = self.subnet.prefix_len();
        let subnet_size = 2usize.pow((32 - prefix_len) as u32);
        subnet_size - 3
    }

    pub fn get_wg_config(&self) -> String {
        let mut config = String::new();
        config.push_str("[Interface]\n");
        config.push_str(&format!(
            "Address = {}/{}\n",
            self.subnet.subnets(32).unwrap().nth(1).unwrap().addr(),
            self.subnet.prefix_len()
        ));
        config.push_str(&format!("PostUp = iptables -A FORWARD -i %i -j ACCEPT; iptables -t nat -A POSTROUTING -o {} -j MASQUERADE\n", self.network_interface));
        config.push_str(&format!("PostDown = iptables -D FORWARD -i %i -j ACCEPT; iptables -t nat -D POSTROUTING -o {} -j MASQUERADE\n", self.network_interface));
        config.push_str(&format!("ListenPort = {}\n", self.port));
        config.push_str(&format!("PrivateKey = {}\n", self.keys.private));
        config.push('\n');
        for client in self.clients.values() {
            config.push_str("[Peer]\n");
            config.push_str(&format!("PublicKey = {}\n", client.keys.public));
            config.push_str(&format!(
                "AllowedIPs = {}/32\n",
                client.get_ip_address(&self.subnet),
            ));
            config.push('\n');
        }
        config
    }

    pub fn get_client_wg_config(&self, id: usize) -> anyhow::Result<String> {
        match self.clients.get(&id) {
            Some(client) => Ok(client.get_wg_config(self)),
            None => Err(anyhow!("client with id {} doesn't exist", id)),
        }
    }

    fn find_free_id(&self) -> Option<usize> {
        (0..self.max_number_of_clients()).find(|&id| !self.clients.contains_key(&id))
    }

    fn from_server_config(config: ServerConfig) -> anyhow::Result<Self> {
        Ok(Self {
            subnet: config.subnet_cidr.parse()?,
            endpoint: config.endpoint,
            port: config.port,
            network_interface: config.network_interface,
            keys: config.keys,
            clients: config.clients,
        })
    }
}
