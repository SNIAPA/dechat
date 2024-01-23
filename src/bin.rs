#![feature(async_closure)]
extern crate rocket;

use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};
use tokio::sync::Mutex;

use anyhow::Result;
use dechat_lib::{
    client::Client,
    server::{rocket, Server},
    tor::start_tor,
    tui::tui,
};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tor_handle: Option<JoinHandle<_>> = None;
    let mut backend_handle = None;

    let res = {
        let (hostname, handle) = start_tor().await?;
        tor_handle = Some(handle);

        let client = Arc::new(Mutex::new(Client::new(hostname)?));
        let server = Arc::new(Mutex::new(Server { messages: vec![] }));

        backend_handle = Some(tokio::spawn(rocket(server.clone())));

        tui(client, server).await.unwrap();
        Ok(())
    };

    unsafe {
        if let Some(tor_handle) = tor_handle {
            stop_thread::kill_thread_graceful(tor_handle);
        }
        if let Some(backend_handle) = backend_handle {
            backend_handle.abort_handle().abort();
        }
    };

    res
}
