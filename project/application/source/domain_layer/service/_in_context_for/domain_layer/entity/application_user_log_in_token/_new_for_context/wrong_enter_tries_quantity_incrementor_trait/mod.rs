use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::error::error_aggregator::_component::logic_error::LogicError;
use std::convert::From;
use std::error::Error;

pub trait WrongEnterTriesQuantityIncrementorTrait {
    type Error: Error + From<LogicError>;

    fn increment<'a>(
        application_user_log_in_token: &'a mut ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error> {
        let wrong_enter_tries_quantity = *application_user_log_in_token.get_wrong_enter_tries_quantity();
        if wrong_enter_tries_quantity == u8::max_value() {
            return Err(LogicError::new(false, "Out of range for `u8` type."))?;
        }

        application_user_log_in_token.set_wrong_enter_tries_quantity(wrong_enter_tries_quantity + 1);

        return Ok(());
    }
}