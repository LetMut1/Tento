use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct GetManyByIdRegistry {
    channel_id_registry: Vec<i64>,
}

impl GetManyByIdRegistry {
    pub fn into_inner(
        self
    ) -> Vec<i64> {
        return self.channel_id_registry;
    }
}