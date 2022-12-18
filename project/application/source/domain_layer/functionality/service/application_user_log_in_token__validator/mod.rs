use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use extern_crate::regex::Regex;

#[allow(non_camel_case_types)]
pub struct ApplicationUserogInToken_Validator;

impl ApplicationUserogInToken_Validator {
    pub fn is_valid_value<'a>(
        application_user_log_in_token_value: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match Regex::new(r"^[0-9]{6}$") {
            Ok(regex) => {
                return Ok(regex.is_match(application_user_log_in_token_value));
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}