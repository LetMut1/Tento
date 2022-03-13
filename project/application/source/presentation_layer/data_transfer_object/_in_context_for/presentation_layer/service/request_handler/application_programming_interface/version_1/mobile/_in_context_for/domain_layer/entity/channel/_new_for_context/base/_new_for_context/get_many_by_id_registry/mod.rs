use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct GetManyByIdRegistry {
    channel_id_registry: String,
}

impl GetManyByIdRegistry {
    pub fn into_inner(
        self
    ) -> String {
        return self.channel_id_registry;
    }
}