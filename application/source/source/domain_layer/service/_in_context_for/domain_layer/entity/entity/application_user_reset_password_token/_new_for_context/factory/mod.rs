use crate::domain_layer::entity::entity::application_user_reset_password_token::_core::value::Value;
use crate::domain_layer::entity::entity::application_user_reset_password_token::_core::wrong_enter_tries_quantity::WrongEnterTriesQuanity;
use crate::domain_layer::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Factory;

impl Factory {
    pub fn new_from_application_user<'outer_a>(application_user: &'outer_a ApplicationUser<'_>) -> Result<ApplicationUserResetPasswordToken<'outer_a>, BaseError> {
        return Ok(
            ApplicationUserResetPasswordToken::new(
                application_user.get_id()?,
                Cow::Borrowed(application_user.get_email()),
                Value::new(Uuid::new_v4().to_string()),    // TODO создать генератор значения + метода Рефреш ниже
                WrongEnterTriesQuanity::new(0)
            )
        );
    }

    pub fn new_from_common<'outer_a>(common: Common<'_>, application_user_id: &'outer_a ApplicationUserId) -> ApplicationUserResetPasswordToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) = common.into_inner();

        return ApplicationUserResetPasswordToken::new(
            application_user_id,
            Cow::Owned(Email::new(application_user_email.into_owned())),
            Value::new(value.into_owned()),
            WrongEnterTriesQuanity::new(wrong_enter_tries_quantity)
        );
    }
}