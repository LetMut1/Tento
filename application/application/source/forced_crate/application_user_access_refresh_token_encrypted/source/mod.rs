use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessRefreshTokenEncrypted(pub Vec<u8>);