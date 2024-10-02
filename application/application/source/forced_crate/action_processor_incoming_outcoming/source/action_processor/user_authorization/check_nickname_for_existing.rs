use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub user__nickname: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub result: bool,
}
