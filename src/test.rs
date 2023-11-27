use std::io::{self, stdout, BufRead, Write};

use crate::protocol::{node::Node, Message};

//{  "src": "c1",  "dest": "n1",  "body": {    "type": "echo",    "msg_id": "1",    "echo": "Please echo 35"  }}
//{"src": "c1", "dest": "n1", "body": {"msg_id": 1, "type": "init", "node_id": "n1", "node_ids": ["n1"]}}

pub fn malestrom_test() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let node = Node::new();
    let mut stdin_handler = stdin.lock();
    let mut stdout_handler = stdout().lock();

    loop {
        stdin_handler.read_line(&mut buffer).unwrap();
        dbg!(&buffer);
        eprint!("req: {}", &buffer);
        let messages = buffer.trim().split("\n");
        for message in messages {
            let parsed_messages = serde_json::from_str::<Message>(&message).unwrap();
            let res = node.receive(parsed_messages).unwrap();
            let ans = format!("{}\n", serde_json::to_string(&res).unwrap());
            eprint!("ans: {}", &ans);
            stdout_handler.write_all(ans.as_bytes()).unwrap();
            print!("{}", ans);
        }
    }
}
