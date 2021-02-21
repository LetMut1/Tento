use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    nickname: String
}

impl<'a> Request {

    pub fn get_nickname(&'a self) -> &'a String {
        return &self.nickname;
    }
}