use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common_from::CommonFrom;
use crate::entity::entity::json_web_token::json_access_web_token::core::header::header::Header;
use crate::entity::entity::json_web_token::json_access_web_token::core::payload::Payload;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;

pub struct JsonAccessWebToken<'outer> {
    payload: Payload<'outer>
}

impl<'this, 'outer: 'this> JsonAccessWebToken<'outer> {
    const HEADER: Header = Header::new();

    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Self {
        return Self {
            payload: Payload::new_from_json_refresh_web_token(json_refresh_web_token)
        };
    }

    pub fn new_from_payload_common_from(common_from: CommonFrom) -> Self {
        return Self {
            payload: Payload::new_from_common_from(common_from)
        };
    }

    pub fn get_header(&'this self) -> &'this Header {
        return &Self::HEADER;
    }

    pub fn get_payload(&'this self) -> &'this Payload<'outer> {
        return &self.payload;
    }
}