use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApplicationUserAccessRefreshTokenEncrypted(pub String);
