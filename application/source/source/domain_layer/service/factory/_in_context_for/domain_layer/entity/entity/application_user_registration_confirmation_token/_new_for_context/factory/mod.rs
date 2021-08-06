use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::_component::value::Value;
use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::_component::wrong_enter_tries_quantity::WrongEnterTriesQuanity;
use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::_component::id::Id as PreConfirmedApplicationUserId;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Factory;

impl Factory {
    pub fn new_from_pre_confirmed_application_user<'outer_a>(
        pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<ApplicationUserRegistrationConfirmationToken<'_>, BaseError> {
        return Ok(
            ApplicationUserRegistrationConfirmationToken::new(
                pre_confirmed_application_user.get_id()?,
                Cow::Borrowed(pre_confirmed_application_user.get_email()),
                Value::new(Uuid::new_v4().to_string()),       // TODO создать генератор значения + метода Рефреш ниже
                WrongEnterTriesQuanity::new(0)
            )
        );
    }

    pub fn new_from_common<'outer_a>(
        common: Common<'_>, pre_confirmed_application_user_id: &'outer_a PreConfirmedApplicationUserId
    ) -> ApplicationUserRegistrationConfirmationToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) = common.into_inner();

        return ApplicationUserRegistrationConfirmationToken::new(
            pre_confirmed_application_user_id,
            Cow::Owned(Email::new(application_user_email.into_owned())),
            Value::new(value.into_owned()),
            WrongEnterTriesQuanity::new(wrong_enter_tries_quantity)
        );
    }
}