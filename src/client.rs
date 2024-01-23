use crate::{PORT, TOR_SOCKS_PORT};

use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Client {
    pub url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String) -> Result<Client, Error> {
        Ok(Client {
            url,
            client: reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(format!(
                    "socks5h://127.0.0.1:{}",
                    TOR_SOCKS_PORT
                ))?)
                .build()?,
        })
    }
    pub async fn send(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}:{}", self.url, PORT);
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
