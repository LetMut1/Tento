use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadCommon<'a> {
    json_access_web_token_id: Cow<'a, str>,
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    json_access_web_token_expiration_time: Cow<'a, str>
}

impl<'a> PayloadCommon<'a> {
    pub fn new(
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Self {
        return Self {
            json_access_web_token_id: Cow::Borrowed(json_access_web_token.get_id()),
            application_user_id: json_access_web_token.get_application_user_id(),
            application_user_log_in_token_device_id: Cow::Borrowed(json_access_web_token.get_application_user_log_in_token_device_id()),
            json_access_web_token_expiration_time: Cow::Borrowed(json_access_web_token.get_expiration_time())
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, i64, Cow<'a, str>, Cow<'a, str>) {
        return (
            self.json_access_web_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.json_access_web_token_expiration_time
        );
    }
}