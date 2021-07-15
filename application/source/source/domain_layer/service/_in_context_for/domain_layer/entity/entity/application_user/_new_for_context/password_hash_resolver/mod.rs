use crate::domain_layer::entity::entity::application_user::_core::password_hash::PasswordHash;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_core::password::_new_for_context::encoder_trait::EncoderTrait;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_core::password::_new_for_context::encoder::Encoder;

pub struct PasswordHashResolver;

impl PasswordHashResolver {
    pub fn create<'outer_a>(password: &'outer_a Password) -> Result<PasswordHash, BaseError> {
        return Ok(PasswordHash::new(<Encoder as EncoderTrait>::encode(password)?));
    }

    pub fn is_valid<'outer_a>(password: &'outer_a Password, password_hash: &'outer_a PasswordHash) -> Result<bool, BaseError> {
        return Ok(<Encoder as EncoderTrait>::is_valid(password, password_hash)?);
    }
}