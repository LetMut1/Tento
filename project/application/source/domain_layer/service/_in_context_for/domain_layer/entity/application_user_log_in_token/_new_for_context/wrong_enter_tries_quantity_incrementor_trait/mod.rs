use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;

pub trait WrongEnterTriesQuantityIncrementorTrait {
    fn increment<'a>(
        application_user_log_in_token: &'a mut ApplicationUserLogInToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let wrong_enter_tries_quantity = *application_user_log_in_token.get_wrong_enter_tries_quantity();
        if wrong_enter_tries_quantity == u8::max_value() {
            return Err(
                ErrorAuditor::new(
                    ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Out of range for `u8` type.")},
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        application_user_log_in_token.set_wrong_enter_tries_quantity(wrong_enter_tries_quantity + 1);

        return Ok(());
    }
}