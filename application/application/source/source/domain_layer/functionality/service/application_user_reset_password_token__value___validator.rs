use super::validator::Validator;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::regex::Regex;

impl Validator<ApplicationUserResetPasswordToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token_value: &'a ApplicationUserResetPasswordToken_Value) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(ApplicationUserResetPasswordToken_Value::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(regex.is_match(application_user_authorization_token_value.0.as_str()));
    }
}
