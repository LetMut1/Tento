use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct ApplicationUserAccessTokenEncrypted(String);

impl ApplicationUserAccessTokenEncrypted {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}
