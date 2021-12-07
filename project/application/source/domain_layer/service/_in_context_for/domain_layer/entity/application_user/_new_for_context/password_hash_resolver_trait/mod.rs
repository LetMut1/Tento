use super::password_encoder_trait::PasswordEncoderTrait;

pub trait PasswordHashResolverTrait {
    type PasswordEncoder: PasswordEncoderTrait;

    fn create<'a>(
        password: &'a str
    ) -> Result<String, <Self::PasswordEncoder as PasswordEncoderTrait>::Error> {
        return Ok(<Self::PasswordEncoder as PasswordEncoderTrait>::encode(password)?);
    }

    fn is_valid<'a>(
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, <Self::PasswordEncoder as PasswordEncoderTrait>::Error> {
        return Ok(<Self::PasswordEncoder as PasswordEncoderTrait>::is_valid(password, password_hash)?);
    }
}