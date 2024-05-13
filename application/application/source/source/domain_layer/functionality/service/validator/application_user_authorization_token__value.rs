use super::Validator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use regex::Regex;

impl Validator<ApplicationUserAuthorizationToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value) -> Result<bool, Auditor<Error>> {
        let regex = Regex::new(ApplicationUserAuthorizationToken_Value::REGULAR_EXPRESSION).convert(Backtrace::new(line!(), file!()))?;

        return Ok(regex.is_match(application_user_authorization_token_value.0.as_str()));
    }
}
