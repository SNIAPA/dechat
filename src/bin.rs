#![feature(async_closure)]
extern crate rocket;


use std::env;

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(deprecated)]
    let dir = env::home_dir().unwrap();
    let dir = dir.to_str().unwrap();
    let dir = format!("{dir}/.local/share/dechat");

    dechat_lib::run(&dir).await?;
    Ok(())
}
