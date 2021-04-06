use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_access_web_token::core::header::header::Header;
use crate::entity::entity::json_web_token::json_access_web_token::core::payload::Payload;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use std::clone::Clone;
use super::core::header::core::alg::Alg;
use super::core::header::core::typ::Typ;

#[derive(Clone)]
pub struct JsonAccessWebToken<'outer> {
    payload: Payload<'outer>
}

impl<'this, 'outer: 'this> JsonAccessWebToken<'outer> {
    const HEADER: Header = Header::new();

    pub fn new(
        json_refresh_web_token: &'outer JsonRefreshWebToken,
        application_user_id: &'outer UuidV4, 
        application_user_log_in_token_device_id: &'outer UuidV4
    ) -> Self {
        return Self {
            payload: Payload::new(json_refresh_web_token, application_user_id, application_user_log_in_token_device_id)
        };
    }

    pub fn new_from_payload_common(common: Common) -> Result<Self, InvalidArgumentError> {
        return Ok(
            Self {
                payload: Payload::new_from_common(common)?
            }
        );
    }

    pub fn refresh(&'this mut self) -> &'this mut Self {
        self.payload.refresh();

        return self;
    }

    pub fn is_expired(&'this self) -> bool {
        return !DateTimeManipulator::is_greater_or_equal_than_now(&self.payload.get_exp());
    }

    pub fn get_alg(&'this self) -> &'this Alg {
        return Self::HEADER.get_alg();
    }

    pub fn get_typ(&'this self) -> &'this Typ {
        return Self::HEADER.get_typ();
    }

    pub fn get_id(&'this self) -> &'this UuidV4 {
        return &self.payload.get_id();
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id(&'this self) -> &'this UuidV4 {
        return &self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_exp(&'this self) -> &'this DateTime {
        return &self.payload.get_exp();
    }
}