use crate::dto::resourse_model::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::existing::Existing;
use crate::entity::core::uuid_v4::UuidV4;
use std::borrow::Cow;

pub struct JsonAccessWebTokenBlackList<'outer> {
    json_refresh_web_token_id: Cow<'outer, UuidV4>
}

impl<'this, 'outer: 'this> JsonAccessWebTokenBlackList<'outer> {
    pub fn new(json_refresh_web_token_id: &'outer UuidV4) -> Self {
        return Self {
            json_refresh_web_token_id: Cow::Borrowed(json_refresh_web_token_id)
        };
    }

    pub fn new_from_existing(existing: Existing) -> Self {
        return Self {
            json_refresh_web_token_id: Cow::Owned(UuidV4::new_from_uuid(existing.json_refresh_web_token_id))
        };
    }

    pub fn get_json_refresh_web_token_id(&'this self) -> &'this UuidV4 {
        return self.json_refresh_web_token_id.as_ref();
    }
}