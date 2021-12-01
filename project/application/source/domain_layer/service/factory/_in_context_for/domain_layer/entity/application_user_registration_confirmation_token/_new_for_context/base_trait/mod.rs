use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::error::logic_error::LogicError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use std::convert::From;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error + From<LogicError>;
    type ValueGenerator: ValueGeneratorTrait;

    fn create_from_application_user_pre_confirmed<'a>(
        application_user_pre_confirmed: &'a ApplicationUserPreConfirmed
    ) -> Result<ApplicationUserRegistrationConfirmationToken<'_>, Self::Error> {
        return Ok(
            ApplicationUserRegistrationConfirmationToken::new(
                application_user_pre_confirmed.get_id()?,
                <Self::ValueGenerator as ValueGeneratorTrait>::generate(),
                0
            )
        );
    }
}