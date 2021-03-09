use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    token: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this String {
        return &self.email;
    }

    pub fn get_token(&'this self) -> &'this String {
        return &self.token;
    }
}