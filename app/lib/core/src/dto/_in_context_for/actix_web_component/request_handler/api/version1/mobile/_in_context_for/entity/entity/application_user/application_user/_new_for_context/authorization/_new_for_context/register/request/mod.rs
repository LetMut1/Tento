use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub device_id: String,
    pub nickname: String,
    pub password: String,
    email: String,
    token: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }

    pub fn get_token_value(&'this self) -> &'this str {
        return self.token.as_str();
    }

    pub fn get_nickname(&'this self) -> &'this str {
        return self.nickname.as_str();
    }
}