use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    email: String
}

impl<'this> Query {
    pub fn get_email(&'this self) -> &'this String {
        return &self.email;
    }
}