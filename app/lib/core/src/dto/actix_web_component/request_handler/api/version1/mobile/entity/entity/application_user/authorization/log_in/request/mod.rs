use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
    device_id: String
}

impl<'a> Request {
    pub fn get_email(&'a self) -> &'a String {
        return &self.email;
    }

    pub fn get_password(&'a self) -> &'a String {
        return &self.password;
    }

    pub fn get_device_id(&'a self) -> &'a String {
        return &self.device_id;
    }
}