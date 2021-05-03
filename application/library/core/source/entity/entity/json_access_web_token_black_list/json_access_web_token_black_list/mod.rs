use crate::entity::entity::json_access_web_token::core::payload::core::id::Id as JsonAccessWebTokenId;

pub struct JsonAccessWebTokenBlackList<'outer_a> {
    json_access_web_token_id: &'outer_a JsonAccessWebTokenId
}

impl<'this, 'outer_a: 'this> JsonAccessWebTokenBlackList<'outer_a> {
    pub fn new(json_access_web_token_id: &'outer_a JsonAccessWebTokenId) -> Self {
        return Self {
            json_access_web_token_id: json_access_web_token_id
        };
    }

    pub fn get_json_access_web_token_id(&'this self) -> &'this JsonAccessWebTokenId {
        return &self.json_access_web_token_id;
    }
}