use super::password_encoder_trait::PasswordEncoderTrait;
use std::error::Error;

pub trait PasswordHashResolverTrait {
    type Error: Error;
    type Encoder: PasswordEncoderTrait<Error = Self::Error>;

    fn create<'a>(
        password: &'a str
    ) -> Result<String, Self::Error> {
        return Ok(<Self::Encoder as PasswordEncoderTrait>::encode(password)?);
    }

    fn is_valid<'a>(
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, Self::Error> {
        return Ok(<Self::Encoder as PasswordEncoderTrait>::is_valid(password, password_hash)?);
    }
}