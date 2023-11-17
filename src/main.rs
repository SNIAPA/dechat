use std::{
    fs,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    os::unix::prelude::PermissionsExt,
    panic::catch_unwind,
    time::Duration,
};

use anyhow::Result;
use backend::listen;
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

    tokio::spawn(async move {
        listen().await.unwrap();
    });

    loop {
        let _ = catch_unwind(|| {
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TOR_SOCKS_PORT);
            dbg!(0);
            let mut stream = tor_stream::TorStream::connect_with_address(
                socket,
                "nmqwq5ygh47sorbtixty6wmd4chxwfvza74qw4exok7hfdtk3fph3tad.onion:6131",
            )
            .unwrap();
            let mut buf = Vec::new();
            dbg!(1);
            stream.read_to_end(&mut buf).unwrap();
            dbg!(2);
            dbg!(String::from_utf8_lossy(&buf));
            stream.flush().unwrap();
            dbg!(3);
            stream.write(b"test ans").unwrap();
            dbg!(4);

        });
        std::thread::sleep(Duration::from_secs(1))
    }
}
