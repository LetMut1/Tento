use super::validator::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::regex::Regex;

impl Validator<ApplicationUser_Email> {
    pub fn is_valid<'a>(application_user_email: &'a ApplicationUser_Email) -> Result<bool, ErrorAuditor> {
        let application_user_email_ = application_user_email.0.as_str();

        let regex = match Regex::new(ApplicationUser_Email::REGULAR_EXPRESSION) {
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

        return Ok(
            regex.is_match(application_user_email_)
                && application_user_email_.chars().count() <= ApplicationUser_Email::MAXIMUM_LENGTH
        );
    }
}
