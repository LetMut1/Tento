use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
use std::convert::From;
use std::error::Error;

pub trait WrongEnterTriesQuantityIncrementorTrait {
    type Error: Error + From<LogicError>;

    fn increment<'a>(
        application_user_registration_confirmation_token: &'a mut ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        let wrong_enter_tries_quantity = *application_user_registration_confirmation_token.get_wrong_enter_tries_quantity();
        if wrong_enter_tries_quantity == u8::max_value() {
            return Err(LogicError::new(false, "Out of range for `u8` type."))?;
        }

        application_user_registration_confirmation_token.set_wrong_enter_tries_quantity(wrong_enter_tries_quantity + 1);

        return Ok(());
    }
}