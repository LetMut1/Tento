use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use extern_crate::argon2;
use extern_crate::argon2::Config;
use extern_crate::uuid::Uuid;

pub struct ApplicationUserPasswordHashResolver;

impl ApplicationUserPasswordHashResolver {
    pub fn create<'a>(
        application_user_password: &'a str
    ) -> Result<String, ErrorAuditor> {
        match ApplicationUserPasswordEncoder::encode(application_user_password) {
            Ok(password_hash) => {
                return Ok(password_hash);
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    pub fn is_valid<'a>(
        application_user_password: &'a str,
        application_user_password_hash: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match ApplicationUserPasswordEncoder::is_valid(application_user_password, application_user_password_hash) {
            Ok(is_valid) => {
                return Ok(is_valid);
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}

struct ApplicationUserPasswordEncoder;

impl ApplicationUserPasswordEncoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    fn encode<'a>(                            // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.
        application_user_password: &'a str
    ) -> Result<String, ErrorAuditor> {
        let config = Config::default();

        match argon2::hash_encoded(application_user_password.as_bytes(), Uuid::new_v4().as_bytes().as_slice(), &config) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    fn is_valid<'a>(
        application_user_password: &'a str,
        application_user_password_hash: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match argon2::verify_encoded(application_user_password_hash, application_user_password.as_bytes()) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}