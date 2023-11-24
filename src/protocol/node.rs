use super::{Body, Message};

#[derive(Debug)]
pub struct Node {}

impl Node {
    pub fn new() -> Node {
        Node {}
    }
    pub fn receive(&self, message: Message) -> Option<Message> {
        match message.body {
            Body::Echo { msg_id, echo } => Some(Message {
                src: message.dest,
                dest: message.src,
                body: Body::EchoOk {
                    msg_id: msg_id.clone(),
                    in_reply_to: msg_id,
                    echo,
                },
            }),
            Body::Intit {
                msg_id,
                node_id,
                node_ids,
            } => Some(Message { src: message.dest, dest: message.src, body: Body::IntitOk { in_reply_to: msg_id } }),

            _ => None,
        }
    }
}
