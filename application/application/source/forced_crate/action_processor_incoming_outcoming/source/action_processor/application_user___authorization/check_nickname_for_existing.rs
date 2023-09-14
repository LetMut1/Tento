use entity::application_user::ApplicationUser_Nickname;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_nickname: ApplicationUser_Nickname,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    pub result: bool,
}
