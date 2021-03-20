use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "di"))]
    device_id: String,
    #[serde(rename(deserialize = "aui"))]
    application_user_id: String,
    #[serde(rename(deserialize = "tv"))]
    token_value: String
}

impl<'this> Request {
    pub fn get_device_id(&'this self) -> &'this str {
        return self.device_id.as_str();
    }

    pub fn get_application_user_id(&'this self) -> &'this str {
        return self.application_user_id.as_str();
    }

    pub fn get_token_value(&'this self) -> &'this str {
        return self.token_value.as_str();
    }
}