pub struct JsonAccessWebTokenBlackList<'a> {
    json_access_web_token_id: &'a str
}

impl<'a> JsonAccessWebTokenBlackList<'a> {
    pub fn new(
        json_access_web_token_id: &'a str
    ) -> Self {
        return Self {
            json_access_web_token_id
        };
    }

    pub fn get_json_access_web_token_id<'b>(
        &'b self
    ) -> &'a str {
        return self.json_access_web_token_id;
    }
}