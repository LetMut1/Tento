use super::Encoder;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use argon2::Config;
use uuid::Uuid;

pub use crate::infrastructure_layer::data::control_type::Argon2Id;

impl Encoder<Argon2Id> {
    pub fn encode<'a>(data: &'a [u8]) -> Result<String, Auditor<Error>> { // // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.  // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090
        let config = Config::default(); // TODO настроить конфиг, возможно, вынестки в константу

        let salt = Uuid::new_v4();

        let value = argon2::hash_encoded(
            data,
            salt.as_bytes().as_slice(),
            &config,
        )
        .convert(BacktracePart::new(line!(), file!()))?;

        return Ok(value);
    }

    pub fn is_valid<'a>(
        data: &'a [u8],
        encoded_data: &'a str,
    ) -> Result<bool, Auditor<Error>> {
        let value = argon2::verify_encoded(
            encoded_data,
            data,
        )
        .convert(BacktracePart::new(line!(), file!()))?;

        return Ok(value);
    }
}
