use crate::error::base_error::base_error::BaseError;
use crate::utility::_in_context_for::entity::entity::application_user::_core::email::_new_for_context::validator::Validator;
use std::clone::Clone;

#[derive(Clone)]
pub struct Email {
    value: String
}

impl Email {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn is_valid<'this>(&'this self) -> Result<bool, BaseError> {
        return Ok(Validator::is_valid(self)?);
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}