use super::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Auditor;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use regex::Regex;

impl Validator<ApplicationUser_Email> {
    pub fn is_valid<'a>(application_user_email: &'a ApplicationUser_Email) -> Result<bool, Auditor<Error>> {
        let application_user_email_ = application_user_email.0.as_str();

        todo!("Объект в статическую переменную");

        let regex = match Regex::new(ApplicationUser_Email::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(regex.is_match(application_user_email_) && application_user_email_.chars().count() <= ApplicationUser_Email::MAXIMUM_LENGTH);
    }
}
