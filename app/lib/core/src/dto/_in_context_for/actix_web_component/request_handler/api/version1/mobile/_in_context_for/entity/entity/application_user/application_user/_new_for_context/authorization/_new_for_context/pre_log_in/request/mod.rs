use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "e"))]
    email: String,
    #[serde(rename(deserialize = "p"))]
    password: String,
    #[serde(rename(deserialize = "di"))]
    pub device_id: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }

    pub fn get_password(&'this self) -> &'this str {
        return self.password.as_str();
    }

    pub fn get_device_id(&'this self) -> &'this str {
        return self.device_id.as_str();
    }
}