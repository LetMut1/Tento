use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct UserAccessTokenEncoded {
    pub serialized: Vec<u8>,
    pub encoded: Vec<u8>,
}
