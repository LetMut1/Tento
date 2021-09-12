use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadCommon<'outer_a> {
    #[serde(rename = "jawti")]
    json_access_web_token_id: Cow<'outer_a, str>,
    #[serde(rename = "aui")]
    application_user_id: Cow<'outer_a, i64>,
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: Cow<'outer_a, str>,
    #[serde(rename = "e")]
    exp: Cow<'outer_a, str>
}

impl<'outer_a> PayloadCommon<'outer_a> {
    pub fn new(
        json_access_web_token: &'outer_a JsonAccessWebToken<'_>
    ) -> Self {
        return Self {
            json_access_web_token_id: Cow::Borrowed(json_access_web_token.get_id()),
            application_user_id: Cow::Borrowed(json_access_web_token.get_application_user_id()),
            application_user_log_in_token_device_id: Cow::Borrowed(json_access_web_token.get_application_user_log_in_token_device_id()),
            exp: Cow::Borrowed(json_access_web_token.get_exp())
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'outer_a, str>, Cow<'outer_a, i64>, Cow<'outer_a, str>, Cow<'outer_a, str>) {
        return (
            self.json_access_web_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.exp
        );
    }
}