use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
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