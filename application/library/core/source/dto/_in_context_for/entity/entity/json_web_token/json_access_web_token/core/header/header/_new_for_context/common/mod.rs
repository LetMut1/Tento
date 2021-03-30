use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Common<'outer> {
    alg: &'outer str,
    typ: &'outer str,
}

impl<'outer> Common<'outer> {
    pub fn new(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            alg: json_access_web_token.get_alg().get_value(),
            typ: json_access_web_token.get_typ().get_value()
        };
    }
}