use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Common<'outer> {
    #[serde(rename = "alg")]
    json_access_web_token_payload_alg: &'outer str,
    #[serde(rename = "typ")]
    json_access_web_token_payload_typ: &'outer str,
}

impl<'outer> Common<'outer> {
    pub fn new(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            json_access_web_token_payload_alg: json_access_web_token.get_alg().get_value(),
            json_access_web_token_payload_typ: json_access_web_token.get_typ().get_value()
        };
    }
}