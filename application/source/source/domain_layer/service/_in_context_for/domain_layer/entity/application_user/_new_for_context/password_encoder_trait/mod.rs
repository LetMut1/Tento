use std::error::Error;

pub trait PasswordEncoderTrait {
    type Error: Error;

    fn encode<'outer_a>(
        password: &'outer_a str
    ) -> Result<String, Self::Error>;

    fn is_valid<'outer_a>(
        password: &'outer_a str,
        password_hash: &'outer_a str
    ) -> Result<bool, Self::Error>;
}