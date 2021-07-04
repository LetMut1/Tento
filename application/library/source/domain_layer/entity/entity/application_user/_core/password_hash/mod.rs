use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::utility::_in_context_for::entity::entity::application_user::_core::password::_new_for_context::encoder::Encoder;
use super::password::Password;

pub struct PasswordHash {
    value: String
}

impl PasswordHash {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn new_from_password(value: Password) -> Result<Self, BaseError> {
        return Ok(
            Self {
                value: Encoder::encode(&value)?
            }
        );
    }

    pub fn is_valid_for<'this, 'outer_a>(&'this self, password: &'outer_a Password) -> Result<bool, BaseError> {
        return Ok(Encoder::is_valid(password, self)?);
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}