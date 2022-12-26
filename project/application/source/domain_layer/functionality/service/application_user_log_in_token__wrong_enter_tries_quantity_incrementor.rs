use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;

#[allow(non_camel_case_types)]
pub struct ApplicationUserLogInToken_WrongEnterTriesQuantityIncrementor;

impl ApplicationUserLogInToken_WrongEnterTriesQuantityIncrementor {
    pub fn increment<'a>(
        application_user_log_in_token: &'a mut ApplicationUserLogInToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let wrong_enter_tries_quantity = application_user_log_in_token.get_wrong_enter_tries_quantity();
        if wrong_enter_tries_quantity == u8::max_value() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "Out of range for `u8` type.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        application_user_log_in_token.set_wrong_enter_tries_quantity(wrong_enter_tries_quantity + 1);

        return Ok(());
    }
}