use std::{fs, os::unix::prelude::PermissionsExt, net::{SocketAddr, IpAddr, Ipv4Addr}, io::ErrorKind, time::Duration};

use libtor::{Tor, TorFlag, HiddenServiceVersion, TorAddress};
use anyhow::Result;

use crate::{HS_DIR, TOR_SOCKS_PORT, PORT};


pub async fn start_tor() -> Result<()> {
    fs::create_dir_all(HS_DIR).unwrap();
    let mut perms = fs::metadata(HS_DIR).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(HS_DIR, perms).unwrap();

    Tor::new()
        .flag(TorFlag::SocksPort(TOR_SOCKS_PORT))
        .flag(TorFlag::HiddenServiceDir(HS_DIR.into()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::HiddenServicePort(
            TorAddress::Port(PORT),
            Some(TorAddress::Port(PORT)).into(),
        ))
        .start_background();

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TOR_SOCKS_PORT);
    loop {
        let ans =
            tor_stream::TorStream::connect_with_address(socket, "https://check.torproject.org/");
        if let Err(e) = ans {
            if e.kind() == ErrorKind::ConnectionRefused {
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
        }
        break;
    }

    Ok(())
}
