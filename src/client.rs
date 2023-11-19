use std::{
    fs,
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use crate::{HS_DIR, PORT, TOR_SOCKS_PORT};

use anyhow::Result;

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }
    pub fn run(&self) -> Result<Client> {
        let file_name = format!("{}/hostname", HS_DIR);
        let mut hostname = fs::read_to_string(file_name).unwrap();
        hostname = hostname.strip_suffix("\n").unwrap().to_owned();
        loop {
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TOR_SOCKS_PORT);
            let address = format!("{}:{}", hostname, PORT);

            let mut stream = tor_stream::TorStream::connect_with_address(socket, address.as_ref())?;
            stream.write(b"test")?;
            stream.flush()?;
        }
    }
}
