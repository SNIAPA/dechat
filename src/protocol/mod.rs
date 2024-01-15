use serde::{Deserialize, Serialize};

pub mod node;


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Body {
    #[serde(rename = "echo")]
    Echo {
        msg_id: usize,
        echo: String,
    },
    #[serde(rename = "echo_ok")]
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    #[serde(rename = "init")]
    Intit {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "init_ok")]
    IntitOk {
        msg_id: usize,
        in_reply_to: usize
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}
