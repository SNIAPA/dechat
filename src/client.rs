use std::sync::Arc;

use crate::{PORT, TOR_SOCKS_PORT, tui::state::State};

use anyhow::{Error, Result};
use log::debug;
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
        let url = format!("http://{}:{}", state.url, PORT);
        state.messages.push(format!("- {message}"));
        drop(state);
        self.client
            .execute(
                self.client
                    .post(url)
                    .body(message.to_owned())
                    .build()
                    .unwrap(),
            )
            .await?;
        Ok(())
    }
}
