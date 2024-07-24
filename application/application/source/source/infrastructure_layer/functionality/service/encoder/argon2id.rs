use super::Encoder;
use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    },
    control_type::Argon2Id,
};
use argon2::Config;
use uuid::Uuid;
impl Encoder<Argon2Id> {
    pub fn encode<'a>(data: &'a [u8]) -> Result<String, AggregateError> {
        // // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.  // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090
        let config = Config::default(); // TODO настроить конфиг, возможно, вынестки в константу
        let salt = Uuid::new_v4();
        return argon2::hash_encoded(
            data,
            salt.as_bytes().as_slice(),
            &config,
        )
        .into_invalid_argument_from_outside_and_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
    pub fn is_valid<'a>(data: &'a [u8], encoded_data: &'a str) -> Result<bool, AggregateError> {
        return argon2::verify_encoded(
            encoded_data,
            data,
        )
        .into_invalid_argument_from_outside_and_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
