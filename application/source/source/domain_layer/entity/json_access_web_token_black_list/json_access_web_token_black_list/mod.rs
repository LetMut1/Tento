pub struct JsonAccessWebTokenBlackList<'outer_a> {
    json_access_web_token_id: &'outer_a str
}

impl<'outer_a> JsonAccessWebTokenBlackList<'outer_a> {
    pub fn new(json_access_web_token_id: &'outer_a str) -> Self {
        return Self {
            json_access_web_token_id
        };
    }

    pub fn get_json_access_web_token_id<'this>(&'this self) -> &'this str {
        return self.json_access_web_token_id;
    }
}