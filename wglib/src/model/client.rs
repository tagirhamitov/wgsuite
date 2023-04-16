use std::net::Ipv4Addr;

use ipnet::Ipv4Net;
use serde::{Deserialize, Serialize};

use crate::{crypto::KeyPair, Server};

#[derive(Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: usize,
    pub name: String,
    pub keys: KeyPair,
}

impl Client {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            keys: KeyPair::generate(),
        }
    }

    pub fn get_wg_config(&self, server: &Server) -> String {
        let mut config = String::new();
        config.push_str("[Interface]\n");
        config.push_str(&format!(
            "Address = {}\n",
            self.get_ip_address(&server.subnet)
        ));
        config.push_str(&format!("PrivateKey = {}\n", self.keys.private));
        config.push_str("DNS = 8.8.8.8\n");
        config.push_str("\n[Peer]\n");
        config.push_str(&format!("PublicKey = {}\n", server.keys.public));
        config.push_str(&format!("Endpoint = {}:{}\n", server.endpoint, server.port));
        config.push_str("AllowedIPs = 0.0.0.0/0\n");
        config.push_str("PersistentKeepalive = 25\n");
        config
    }

    pub fn get_ip_address(&self, subnet: &Ipv4Net) -> Ipv4Addr {
        subnet.subnets(32).unwrap().nth(self.id + 2).unwrap().addr()
    }
}
