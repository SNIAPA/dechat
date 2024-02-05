use rocket::data::FromData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub value: String,
}
