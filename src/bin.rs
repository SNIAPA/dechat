extern crate rocket;

use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::Result;
use dechat_lib::{server::rocket, tor::start_tor, tui::tui, client::Client};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<()> {

    let hostname = start_tor().await.unwrap();

    let hostname2 = hostname.clone();
    tokio::spawn(async move { tui(&hostname2).unwrap() });

    let client = Client {};
    tokio::spawn(async move { client.run(hostname).unwrap() });
    

    rocket().await;
    Ok(())
}
