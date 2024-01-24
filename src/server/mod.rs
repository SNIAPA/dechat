use std::sync::Arc;
use tokio::sync::Mutex;

use rocket::{
    config::{Config, LogLevel},
    post, routes, State,
};

use crate::PORT;

pub struct Server {
    pub state: Arc<Mutex<crate::tui::state::State>>,
}

#[post("/", data = "<message>")]
async fn test(message: String, state: &State<Arc<Mutex<Server>>>) {
    log::debug!("b state s");
    let server = state.lock().await;
    let mut state = server.state.lock().await;
    log::debug!("a state s");
    state.messages.push(message);
}

pub async fn rocket(server: Arc<Mutex<Server>>) {
    let mut config = Config::default();
    config.port = PORT;
    config.log_level = LogLevel::Off;

    rocket::custom(config)
        .mount("/", routes![test])
        .manage(server)
        .launch()
        .await
        .unwrap();
}
