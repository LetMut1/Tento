use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Incoming {
    pub user__nickname: String,
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Outcoming {
    pub result: bool,
}
