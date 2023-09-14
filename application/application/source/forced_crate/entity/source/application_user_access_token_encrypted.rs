use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApplicationUserAccessTokenEncrypted(pub String);
