use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_encoder::PasswordEncoder;
use super::password_encoder_trait::PasswordEncoderTrait;

pub trait PasswordHashResolverTrait {
    fn create<'a>(
        password: &'a str
    ) -> Result<String, ErrorAuditor> {
        match PasswordEncoder::encode(password) {
            Ok(password_hash) => {
                return Ok(password_hash);
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    fn is_valid<'a>(
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match PasswordEncoder::is_valid(password, password_hash) {
            Ok(is_valid) => {
                return Ok(is_valid);
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}