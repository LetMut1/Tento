use argon2::Config;
use crate::domain_layer::entity::entity::application_user::_core::password_hash::PasswordHash;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_core::password::_new_for_context::encoder_trait::EncoderTrait;
use uuid::Uuid;

pub struct Encoder;

impl EncoderTrait for Encoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    fn encode<'outer_a>(password: &'outer_a Password) -> Result<String, BaseError> {
        let config: Config = Config::default(); 

        return Ok(argon2::hash_encoded(password.get_value().as_bytes(), Uuid::new_v4().as_bytes(), &config)?);
    }

    fn is_valid<'outer_a>(password: &'outer_a Password, password_hash: &'outer_a PasswordHash) -> Result<bool, BaseError> {
        return Ok(argon2::verify_encoded(password_hash.get_value(), password.get_value().as_bytes())?);
    }
}