use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct UserAccessTokenEncoded {
    pub serialized: Vec<u8>,
    pub encoded: Vec<u8>,
}
