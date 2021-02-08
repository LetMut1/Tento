pub mod core;

use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common;
use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use self::core::header::Header;
use self::core::payload::Payload;

pub struct JsonAccessWebToken<'b> {
    header: Header,
    payload: Payload<'b>
}

impl<'a, 'b: 'a> JsonAccessWebToken<'b> {
    pub fn new_from_jrwt(json_refresh_web_token: &'b JsonRefreshWebToken<'a, 'b>) -> Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_jrwt(json_refresh_web_token)
        };
    }

    pub fn new_from_payload_dto_common(common: Common<'b>) -> Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_dto_common(common)
        };
    }

    pub fn get_header(&'a self) -> &'a Header {
        return &self.header;
    }

    pub fn get_payload(&'a self) -> &'a Payload<'b> {
        return &self.payload;
    }
}