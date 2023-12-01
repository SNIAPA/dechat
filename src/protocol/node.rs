use anyhow::{anyhow, Result};

use super::{Body, Message};

#[derive(Debug)]
pub struct Node {}

impl Node {
    pub fn new() -> Node {
        Node {}
    }
    pub fn receive(&self, message: Message) -> Result<Message> {
        Ok(Message {
            src: message.dest,
            dest: message.src,
            body: match message.body {
                Body::Echo { msg_id, echo } => Ok(Body::EchoOk {
                    msg_id: msg_id.clone() + 1,
                    in_reply_to: msg_id,
                    echo,
                }),
                Body::Intit {
                    msg_id,
                    node_id: _,
                    node_ids: _,
                } => Ok(Body::IntitOk {
                    msg_id: msg_id.clone() + 1,
                    in_reply_to: msg_id,
                }),
                _ => Err(anyhow!("invalid or not implemented body")),
            }?,
        })
    }
}
