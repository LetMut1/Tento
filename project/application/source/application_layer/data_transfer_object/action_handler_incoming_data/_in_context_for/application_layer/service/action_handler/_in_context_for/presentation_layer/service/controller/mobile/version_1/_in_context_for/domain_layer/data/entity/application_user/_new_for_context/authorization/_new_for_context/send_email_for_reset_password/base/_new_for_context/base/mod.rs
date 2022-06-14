use serde::Deserialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Serialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct Base {
    application_user_id: i64
}

impl Base {
    pub fn into_inner(
        self
    ) -> i64 {
        return self.application_user_id;
    }
}