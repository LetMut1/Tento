use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
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