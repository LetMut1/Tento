use crate::domain_layer::entity::proxed_type::date_time::DateTime;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::date_time_expiration_resolver::DateTimeExpirationResolver;
use std::clone::Clone;

#[derive(Clone)]
pub struct Exp {
    value: DateTime
}

impl Exp {
    pub fn new() -> Result<Self, BaseError> {
        return Ok(
            Self {
                value: DateTimeExpirationResolver::create_json_access_web_token_first()?
            }
        );
    }

    pub fn new_from_str<'outer_a>(date_time: &'outer_a str) -> Result<Self, BaseError> {
        return Ok(
            Self {
                value: DateTime::new_from_str(date_time)?
            }
        );
    }

    pub fn to_string<'this>(&'this self) -> String {
        return self.value.to_string();
    }
    
    pub fn get_value<'this>(&'this self) -> &'this DateTime {
        return &self.value;
    }
}