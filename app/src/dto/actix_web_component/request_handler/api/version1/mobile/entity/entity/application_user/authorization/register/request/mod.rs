use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
    nickname: String
}

impl<'a> Request {
    pub fn get_email(&'a self) -> &'a String {
        return &self.email;
    }

    pub fn get_password(&'a self) -> &'a String {
        return &self.password;
    }

    pub fn get_nickname(&'a self) -> &'a String {
        return &self.nickname;
    }
}