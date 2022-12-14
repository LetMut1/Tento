use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;

pub struct WrongEnterTriesQuantityIncrementor;

impl WrongEnterTriesQuantityIncrementor {
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