use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    nickname: String
}

impl<'a> Query {
    pub fn get_nickname(&'a self) -> &'a String {
        return &self.nickname;
    }
}