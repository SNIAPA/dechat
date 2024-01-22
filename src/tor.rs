use std::{
    fs,
    os::unix::prelude::PermissionsExt,
    time::Duration, thread::{sleep, JoinHandle},
};

use anyhow::{Result, Error};
use libtor::{HiddenServiceVersion, LogDestination, LogLevel, Tor, TorAddress, TorFlag};

use crate::{HS_DIR, PORT, TOR_SOCKS_PORT};

pub async fn start_tor() -> Result<(String, JoinHandle<Result<u8,libtor::Error>>), Error> {
    fs::create_dir_all(HS_DIR).unwrap();
    let mut perms = fs::metadata(HS_DIR).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(HS_DIR, perms).unwrap();
    let log_dir = HS_DIR.to_owned() + "/log";

    let handle = Tor::new()
        .flag(TorFlag::SocksPort(TOR_SOCKS_PORT))
        .flag(TorFlag::HiddenServiceDir(HS_DIR.into()))
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

    loop {

        let res = || -> Result<(),reqwest::Error> {
            let client = reqwest::Client::builder()
                .proxy(reqwest::Proxy::http(format!(
                    "http://127.0.0.1:{}",
                    TOR_SOCKS_PORT
                ))?)
                .build()?;
            client.get("https://check.torproject.org/").build();
            Ok(())
        }();

        if let Err(e) = res {
            dbg!(e);
            sleep(Duration::from_secs(1));
            continue;
        }
        break;
    }
    let file_name = format!("{}/hostname", HS_DIR);
    let mut hostname = fs::read_to_string(file_name).unwrap();
    hostname = hostname.strip_suffix("\n").unwrap().to_owned();

    Ok((hostname, handle))
}
