use super::Validator;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use regex::Regex;

impl Validator<ApplicationUserRegistrationToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token_value: &'a ApplicationUserRegistrationToken_Value) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(ApplicationUserRegistrationToken_Value::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
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
