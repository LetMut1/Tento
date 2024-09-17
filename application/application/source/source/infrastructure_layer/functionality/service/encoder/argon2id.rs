use super::Encoder;
use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
    OptionConverter,
    ResultConverter,
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Algorithm,
    Argon2,
    Params,
    Version,
};
use std::sync::OnceLock;
static ARGON2: OnceLock<Argon2<'static>> = OnceLock::new();
pub struct Argon2Id;
impl Encoder<Argon2Id> {
    pub fn encode<'a>(data: &'a [u8]) -> Result<String, AggregateError> {
        return Ok(
            Self::get()?
                .hash_password(
                    data,
                    &SaltString::generate(OsRng),
                )
                .into_indefinite_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?
                .to_string(),
        );
    }
    pub fn is_valid<'a>(data: &'a [u8], encoded_data: &'a str) -> Result<bool, AggregateError> {
        return Ok(
            Self::get()?
                .verify_password(
                    data,
                    &PasswordHash::new(encoded_data).into_indefinite_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                )
                .is_ok(),
        );
    }
    fn get() -> Result<&'static Argon2<'static>, AggregateError> {
        return match ARGON2.get() {
            Some(argon2) => Ok(argon2),
            None => {
                if let Err(_) = ARGON2.set(
                    Argon2::new(
                        Algorithm::Argon2id,
                        Version::V0x13,
                        Params::default(),
                    ),
                ) {
                    return Err(
                        AggregateError::new_logic_(
                            Common::ValueAlreadyExist,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                ARGON2.get().into_logic_value_does_not_exist(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )
            }
        };
    }
}
