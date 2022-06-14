use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;

pub struct Base {
    update_email: bool,
    update_nickname: bool,
    update_password_hash: bool
}

impl Base {
    pub fn new(
        update_email: bool,
        update_nickname: bool,
        update_password_hash: bool
    ) -> Result<Self, ErrorAuditor> {
        if update_email || update_nickname || update_password_hash {
            return  Ok(
                Self {
                    update_email,
                    update_nickname,
                    update_password_hash
                }
            );
        }

        return Err(
            ErrorAuditor::new(
                BaseError::LogicError { logic_error: LogicError::new(false, "The columns allowing update should exist for ApplicationUser.") },
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }

    pub fn is_update_email<'a>(
        &'a self
    ) -> bool {
        return self.update_email;
    }

    pub fn is_update_nickname<'a>(
        &'a self
    ) -> bool {
        return self.update_nickname;
    }

    pub fn is_update_password_hash<'a>(
        &'a self
    ) -> bool {
        return self.update_password_hash;
    }
}