use entity::application_user::ApplicationUser_Email;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_email: ApplicationUser_Email,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub result: bool,
}
