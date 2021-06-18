use crate::error::base_error::base_error::BaseError;
use crate::utility::_in_context_for::entity::entity::application_user::_core::password::_new_for_context::password_encoder::PasswordEncoder;
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
                value: PasswordEncoder::encode(&value)?
            }
        );
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}