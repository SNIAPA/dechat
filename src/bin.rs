use std::sync::{Mutex, Arc};

use anyhow::Result;
use dechat_lib::{client::Client, tor::start_tor, protocol::node::Node, server::Socket};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<()> {

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
