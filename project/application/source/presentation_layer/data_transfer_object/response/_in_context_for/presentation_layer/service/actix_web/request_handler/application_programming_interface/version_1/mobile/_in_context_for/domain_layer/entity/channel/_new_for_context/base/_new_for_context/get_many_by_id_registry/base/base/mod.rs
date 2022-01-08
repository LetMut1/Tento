use serde::Deserialize;
use serde::Serialize;
use super::_component::channel::Channel;

#[derive(Serialize, Deserialize)]
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