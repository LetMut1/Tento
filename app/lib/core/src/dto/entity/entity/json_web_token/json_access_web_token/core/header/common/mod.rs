use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use maybe_owned::MaybeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'b> {
    alg: MaybeOwned<'b, String>,
    typ: MaybeOwned<'b, String>,
}

impl<'a, 'b: 'a> Common<'b> {
    pub fn new_from_entity(json_access_web_token: &'b JsonAccessWebToken<'a, 'b>) -> Self {
        return Self {
            alg: MaybeOwned::Owned(json_access_web_token.get_header().get_alg().get_value()),
            typ: MaybeOwned::Owned(json_access_web_token.get_header().get_typ().get_value())
        };
    }
}