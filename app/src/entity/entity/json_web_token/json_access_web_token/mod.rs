pub mod core;

use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use self::core::header::Header;
use self::core::payload::Payload;

pub struct JsonAccessWebToken<'a> {
    header: Header,
    payload: Payload<'a>
}

impl<'a> JsonAccessWebToken<'a> {
    pub fn new_from_jrwt(json_refresh_web_token: &'a JsonRefreshWebToken) ->  Self {
        return Self {
            header: Header::new(),
            payload: Payload::new_from_jrwt(json_refresh_web_token)
        };
    }

    pub fn get_header(&'a self) -> &'a Header {
        return &self.header;
    }

    pub fn get_payload(&'a self) -> &'a Payload {
        return &self.payload;
    }
}