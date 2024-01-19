use std::{
    fs,
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    os::unix::prelude::PermissionsExt,
    time::Duration,
};

use anyhow::Result;
use libtor::{HiddenServiceVersion, LogDestination, LogLevel, Tor, TorAddress, TorFlag};

use crate::{HS_DIR, PORT, TOR_SOCKS_PORT};

pub async fn start_tor() -> Result<String> {
    fs::create_dir_all(HS_DIR).unwrap();
    let mut perms = fs::metadata(HS_DIR).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(HS_DIR, perms).unwrap();
    let log_dir = HS_DIR.to_owned() + "/log";

    Tor::new()
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

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TOR_SOCKS_PORT);
    loop {
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::http(format!(
                "socks5://127.0.0.1:{}",
                TOR_SOCKS_PORT
            ))?)
            .build()?;
        let ans =client.get("https://check.torproject.org/").build();
        if let Err(e) = ans {
            dbg!(e);
        }
        break;
    }
    let file_name = format!("{}/hostname", HS_DIR);
    let mut hostname = fs::read_to_string(file_name).unwrap();
    hostname = hostname.strip_suffix("\n").unwrap().to_owned();

    Ok(hostname)
}
