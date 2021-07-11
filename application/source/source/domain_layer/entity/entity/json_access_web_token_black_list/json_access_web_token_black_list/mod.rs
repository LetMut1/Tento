use crate::domain_layer::entity::entity::json_access_web_token::_core::payload::_core::id::Id as JsonAccessWebTokenId;

pub struct JsonAccessWebTokenBlackList<'outer_a> {
    json_access_web_token_id: &'outer_a JsonAccessWebTokenId
}

impl<'outer_a> JsonAccessWebTokenBlackList<'outer_a> {
    pub fn new(json_access_web_token_id: &'outer_a JsonAccessWebTokenId) -> Self {
        return Self {
            json_access_web_token_id
        };
    }

    pub fn get_json_access_web_token_id<'this>(&'this self) -> &'this JsonAccessWebTokenId {
        return &self.json_access_web_token_id;
    }
}