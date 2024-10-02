use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct UserAccessRefreshTokenEncoded(pub Vec<u8>);
