use crate::domain_layer::entity::entity::application_user::_core::password_hash::PasswordHash;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub trait EncoderTrait {
    fn encode<'outer_a>(password: &'outer_a Password) -> Result<String, BaseError>;

    fn is_valid<'outer_a>(password: &'outer_a Password, password_hash: &'outer_a PasswordHash) -> Result<bool, BaseError>;
}