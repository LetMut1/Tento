use crate::data_transfer_object::_in_context_for::entity::entity::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::entity::entity::json_access_web_token::core::header::header::Header;
use crate::entity::entity::json_access_web_token::core::payload::core::id::Id;
use crate::entity::entity::json_access_web_token::core::payload::payload::Payload;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use std::clone::Clone;
use super::core::header::core::alg::Alg;
use super::core::header::core::typ::Typ;

#[derive(Clone)]
pub struct JsonAccessWebToken<'outer_a> {
    payload: Payload<'outer_a>
}

impl<'this, 'outer_a: 'this> JsonAccessWebToken<'outer_a> {
    const HEADER: Header = Header::new();

    pub fn new(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'outer_a>) -> Self {
        return Self {
            payload: Payload::new(json_refresh_web_token)
        };
    }

    pub fn new_from_payload_common(common: Common) -> Result<Self, InvalidArgumentError> {
        return Ok(
            Self {
                payload: Payload::new_from_common(common)?
            }
        );
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

    pub fn get_id(&'this self) -> &'this Id {
        return &self.payload.get_id();
    }

    pub fn get_application_user_id(&'this self) -> &'this ApplicationUserId {
        return &self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id(&'this self) -> &'this ApplicationUserLogInTokenDeviceId {
        return &self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_exp(&'this self) -> &'this DateTime {
        return &self.payload.get_exp();
    }
}