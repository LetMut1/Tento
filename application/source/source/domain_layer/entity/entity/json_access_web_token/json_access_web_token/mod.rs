use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_access_web_token::_core::header::header::Header;
use crate::domain_layer::entity::entity::json_access_web_token::_core::payload::_core::exp::Exp;
use crate::domain_layer::entity::entity::json_access_web_token::_core::payload::_core::id::Id;
use crate::domain_layer::entity::entity::json_access_web_token::_core::payload::payload::Payload;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::proxed_type::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use std::clone::Clone;
use super::_core::header::_core::alg::Alg;
use super::_core::header::_core::typ::Typ;

#[derive(Clone)]
pub struct JsonAccessWebToken<'outer_a> {
    payload: Payload<'outer_a>
}

impl<'outer_a> JsonAccessWebToken<'outer_a> {
    const HEADER: Header = Header::new();

    pub fn new(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Result<Self, BaseError> {
        return Ok(
            Self {
                payload: Payload::new(json_refresh_web_token)?
            }
        );
    }

    pub fn new_from_common(payload_common: PayloadCommon) -> Result<Self, BaseError> {
        return Ok(
            Self {
                payload: Payload::new_from_common(payload_common)?
            }
        );
    }

    pub fn is_expired<'this>(&'this self) -> bool {
        return !DateTimeManipulator::is_greater_or_equal_than_now(&self.payload.get_exp().get_value());
    }

    pub fn get_alg<'this>(&'this self) -> &'this Alg {
        return Self::HEADER.get_alg();
    }

    pub fn get_typ<'this>(&'this self) -> &'this Typ {
        return Self::HEADER.get_typ();
    }

    pub fn get_id<'this>(&'this self) -> &'this Id {
        return &self.payload.get_id();
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.payload.get_application_user_id();
    }

    pub fn get_application_user_log_in_token_device_id<'this>(&'this self) -> &'this ApplicationUserLogInTokenDeviceId {
        return &self.payload.get_application_user_log_in_token_device_id();
    }

    pub fn get_exp<'this>(&'this self) -> &'this Exp {
        return &self.payload.get_exp();
    }
}