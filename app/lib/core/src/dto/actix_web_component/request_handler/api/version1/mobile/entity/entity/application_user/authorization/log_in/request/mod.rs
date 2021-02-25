use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
    device_id: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this String {
        return &self.email;
    }

    pub fn get_password(&'this self) -> &'this String {
        return &self.password;
    }

    pub fn get_device_id(&'this self) -> &'this String {
        return &self.device_id;
    }
}