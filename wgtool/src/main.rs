use wglib::actions::{add_client, get_client_wg_config, get_server_wg_config, init};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cp = "server.json";
    init(
        "10.200.200.0/24".parse().unwrap(),
        "vpn.net".to_string(),
        51820,
        Some("eth0".to_string()),
        cp,
    )?;
    add_client(cp, "client1".to_string())?;
    add_client(cp, "client2".to_string())?;
    print!("{}", get_client_wg_config(cp, 0)?);
    Ok(())
}
