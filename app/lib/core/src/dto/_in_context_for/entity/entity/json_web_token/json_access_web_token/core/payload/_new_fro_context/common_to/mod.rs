use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CommonTo<'outer> {
    application_user_id: String,
    device_id: &'outer str,
    json_refresh_web_token_value: &'outer str,
    exp: String
}

impl<'outer> CommonTo<'outer> {
    pub fn new(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            application_user_id: json_access_web_token.get_payload().get_application_user_id().get_value().to_string(),
            device_id: json_access_web_token.get_payload().get_device_id().get_value(),
            json_refresh_web_token_value: json_access_web_token.get_payload().get_value().get_value(),
            exp: json_access_web_token.get_payload().get_exp().get_value().to_rfc3339()
        };
    }
}