use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::argon2;
use extern_crate::argon2::Config;
use extern_crate::uuid::Uuid;
use std::marker::PhantomData;

pub struct Encoder<S> {
    _subject: PhantomData<S>
}

pub struct Argon2Id;

impl Encoder<Argon2Id> {       // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090
    pub fn encode<'a>(data: &'a [u8]) -> Result<String, ErrorAuditor> {    // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.
        let config = Config::default();   // TODO настроить конфиг

        let salt = Uuid::new_v4();

        let value = match argon2::hash_encoded(
            data,
            salt.as_bytes().as_slice(),
            &config
        ) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(value);
    }

    pub fn is_valid<'a>(data: &'a [u8], encoded_data: &'a str) -> Result<bool, ErrorAuditor> {
        let value = match argon2::verify_encoded(encoded_data, data) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(value);
    }
}