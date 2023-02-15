use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::regex::Regex;

pub struct ApplicationUserResetPasswordToken_Validator;

impl ApplicationUserResetPasswordToken_Validator {
    pub fn is_valid_value<'a>(application_user_reset_password_token_value: &'a str) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(r"^[0-9]{6}$") {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(regex.is_match(application_user_reset_password_token_value));
    }
}