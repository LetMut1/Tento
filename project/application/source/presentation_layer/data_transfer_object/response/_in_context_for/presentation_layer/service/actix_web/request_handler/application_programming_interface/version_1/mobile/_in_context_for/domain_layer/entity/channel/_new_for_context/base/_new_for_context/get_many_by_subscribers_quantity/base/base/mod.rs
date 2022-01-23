use serde::Serialize;
use super::_component::channel::Channel;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub struct Base {
    channel_registry: Option<Vec<Channel>>
}

impl Base {
    pub fn new(
        channel_registry: Option<Vec<Channel>>
    ) -> Self {
        return Self {
            channel_registry
        };
    }
}