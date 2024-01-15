use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{protocol::Message, HS_DIR, PORT, TOR_SOCKS_PORT};

use anyhow::Result;

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }
    pub fn run(&self, nodes: &Vec<String>) -> Result<()> {
        //TODO: very temp
        let hostname = nodes.first().unwrap();
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TOR_SOCKS_PORT);
        let address = format!("{}:{}", hostname, PORT);

        if let Ok(mut stream) =
            tor_stream::TorStream::connect_with_address(socket, address.as_ref())
        {
            let message = Message {
                src: "c1".to_string(),
                dest: "c1".to_string(),
                body: crate::protocol::Body::Intit {
                    msg_id: 1,
                    node_id: "1".to_string(),
                    node_ids: vec![],
                },
            };
            let serialized = serde_json::to_string(&message).unwrap();

            dbg!("");
            stream.write_all(serialized.as_bytes()).unwrap();
            dbg!("");

            let mut message = String::new();
            dbg!("");
            stream.read_to_string(&mut message).unwrap();
            dbg!("client", message);
        }
        Ok(())
    }
}
