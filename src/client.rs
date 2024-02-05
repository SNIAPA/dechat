use std::sync::Arc;

use crate::{protocol::message::Message, tui::state::State, PORT, TOR_SOCKS_PORT};

use anyhow::{Error, Result};
use serde_json::to_string;
use tokio::sync::Mutex;

pub struct Client {
    pub state: Arc<Mutex<crate::tui::state::State>>,
    client: reqwest::Client,
}

impl Client {
    pub fn new(state: Arc<Mutex<State>>) -> Result<Client, Error> {
        Ok(Client {
            state,
            client: reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(format!(
                    "socks5h://127.0.0.1:{}",
                    TOR_SOCKS_PORT
                ))?)
                .build()?,
        })
    }
    pub async fn send(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = self.state.lock().await;
        let to = state.url.clone();
        let url = format!("http://{}:{}", to, PORT);
        state.messages.push(format!("{}\n- {}",to,message));
        let message = Message {
            from: state.hostname.clone(),
            value: message.to_owned(),
        };
        let message = to_string(&message).unwrap();
        drop(state);
        self.client
            .execute(self.client.post(url).body(message).build().unwrap())
            .await?;
        Ok(())
    }
}
