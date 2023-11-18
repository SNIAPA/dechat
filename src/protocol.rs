use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BodyType {
    #[serde(alias = "echo")]
    Echo,
    #[serde(alias = "echo_ok")]
    EchoOk,
}

#[derive(Serialize, Deserialize)]
pub enum Body {
    Echo {
        r#type: BodyType,
        msg_id: String,
        echo: String,
    },
    EchoOk {
        r#type: BodyType,
        msg_id: String,
        in_reply_to: String,
        echo: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}
