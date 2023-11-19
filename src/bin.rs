use anyhow::Result;
use dechat_lib::{client::Client, server::listen, tor::start_tor};

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;
pub static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<()> {
    start_tor().await?;

    tokio::spawn(async move {
        listen().await.unwrap();
    });

    let client = Client::new();
    dbg!(&client);
    client.run().unwrap();
    Ok(())
}
