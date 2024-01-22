use rocket::{config::{Config, LogLevel}, get, launch, routes};

use crate::PORT;

#[get("/")]
fn test() -> String {
    format!("Hello")
}

pub async fn rocket() {
    let mut config = Config::default();
    config.port = PORT;
    config.log_level = LogLevel::Off;

    rocket::custom(config).mount("/", routes![hello]).launch().await.unwrap();
}
