use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    channel_id_registry: String,
}

impl Base {
    pub fn into_inner(
        self
    ) -> String {
        return self.channel_id_registry;
    }
}