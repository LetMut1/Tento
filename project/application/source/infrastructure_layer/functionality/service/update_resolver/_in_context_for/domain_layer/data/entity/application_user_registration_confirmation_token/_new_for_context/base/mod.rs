use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;

pub struct Base {
    update_wrong_enter_tries_quantity: bool,
    update_created_at: bool
}

impl Base {
    pub fn new(
        update_wrong_enter_tries_quantity: bool,
        update_created_at: bool
    ) -> Result<Self, ErrorAuditor> {
        if update_wrong_enter_tries_quantity || update_created_at {
            return  Ok(
                Self {
                    update_created_at,
                    update_wrong_enter_tries_quantity
                }
            );
        }

        return Err(
            ErrorAuditor::new(
                BaseError::LogicError { logic_error: LogicError::new(false, "The columns allowing update should exist for ApplicationUserRegistrationConfirmationToken.") },
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }

    pub fn is_update_wrong_enter_tries_quantity<'a>(
        &'a self
    ) -> bool {
        return self.update_wrong_enter_tries_quantity;
    }

    pub fn is_update_created_at<'a>(
        &'a self
    ) -> bool {
        return self.update_created_at;
    }
}