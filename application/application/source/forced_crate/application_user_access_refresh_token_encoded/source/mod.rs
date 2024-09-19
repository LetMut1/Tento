use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessRefreshTokenEncoded(pub Vec<u8>);
