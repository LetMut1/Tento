use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub email: String,
    pub password: String,
    pub nickname: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }

    pub fn get_password(&'this self) -> &'this str {
        return self.password.as_str();
    }

    pub fn get_nickname(&'this self) -> &'this str {
        return self.nickname.as_str();
    }
}