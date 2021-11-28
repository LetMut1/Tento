use argon2::Config;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_encoder_trait::PasswordEncoderTrait;
use uuid::Uuid;

pub struct PasswordEncoder;

impl PasswordEncoderTrait for PasswordEncoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    type Error = BaseError;

    fn encode<'a>(
        password: &'a str
    ) -> Result<String, Self::Error> {
        let config: Config = Config::default(); 

        return Ok(argon2::hash_encoded(password.as_bytes(), Uuid::new_v4().as_bytes(), &config)?);
    }

    fn is_valid<'a>(
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, Self::Error> {
        return Ok(argon2::verify_encoded(password_hash, password.as_bytes())?);
    }
}