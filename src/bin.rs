#![feature(async_closure)]
extern crate rocket;


use anyhow::Result;

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dechat_lib::run().await?;
    Ok(())
}
