use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "di"))]
    pub device_id: String,
    #[serde(rename(deserialize = "n"))]
    pub nickname: String,
    #[serde(rename(deserialize = "p"))]
    pub password: String,
    #[serde(rename(deserialize = "e"))]
    email: String,
    #[serde(rename(deserialize = "tv"))]
    token_value: String
}

impl<'this> Request {
    pub fn get_email(&'this self) -> &'this str {
        return self.email.as_str();
    }

    pub fn get_token_value(&'this self) -> &'this str {
        return self.token_value.as_str();
    }

    pub fn get_nickname(&'this self) -> &'this str {
        return self.nickname.as_str();
    }
}