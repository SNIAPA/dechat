extern crate rocket;

use std::{sync::Arc, thread};
use tokio::sync::Mutex;

use anyhow::Result;
use dechat_lib::{client::Client, server::rocket, tor::start_tor, tui::tui};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<()> {
    let (hostname, tor_handle) = start_tor().await.unwrap();

    let client = Arc::new(Mutex::new(Client::new(hostname).unwrap()));

    let bakcend_handle = tokio::spawn(rocket());

    tui(client).unwrap();
    unsafe {
        stop_thread::kill_thread_graceful(tor_handle);
        bakcend_handle.abort_handle().abort();
    }

    Ok(())
}
