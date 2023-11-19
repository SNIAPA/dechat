use std::sync::{Arc, Mutex};

use anyhow::{Result, Ok};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::{protocol::{node::Node, Message}, PORT};

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
                let mut buf = String::new();
                dbg!("before");
                stream.read_to_string(&mut buf).await.unwrap();
                dbg!("after");

                let message = serde_json::from_str::<Message>(&buf).unwrap();
                dbg!("1");

                let node = node_ref.lock().unwrap();
                dbg!("2");
                node.receive(message);
                dbg!("3");

                Ok(())
            });
        }
    }
}
