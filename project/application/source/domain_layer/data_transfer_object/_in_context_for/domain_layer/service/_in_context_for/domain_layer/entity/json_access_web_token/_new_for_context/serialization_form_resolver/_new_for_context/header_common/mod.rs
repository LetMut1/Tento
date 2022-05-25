use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize)]
pub struct HeaderCommon<'a> {
    json_access_web_token_header_type: &'a str,
}

impl<'a> HeaderCommon<'a> {
    pub fn new(
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Self {
        return Self {
            json_access_web_token_header_type: json_access_web_token.get_type()
        };
    }
}