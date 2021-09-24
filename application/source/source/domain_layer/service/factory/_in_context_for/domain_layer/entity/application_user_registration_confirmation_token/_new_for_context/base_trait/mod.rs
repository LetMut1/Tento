use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::error::logic_error::LogicError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use std::borrow::Cow;
use std::convert::From;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error + From<LogicError>;
    type ValueGenerator: ValueGeneratorTrait;

    fn create_from_application_user_pre_confirmed<'outer_a>(
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<ApplicationUserRegistrationConfirmationToken<'_>, Self::Error> {
        return Ok(
            ApplicationUserRegistrationConfirmationToken::new(
                application_user_pre_confirmed.get_id()?,
                Cow::Borrowed(application_user_pre_confirmed.get_application_user_email()),
                <Self::ValueGenerator as ValueGeneratorTrait>::generate(),
                0
            )
        );
    }

    fn create_from_common<'outer_a>(
        common: Common<'_>,
        application_user_pre_confirmed_id: &'outer_a i64
    ) -> ApplicationUserRegistrationConfirmationToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) : (
            Cow<'_, str>,
            Cow<'_, str>,
            Cow<'_, u8>
        ) = common.into_inner();

        return ApplicationUserRegistrationConfirmationToken::new(
            application_user_pre_confirmed_id,
            Cow::Owned(application_user_email.into_owned()),
            value.into_owned(),
            wrong_enter_tries_quantity.into_owned()
        );
    }
}