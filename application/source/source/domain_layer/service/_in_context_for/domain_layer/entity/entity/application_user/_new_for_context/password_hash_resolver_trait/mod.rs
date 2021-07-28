use crate::domain_layer::entity::entity::application_user::_component::password_hash::PasswordHash;
use crate::domain_layer::entity::entity::application_user::_component::password::Password;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_component::password::_new_for_context::encoder_trait::EncoderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub trait PasswordHashResolverTrait {
    type Encoder: EncoderTrait;

    fn create<'outer_a>(password: &'outer_a Password) -> Result<PasswordHash, BaseError> {
        return Ok(PasswordHash::new(Self::Encoder::encode(password)?));
    }

    fn is_valid<'outer_a>(password: &'outer_a Password, password_hash: &'outer_a PasswordHash) -> Result<bool, BaseError> {
        return Ok(Self::Encoder::is_valid(password, password_hash)?);
    }
}