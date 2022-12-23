use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor;

impl ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor {
    pub fn increment<'a>(
        application_user_registration_confirmation_token: &'a mut ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let wrong_enter_tries_quantity = application_user_registration_confirmation_token.get_wrong_enter_tries_quantity();
        if wrong_enter_tries_quantity == u8::max_value() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "Out of range for `u8` type.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        application_user_registration_confirmation_token.set_wrong_enter_tries_quantity(wrong_enter_tries_quantity + 1);

        return Ok(());
    }
}