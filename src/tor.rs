use std::{
    fs,
    os::unix::prelude::PermissionsExt,
    thread::{sleep, JoinHandle},
    time::Duration,
};

use anyhow::{Error, Result};
use libtor::{HiddenServiceVersion, LogDestination, LogLevel, Tor, TorAddress, TorFlag};
use log::debug;

use crate::{PORT, TOR_SOCKS_PORT};

pub async fn start_tor(
    dir: &str,
) -> Result<(String, JoinHandle<Result<u8, libtor::Error>>), Error> {
    let hs_dir = format!("{dir}/hs");
    fs::create_dir_all(&hs_dir).unwrap();
    let mut perms = fs::metadata(&hs_dir).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(&hs_dir, perms).unwrap();
    let log_dir = hs_dir.clone() + "/tor.log";

    let handle = Tor::new()
        .flag(TorFlag::SocksPort(TOR_SOCKS_PORT))
        .flag(TorFlag::HiddenServiceDir(hs_dir.clone()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::LogTo(
            LogLevel::Info,
            LogDestination::File(log_dir),
        ))
        .flag(TorFlag::Quiet())
        .flag(TorFlag::HiddenServicePort(
            TorAddress::Port(PORT),
            Some(TorAddress::Port(PORT)).into(),
        ))
        .start_background();

    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(format!(
            "socks5h://127.0.0.1:{}",
            TOR_SOCKS_PORT
        ))?)
        .build()?;
    loop {
        let res = client
            .execute(client.get("https://check.torproject.org/").build().unwrap())
            .await;

        if let Err(e) = res {
            debug!("{}", e);
            sleep(Duration::from_secs(1));
            continue;
        }
        break;
    }
    let file_name = format!("{}/hostname", hs_dir);
    let mut hostname = fs::read_to_string(file_name).unwrap();
    hostname = hostname.strip_suffix("\n").unwrap().to_owned();

    Ok((hostname, handle))
}
