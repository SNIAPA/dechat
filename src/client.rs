use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{HS_DIR, PORT, TOR_SOCKS_PORT};

use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Client {
    url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String) -> Result<Client, Error> {
        Ok(Client {
            url,
            client: reqwest::Client::builder()
                .proxy(reqwest::Proxy::http(format!(
                    "http://127.0.0.1:{}",
                    TOR_SOCKS_PORT
                ))?)
                .build()?,
        })
    }
    pub fn send(&mut self, message: &str) -> Result<(), Error> {
        let url = format!("http://{}",self.url);
        dbg!(message);
        self.client.get(url).build().unwrap(); Ok(())
    }
}
