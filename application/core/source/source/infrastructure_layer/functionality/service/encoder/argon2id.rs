use {
    argon2::{
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
    },
    crate::infrastructure_layer::data::aggregate_error::AggregateError,
    super::Encoder,
    std::sync::OnceLock,
};
static ARGON2: OnceLock<Argon2<'static>> = OnceLock::new();
pub struct Argon2Id;
impl Encoder<Argon2Id> {
    pub fn encode<'a>(data_for_encode: &'a [u8]) -> Result<String, AggregateError> {
        return Result::Ok(
            crate::result_return_indefinite_argument!(
                Self::get()?
                .hash_password(
                    data_for_encode,
                    &SaltString::generate(OsRng),
                )
            )
            .to_string(),
        );
    }
    pub fn is_valid<'a>(data_for_encode: &'a [u8], encoded_data: &'a str) -> Result<bool, AggregateError> {
        return Result::Ok(
            Self::get()?
                .verify_password(
                    data_for_encode,
                    &crate::result_return_indefinite_argument!(PasswordHash::new(encoded_data)),
                )
                .is_ok(),
        );
    }
    fn get() -> Result<&'static Argon2<'static>, AggregateError> {
        return match ARGON2.get() {
            Option::Some(argon2) => Result::Ok(argon2),
            Option::None => {
                if ARGON2
                    .set(
                        Argon2::new(
                            Algorithm::Argon2id,
                            Version::V0x13,
                            Params::default(),
                        ),
                    )
                    .is_err()
                {
                    return Result::Err(crate::new_logic_value_already_exist!());
                }
                crate::option_into_logic_value_does_not_exist!(ARGON2.get())
            }
        };
    }
}
