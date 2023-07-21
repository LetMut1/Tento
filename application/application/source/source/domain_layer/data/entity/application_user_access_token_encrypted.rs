use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct ApplicationUserAccessTokenEncrypted(pub String);