use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub struct Base {
    result: bool
}

impl Base {
    pub fn new(
        result: bool
    ) -> Self {
        return Self {
            result
        };
    }
}