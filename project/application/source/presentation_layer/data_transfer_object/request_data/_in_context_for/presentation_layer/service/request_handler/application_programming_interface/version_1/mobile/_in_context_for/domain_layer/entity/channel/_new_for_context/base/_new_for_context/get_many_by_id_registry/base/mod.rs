pub struct Base {
    json_access_web_token: String,
    channel_id_registry: String,
}

impl Base {
    pub fn new(
        json_access_web_token: String,
        channel_id_registry: String,
    ) -> Self {
        return Self {
            json_access_web_token,
            channel_id_registry
        };
    }

    pub fn into_inner(
        self
    ) -> (String, String) {
        return (
            self.json_access_web_token,
            self.channel_id_registry,
        );
    }
}