use super::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use regex::Regex;

impl Validator<ApplicationUser_Email> {
    pub fn is_valid<'a>(application_user_email: &'a ApplicationUser_Email) -> Result<bool, Auditor<Error>> {
        let application_user_email_ = application_user_email.0.as_str();

        todo!("Объект в статическую переменную и везде, где есть regex");

        let regex = Regex::new(ApplicationUser_Email::REGULAR_EXPRESSION).convert(Backtrace::new(line!(), file!()))?;

        return Ok(regex.is_match(application_user_email_) && application_user_email_.chars().count() <= ApplicationUser_Email::MAXIMUM_LENGTH);
    }
}
