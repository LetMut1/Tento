use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessTokenEncrypted {
    pub application_user_access_token_serialized: Vec<u8>,
    pub application_user_access_token_encoded: Vec<u8>,
}