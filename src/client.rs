use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{HS_DIR, PORT, TOR_SOCKS_PORT};

use anyhow::Result;

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }
    pub fn run(&self, hostname: String) -> Result<()> {
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::http(format!(
                "socks5://127.0.0.1:{}",
                TOR_SOCKS_PORT
            ))?)
            .build()?;
        let hostname = format!("{hostname}:{PORT}");
        client.get(hostname);
        Ok(())
    }
}
