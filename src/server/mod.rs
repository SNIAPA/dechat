use std::sync::Arc;
use tokio::sync::Mutex;

use rocket::{
    config::{Config, LogLevel},
    post, routes, State,
};

use crate::PORT;

pub struct Server {
    pub messages: Vec<String>,
}

#[post("/", data = "<message>")]
async fn test(message: String, state: &State<Arc<Mutex<Server>>>) {
    let mut server = state.lock().await;
    server.messages.push(message);
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
