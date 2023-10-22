use super::Encoder;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use argon2::hash_encoded;
use argon2::verify_encoded;
use argon2::Config;
use uuid::Uuid;

pub use crate::infrastructure_layer::data::control_type::Argon2Id;

impl Encoder<Argon2Id> {
    pub fn encode<'a>(data: &'a [u8]) -> Result<String, ErrorAuditor> { // // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.  // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090
        let config = Config::default(); // TODO настроить конфиг, возможно, вынестки в константу

        let salt = Uuid::new_v4();

        let value = match hash_encoded(
            data,
            salt.as_bytes().as_slice(),
            &config,
        ) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(value);
    }

    pub fn is_valid<'a>(
        data: &'a [u8],
        encoded_data: &'a str,
    ) -> Result<bool, ErrorAuditor> {
        let value = match verify_encoded(
            encoded_data,
            data,
        ) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(value);
    }
}
