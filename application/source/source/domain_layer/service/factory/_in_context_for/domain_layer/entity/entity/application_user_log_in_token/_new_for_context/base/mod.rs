use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId;
use crate::domain_layer::entity::entity::application_user_log_in_token::_component::value::Value;
use crate::domain_layer::entity::entity::application_user_log_in_token::_component::wrong_enter_tries_quantity::WrongEnterTriesQuanity;
use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Base;

impl Base {
    pub fn new_from_application_user<'outer_a>(application_user: &'outer_a ApplicationUser<'_>, device_id: &'outer_a DeviceId) -> Result<ApplicationUserLogInToken<'outer_a>, BaseError> {
        return Ok(
            ApplicationUserLogInToken::new(
                application_user.get_id()?,
                device_id,
                Cow::Borrowed(application_user.get_email()),
                Value::new(Uuid::new_v4().to_string()),       // TODO создать генератор значения + метода Рефреш в модели
                WrongEnterTriesQuanity::new(0)
            )
        );
    }

    pub fn new_from_common<'outer_a>(common: Common<'_>, application_user_id: &'outer_a ApplicationUserId, device_id: &'outer_a DeviceId) -> ApplicationUserLogInToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) = common.into_inner();

        return ApplicationUserLogInToken::new(
            application_user_id,
            device_id,
            Cow::Owned(Email::new(application_user_email.into_owned())),
            Value::new(value.into_owned()),
            WrongEnterTriesQuanity::new(wrong_enter_tries_quantity)
        );
    }
}