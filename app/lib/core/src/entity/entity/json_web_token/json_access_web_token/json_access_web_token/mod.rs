use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::entity::json_web_token::json_access_web_token::core::header::header::Header;
use crate::entity::entity::json_web_token::json_access_web_token::core::payload::Payload;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use std::clone::Clone;

#[derive(Clone)]
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

    pub fn new_from_payload_common(common: Common<'outer>) -> Result<Self, ()> {
        return Ok(
            Self {
                payload: Payload::new_from_common(common)?
            }
        );
    }

    pub fn get_header(&'this self) -> &'this Header {
        return &Self::HEADER;
    }

    pub fn get_payload(&'this self) -> &'this Payload<'outer> {
        return &self.payload;
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id(&'this self) -> &'this UuidV4 {
        return &self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_json_refresh_web_token_value(&'this self) -> &'this Value {
        return &self.payload.get_json_refresh_web_token_value();
    }

    pub fn is_expired(&'this self) -> bool {
        return !DateTimeManipulator::is_greater_or_equal_than_now(&self.payload.get_exp());
    }

}