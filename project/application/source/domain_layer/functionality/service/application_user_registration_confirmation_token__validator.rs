use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use extern_crate::regex::Regex;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationConfirmationToken_Validator;

impl ApplicationUserRegistrationConfirmationToken_Validator {
    pub fn is_valid_value<'a>(application_user_registration_confirmation_token_value: &'a str) -> Result<bool, ErrorAuditor> {
        match Regex::new(r"^[0-9]{6}$") {
            Ok(regex) => {
                return Ok(regex.is_match(application_user_registration_confirmation_token_value));
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