#![feature(unboxed_closures)]
#![feature(async_closure)]
#![feature(fn_traits)]

use std::{fs, process::exit, sync::Arc, thread::JoinHandle};

use client::Client;
use log::LevelFilter;
use server::{rocket, Server};
use tokio::sync::Mutex;
use tor::start_tor;
use tui::{state::State, tui};

pub mod client;
pub mod protocol;
pub mod server;
pub mod tor;
pub mod tui;

pub static PORT: u16 = 6131;
pub static TOR_SOCKS_PORT: u16 = 9052;

pub fn init_log(dir: &str) {
    let log_file = format!("{}/main.log", dir);
    simple_logging::log_to_file(log_file, LevelFilter::Debug).unwrap();
}

pub async fn run(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir).unwrap();

    init_log(dir);

    #[allow(unused)]
    let mut tor_handle: Option<JoinHandle<_>> = None;
    #[allow(unused)]
    let mut backend_handle = None;

    let res = {
        let (hostname, handle) = start_tor(dir).await?;
        tor_handle = Some(handle);

        let state = Arc::new(Mutex::new(State::new(hostname.as_str(), hostname.as_str())));
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
