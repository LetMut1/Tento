use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common;
use crate::entity::entity::json_web_token::json_access_web_token::core::header::Header;
use crate::entity::entity::json_web_token::json_access_web_token::core::payload::Payload;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;

pub struct JsonAccessWebToken<'b> {
    header: Header,
    payload: Payload<'b, 'b>
}

impl<'a, 'b: 'a> JsonAccessWebToken<'b> {
    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'b JsonRefreshWebToken<'b, 'b>) -> Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_json_refresh_web_token(json_refresh_web_token)
        };
    }

    pub fn new_from_payload_dto_common(common: &'b Common<'b>) -> Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_dto_common(common)
        };
    }

    pub fn get_header(&'a self) -> &'a Header {
        return &self.header;
    }

    pub fn get_payload(&'a self) -> &'a Payload<'b, 'b> {
        return &self.payload;
    }
}