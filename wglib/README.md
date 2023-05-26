# WGLib

This library contains classes and methods for managing Wireguard vpn.

These classes represent server and client:
```rust
pub struct Server {
    pub subnet: Ipv4Net,
    pub endpoint: String,
    pub port: u16,
    pub network_interface: String,
    pub keys: KeyPair,
    pub clients: HashMap<usize, Client>,
}

pub struct Client {
    pub id: usize,
    pub name: String,
    pub keys: KeyPair,
}
```

You can use two levels of API:
*   low level API allows to have more control over loading and dumping configuration on disk. Example usage:
    ```rust
    let mut server = Server::load_from_file("path/to/config")?;
    server.add_client("ClientName")?;
    server.dump_to_file("path/to/config")?;
    ```
    You can see the full list of methods in [server.rs](src/model/server.rs)
*   high level API allows to run commands without manually loading server from config. Example usage:
    ```rust
    wglib::actions::add_client("wg0", "path/to/config", "ClientName")?;
    ```
    You can see the full list of available functions in [actions.rs](src/actions.rs) and [metrics.rs](src/metrics.rs)
