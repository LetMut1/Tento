use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Base;

impl Base {
    pub fn new_from_application_user_pre_confirmed<'outer_a>(
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<ApplicationUserRegistrationConfirmationToken<'_>, BaseError> {
        return Ok(
            ApplicationUserRegistrationConfirmationToken::new(
                application_user_pre_confirmed.get_id()?,
                Cow::Borrowed(application_user_pre_confirmed.get_application_user_email()),
                Uuid::new_v4().to_string(),       // TODO создать генератор значения + метода Рефреш ниже
                0
            )
        );
    }

    pub fn new_from_common<'outer_a>(
        common: Common<'_>,
        application_user_pre_confirmed_id: &'outer_a i64
    ) -> ApplicationUserRegistrationConfirmationToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) = common.into_inner();

        return ApplicationUserRegistrationConfirmationToken::new(
            application_user_pre_confirmed_id,
            Cow::Owned(application_user_email.into_owned()),
            value.into_owned(),
            wrong_enter_tries_quantity.into_owned()
        );
    }
}