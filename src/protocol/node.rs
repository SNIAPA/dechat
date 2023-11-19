use super::Message;


#[derive(Debug)]
pub struct Node {
}

impl Node {
    pub fn new() -> Node {
        Node {}
    }
    pub fn receive(&self, message: Message) {
        dbg!(message);
    }
}
