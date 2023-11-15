use rocket::{Rocket, config::Environment};

use crate::PORT;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> Rocket{
    Rocket::custom(
        rocket::Config::build(Environment::Staging)
            .port(PORT)
            .finalize()
            .unwrap(),
    )
    .mount("/", routes![index])
}
