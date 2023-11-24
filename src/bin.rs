use std::{sync::{Mutex, Arc}, io};

use anyhow::Result;
use dechat_lib::{client::Client, tor::start_tor, protocol::node::Node, server::Socket, test::malestrom_test};
use clap::Parser;


pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    test: bool,

}

#[tokio::main]
async fn main() -> Result<()> {

    malestrom_test();
    return Ok(());

    let node = Arc::new(Mutex::new(Node::new()));

    start_tor().await?;

    tokio::spawn(async move {
        let socket = Socket::new();
        socket.listen(node).await.unwrap();
    });

    let client = Client::new();
    client.run().unwrap();
    Ok(())
}
