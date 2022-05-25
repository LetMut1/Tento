use serde::Deserialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Serialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct Base {
    json_access_web_token: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> String {
        return self.json_access_web_token;
    }
}