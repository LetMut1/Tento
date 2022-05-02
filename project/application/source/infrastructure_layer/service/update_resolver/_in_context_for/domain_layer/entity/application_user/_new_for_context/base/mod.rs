use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;

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
                ErrorAggregator::LogicError { logic_error: LogicError::new(false, "The columns allowing update should exist for ApplicationUser.") },
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }
}

impl UpdateResolverApplicationUserTrait for Base {
    fn is_update_email<'a>(
        &'a self
    ) -> bool {
        return self.update_email;
    }

    fn is_update_nickname<'a>(
        &'a self
    ) -> bool {
        return self.update_nickname;
    }

    fn is_update_password_hash<'a>(
        &'a self
    ) -> bool {
        return self.update_password_hash;
    }
}