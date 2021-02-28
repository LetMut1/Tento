use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use maybe_owned::MaybeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]    // TODO подумать, нужнл
pub struct Common<'outer> {
    alg: MaybeOwned<'outer, String>,
    typ: MaybeOwned<'outer, String>,
}

impl<'outer> Common<'outer> {
    pub fn new_from_entity(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            alg: MaybeOwned::Owned(json_access_web_token.get_header().get_alg().get_value()),
            typ: MaybeOwned::Owned(json_access_web_token.get_header().get_typ().get_value())
        };
    }
}