#![feature(unboxed_closures)]
#![feature(async_closure)]
#![feature(fn_traits)]
#![feature(async_fn_in_trait)]


pub mod server;
pub mod tor;
pub mod client;
pub mod tui;

pub static PORT: u16 = 6131;
static TOR_SOCKS_PORT: u16 = 9052;
static HS_DIR: &str = "/tmp/dechat/hs";

