use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HeaderCommon<'outer_a> {
    #[serde(rename = "jawtpa")]
    json_access_web_token_payload_alg: &'outer_a str,
    #[serde(rename = "jawtpt")]
    json_access_web_token_payload_typ: &'outer_a str,
}

impl<'outer_a> HeaderCommon<'outer_a> {
    pub fn new(json_access_web_token: &'outer_a JsonAccessWebToken<'_>) -> Self {
        return Self {
            json_access_web_token_payload_alg: json_access_web_token.get_alg().get_value(),
            json_access_web_token_payload_typ: json_access_web_token.get_typ().get_value()
        };
    }
}