use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    nickname: String
}

impl<'this> Query {
    pub fn get_nickname(&'this self) -> &'this str {
        return self.nickname.as_str();
    }
}