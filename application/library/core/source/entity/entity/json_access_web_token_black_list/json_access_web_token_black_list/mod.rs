use crate::entity::core::uuid_v4::UuidV4;
use std::borrow::Cow;

pub struct JsonAccessWebTokenBlackList<'outer_a> {
    json_access_web_token_id: Cow<'outer_a, UuidV4>
}

impl<'this, 'outer_a: 'this> JsonAccessWebTokenBlackList<'outer_a> {
    pub fn new(json_access_web_token_id: &'outer_a UuidV4) -> Self {
        return Self {
            json_access_web_token_id: Cow::Borrowed(json_access_web_token_id)
        };
    }

    pub fn get_json_access_web_token_id(&'this self) -> &'this UuidV4 {
        return self.json_access_web_token_id.as_ref();
    }
}