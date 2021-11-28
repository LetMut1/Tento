use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "cir")]
    channel_id_registry: String,
}

impl Base {
    pub fn get_channel_id_registry(
        self
    ) -> String {
        return self.channel_id_registry;
    }
}