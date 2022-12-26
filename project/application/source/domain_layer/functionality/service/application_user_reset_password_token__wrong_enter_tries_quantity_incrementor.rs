use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;

#[allow(non_camel_case_types)]
pub struct ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor;

impl ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor {
    pub fn increment<'a>(
        application_user_reset_password_token: &'a mut ApplicationUserResetPasswordToken
    ) -> Result<(), ErrorAuditor> {
        let wrong_enter_tries_quantity = application_user_reset_password_token.get_wrong_enter_tries_quantity();
        if wrong_enter_tries_quantity == u8::max_value() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "Out of range for `u8` type.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        application_user_reset_password_token.set_wrong_enter_tries_quantity(wrong_enter_tries_quantity + 1);

        return Ok(());
    }
}