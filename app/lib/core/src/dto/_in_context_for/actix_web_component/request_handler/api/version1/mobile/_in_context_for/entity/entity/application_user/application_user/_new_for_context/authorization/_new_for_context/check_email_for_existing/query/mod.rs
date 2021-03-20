use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename(deserialize = "e"))]
    email: String
}

impl<'this> Query {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }
}