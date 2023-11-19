#[cfg(test)]
mod test;

pub mod server;
pub mod protocol;
pub mod tor;
pub mod client;

pub static PORT: u16 = 6131;
static TOR_SOCKS_PORT: u16 = 9052;
static HS_DIR: &str = "/tmp/dechat/hs";

