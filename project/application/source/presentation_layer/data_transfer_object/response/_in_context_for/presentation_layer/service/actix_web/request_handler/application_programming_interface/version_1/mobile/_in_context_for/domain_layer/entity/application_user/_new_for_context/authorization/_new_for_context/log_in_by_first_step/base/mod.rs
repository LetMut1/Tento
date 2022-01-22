use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub struct Base {
    application_user_id: i64
}

impl Base {
    pub fn new(
        application_user_id: i64
    ) -> Self {
        return Self {
            application_user_id
        };
    }
}