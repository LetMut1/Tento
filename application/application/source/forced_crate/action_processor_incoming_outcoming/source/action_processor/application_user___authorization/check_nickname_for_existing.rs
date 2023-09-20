use entity::application_user::ApplicationUser_Nickname;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_nickname: ApplicationUser_Nickname,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub result: bool,
}
