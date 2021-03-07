use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
    nickname: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this String {
        return &self.email;
    }

    pub fn get_password(&'this self) -> &'this String {
        return &self.password;
    }

    pub fn get_nickname(&'this self) -> &'this String {
        return &self.nickname;
    }
}