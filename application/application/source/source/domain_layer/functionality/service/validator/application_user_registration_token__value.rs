use super::Validator;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use regex::Regex;

impl Validator<ApplicationUserRegistrationToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token_value: &'a ApplicationUserRegistrationToken_Value) -> Result<bool, Auditor<Error>> {
        let regex = Regex::new(ApplicationUserRegistrationToken_Value::REGULAR_EXPRESSION).convert(BacktracePart::new(line!(), file!()))?;

        return Ok(regex.is_match(application_user_authorization_token_value.0.as_str()));
    }
}
