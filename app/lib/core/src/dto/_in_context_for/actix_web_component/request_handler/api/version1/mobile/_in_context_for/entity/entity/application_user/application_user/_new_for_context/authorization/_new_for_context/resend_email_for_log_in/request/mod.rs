use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "di"))]
    device_id: String,
    #[serde(rename(deserialize = "e"))]
    email: String
}

impl<'this> Request {
    pub fn get_device_id(&'this self) -> &'this str {
        return self.device_id.as_str();
    }

    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }
}