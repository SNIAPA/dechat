use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::{Ok, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{
    protocol::{node::Node, Message},
    PORT,
};

#[derive(Debug)]
pub struct Socket {}

impl Socket {
    pub fn new() -> Socket {
        Socket {}
    }
    pub async fn listen(&self, node: Arc<Mutex<Node>>) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            let node_ref = node.clone();
            tokio::spawn(async move {
                let mut message = String::new();
                dbg!("");
                stream.read_to_string(&mut message).await.unwrap();
                dbg!("server", message);
                dbg!("");
                stream.write_all(b"test").await.unwrap();
                dbg!("");
            });
        }
    }
}
