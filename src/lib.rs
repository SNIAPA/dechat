#![feature(unboxed_closures)]
#![feature(async_closure)]
#![feature(fn_traits)]
#![feature(async_fn_in_trait)]

use std::{sync::Arc, thread::JoinHandle, fs};

use client::Client;
use log::LevelFilter;
use server::{rocket, Server};
use tokio::sync::Mutex;
use tor::start_tor;
use tui::{state::State, tui};

pub mod client;
pub mod server;
pub mod tor;
pub mod tui;

pub static PORT: u16 = 6131;
static TOR_SOCKS_PORT: u16 = 9052;
static HS_DIR: &str = "/tmp/dechat/hs";

pub fn init_log() {
    fs::create_dir_all(HS_DIR).unwrap();
    let log_file = format!("{}/main.log", HS_DIR);
    simple_logging::log_to_file(log_file, LevelFilter::Debug).unwrap();
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    init_log();

    let mut tor_handle: Option<JoinHandle<_>> = None;
    let mut backend_handle = None;

    let res = {
        let (hostname, handle) = start_tor().await?;
        tor_handle = Some(handle);

        let state = Arc::new(Mutex::new(State::new(hostname.as_str())));
        let client = Arc::new(Mutex::new(Client::new(state.clone())?));
        let server = Arc::new(Mutex::new(Server {
            state: state.clone(),
        }));

        backend_handle = Some(tokio::spawn(rocket(server.clone())));

        tui(client, state).await.unwrap();
        Ok(())
    };

    unsafe {
        if let Some(tor_handle) = tor_handle {
            stop_thread::kill_thread_graceful(tor_handle);
        }
        if let Some(backend_handle) = backend_handle {
            backend_handle.abort_handle().abort();
        }
    };

    res
}
