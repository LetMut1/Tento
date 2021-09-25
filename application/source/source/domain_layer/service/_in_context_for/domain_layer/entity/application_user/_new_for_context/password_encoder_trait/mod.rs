use std::error::Error;

pub trait PasswordEncoderTrait {
    type Error: Error;

    fn encode<'a>(
        password: &'a str
    ) -> Result<String, Self::Error>;

    fn is_valid<'a>(
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, Self::Error>;
}