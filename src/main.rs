use anyhow::Result;
use libtor::*;
use rocket::{config::Environment, Rocket};
use tokio::io::AsyncReadExt;

static PORT: u16 = 6131;
static TOR_CONTROL_PORT: u16 = 9051;
static TOR_SOCKS_PORT: u16 = 9052;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(async move {
        Tor::new()
            .flag(TorFlag::ControlPort(TOR_CONTROL_PORT))
            .flag(TorFlag::SocksPort(TOR_SOCKS_PORT))
            .flag(TorFlag::HiddenServiceDir("/tmp/dechat/hs".into()))
            .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
            .flag(TorFlag::HiddenServicePort(
                TorAddress::Port(PORT),
                None.into(),
            ))
            .start()
            .unwrap();
        
    });

    tokio::spawn(async move {
        Rocket::custom(
            rocket::Config::build(Environment::Staging)
                .port(PORT)
                .finalize()
                .unwrap(),
        )
        .launch();
        
    });

    let config = TorClientConfig::default();

    // Start the Arti client, and let it bootstrap a connection to the Tor network.
    // (This takes a while to gather the necessary directory information.
    // It uses cached information when possible.)
    let tor_client = TorClient::create_bootstrapped(config).await?;

    Ok(())
}
