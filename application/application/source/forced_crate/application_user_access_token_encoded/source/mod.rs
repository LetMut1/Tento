use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessTokenEncoded {
    pub serialized: Vec<u8>,
    pub encoded: Vec<u8>,
}
