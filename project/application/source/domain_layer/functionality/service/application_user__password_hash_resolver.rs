use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use extern_crate::argon2;
use extern_crate::argon2::Config;
use extern_crate::uuid::Uuid;

#[allow(non_camel_case_types)]
pub struct ApplicationUser_PasswordHashResolver;

impl ApplicationUser_PasswordHashResolver {
    pub fn create<'a>(application_user_password: &'a str) -> Result<String, ErrorAuditor> {
        let password_hash = match ApplicationUser_PasswordEncoder::encode(application_user_password) {
            Ok(password_hash_) => password_hash_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(password_hash);
    }

    pub fn is_valid<'a>(application_user_password: &'a str, application_user_password_hash: &'a str) -> Result<bool, ErrorAuditor> {
        let is_valid = match ApplicationUser_PasswordEncoder::is_valid(application_user_password, application_user_password_hash) {
            Ok(is_valid_) => is_valid_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(is_valid);
    }
}

#[allow(non_camel_case_types)]
struct ApplicationUser_PasswordEncoder;

impl ApplicationUser_PasswordEncoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    fn encode<'a>(application_user_password: &'a str) -> Result<String, ErrorAuditor> {                          // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.
        let config = Config::default();   // TODO настроить конфиг

        let value = match argon2::hash_encoded(
            application_user_password.as_bytes(),
            Uuid::new_v4().as_bytes().as_slice(),
            &config
        ) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(value);
    }

    fn is_valid<'a>(application_user_password: &'a str, application_user_password_hash: &'a str) -> Result<bool, ErrorAuditor> {
        let value = match argon2::verify_encoded(application_user_password_hash, application_user_password.as_bytes()) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(value);
    }
}