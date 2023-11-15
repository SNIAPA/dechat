#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::{
    fs,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    os::unix::prelude::PermissionsExt,
    panic::catch_unwind,
    time::Duration,
};

use anyhow::Result;
use arti_client::*;
use backend::rocket;
use libtor::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod backend;

pub static PORT: u16 = 6131;
static TOR_SOCKS_PORT: u16 = 9052;
static HS_DIR: &str = "/tmp/dechat/hs";

#[tokio::main]
async fn main() -> Result<()> {
    fs::create_dir_all(HS_DIR).unwrap();
    let mut perms = fs::metadata(HS_DIR).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(HS_DIR, perms).unwrap();
    Tor::new()
        .flag(TorFlag::SocksPort(TOR_SOCKS_PORT))
        .flag(TorFlag::Log(LogLevel::Err))
        .flag(TorFlag::HiddenServiceDir(HS_DIR.into()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::HiddenServicePort(
            TorAddress::Port(PORT),
            Some(TorAddress::Port(PORT)).into(),
        ))
        .start_background();

    tokio::spawn(async move { rocket().launch() });

    loop {
        let _ = catch_unwind(|| {
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TOR_SOCKS_PORT);
            let mut stream =
                tor_stream::TorStream::connect_with_address(socket, "kpjqf7gb7yoo5jk536p75wzyujjipswfraytmfs7irwot4mh5dmxhgyd.onion:6131").unwrap();
            stream
            .write_all(b"GET / HTTP/1.1\r\nHost: kpjqf7gb7yoo5jk536p75wzyujjipswfraytmfs7irwot4mh5dmxhgyd.onion\r\nConnection: close\r\n\r\n").unwrap();
            stream.flush().unwrap();
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).unwrap();

            dbg!(String::from_utf8_lossy(&buf));
        });
        std::thread::sleep(Duration::from_secs(5))
    }
}
