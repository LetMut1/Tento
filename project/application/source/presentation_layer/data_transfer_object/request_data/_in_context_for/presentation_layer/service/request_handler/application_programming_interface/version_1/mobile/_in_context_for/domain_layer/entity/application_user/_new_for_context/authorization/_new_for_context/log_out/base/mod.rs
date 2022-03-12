pub struct Base {
    json_access_web_token: String
}

impl Base {
    pub fn new(
        json_access_web_token: String
    ) -> Self {
        return Self {
            json_access_web_token
        };
    }
    
    pub fn into_inner(
        self
    ) -> String {
        return self.json_access_web_token;
    }
}