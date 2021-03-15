use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }
}