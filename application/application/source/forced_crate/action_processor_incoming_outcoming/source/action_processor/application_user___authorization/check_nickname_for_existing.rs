use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_nickname: String,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub result: bool,
}
