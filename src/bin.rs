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
    tui::{state::State, tui},
};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dechat_lib::run().await?;
    Ok(())
}
