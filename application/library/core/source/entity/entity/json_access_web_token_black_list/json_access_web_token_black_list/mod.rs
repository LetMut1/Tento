use crate::entity::core::uuid_v4::UuidV4;
use std::borrow::Cow;

pub struct JsonAccessWebTokenBlackList<'outer> {
    json_access_web_token_id: Cow<'outer, UuidV4>
}

impl<'this, 'outer: 'this> JsonAccessWebTokenBlackList<'outer> {
    pub fn new(json_access_web_token_id: &'outer UuidV4) -> Self {
        return Self {
            json_access_web_token_id: Cow::Borrowed(json_access_web_token_id)
        };
    }

    pub fn get_json_access_web_token_id(&'this self) -> &'this UuidV4 {
        return self.json_access_web_token_id.as_ref();
    }
}