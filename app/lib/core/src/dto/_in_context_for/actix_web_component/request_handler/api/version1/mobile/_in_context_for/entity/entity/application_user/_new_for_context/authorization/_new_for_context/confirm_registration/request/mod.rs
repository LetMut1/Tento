use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    token: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }

    pub fn get_token(&'this self) -> &'this str {
        return self.token.as_str();
    }
}