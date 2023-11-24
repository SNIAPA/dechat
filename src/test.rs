use std::io;

use crate::protocol::{node::Node, Message};

//{  "src": "c1",  "dest": "n1",  "body": {    "type": "echo",    "msg_id": "1",    "echo": "Please echo 35"  }}

pub fn malestrom_test() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let node = Node::new();

    stdin.read_line(&mut buffer).unwrap();
    let parsed_messages = serde_json::from_str::<Message>(&buffer).unwrap();
    let res = node.receive(parsed_messages);
    if let Some(res) = res {
        println!("{}", serde_json::to_string(&res).unwrap())
    }
}
