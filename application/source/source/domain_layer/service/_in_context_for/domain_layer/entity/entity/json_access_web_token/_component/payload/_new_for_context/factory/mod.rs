use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::exp::Exp;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::id::Id;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::payload::Payload;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::proxed_type::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
use std::borrow::Cow;

pub struct Factory;

impl Factory {
    pub fn new_from_json_refresh_web_token<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Result<Payload<'outer_a>, BaseError> {
        return Ok(
            Payload::new(
                Cow::Borrowed(json_refresh_web_token.get_json_access_web_token_id()),
                Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
                Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
                Exp::new(DateTimeManipulator::add_interval_from_now(DateTimeExpirationStorage::QUANTITY_OF_MINUTES_JSON_ACCESS_WEB_TOKEN_FIRST)?)
            )
        );
    }

    pub fn new_from_payload_common(payload_common: PayloadCommon) -> Result<Payload<'static>, BaseError> {
        let (
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            exp
        ) = payload_common.into_inner();

        return Ok (
            Payload::new(
                Cow::Owned(Id::new_from_string(json_access_web_token_id)?),
                Cow::Owned(ApplicationUserId::new(application_user_id)),
                Cow::Owned(ApplicationUserLogInTokenDeviceId::new_from_string(application_user_log_in_token_device_id)?),
                Exp::new_from_str(exp.as_str())?
            )
        );
    }
}