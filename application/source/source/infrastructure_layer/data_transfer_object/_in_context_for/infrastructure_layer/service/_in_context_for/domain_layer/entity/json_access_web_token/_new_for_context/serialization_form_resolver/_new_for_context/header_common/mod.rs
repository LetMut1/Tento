use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HeaderCommon<'outer_a> {
    #[serde(rename = "jawtpt")]
    json_access_web_token_payload_type: &'outer_a str,
}

impl<'outer_a> HeaderCommon<'outer_a> {
    pub fn new(
        json_access_web_token: &'outer_a JsonAccessWebToken<'_>
    ) -> Self {
        return Self {
            json_access_web_token_payload_type: json_access_web_token.get_type()
        };
    }
}