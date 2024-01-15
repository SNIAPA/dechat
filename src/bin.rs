use std::{io, sync::Arc};
use tokio::sync::Mutex;

use anyhow::Result;
use dechat_lib::{client::Client, protocol::node::Node, server::Socket, tor::start_tor};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<()> {
    let node = Arc::new(Mutex::new(Node::new()));

    let hostname = start_tor().await?;

    tokio::spawn(async move {
        let socket = Socket::new();
        socket.listen(node).await.unwrap();
    });

    let nodes = vec![hostname];
    let client = Client::new();
    client.run(&nodes).unwrap();
    Ok(())
}
