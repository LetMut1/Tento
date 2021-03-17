use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    device_id: String,
    token: String
}

impl<'this> Request {
    pub fn get_token_value(&'this self) -> &'this str {
        return self.token.as_str();
    }

    pub fn get_device_id(&'this self) -> &'this str {
        return self.device_id.as_str();
    }
}