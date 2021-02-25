use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common;
use crate::entity::entity::json_web_token::json_access_web_token::core::header::Header;
use crate::entity::entity::json_web_token::json_access_web_token::core::payload::Payload;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;

pub struct JsonAccessWebToken<'outer> {
    header: Header,
    payload: Payload<'outer, 'outer>
}

impl<'this, 'outer: 'this> JsonAccessWebToken<'outer> {
    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer, 'outer>) -> Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_json_refresh_web_token(json_refresh_web_token)
        };
    }

    pub fn new_from_payload_dto_common(common: &'outer Common<'outer>) -> Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_dto_common(common)
        };
    }

    pub fn get_header(&'this self) -> &'this Header {
        return &self.header;
    }

    pub fn get_payload(&'this self) -> &'this Payload<'outer, 'outer> {
        return &self.payload;
    }
}