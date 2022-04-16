use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use std::env;

pub struct EnvironmentVariableResolver;

impl EnvironmentVariableResolver {
    pub const IS_PRODUCTION_KEY: &'static str = "IS_PRODUCTION";
    pub const IS_PRODUCTION_VALUE_TRUE: &'static str = "t";
    pub const IS_PRODUCTION_VALUE_FALSE: &'static str = "f";
    pub const SERVER_SOCKET_ADDRESS_KEY: &'static str = "SERVER_SOCKET_ADDRESS";
    pub const LOGGER_ROLLER_LOG_FILE_NAME_KEY: &'static str = "LOGGER_ROLLER_LOG_FILE_NAME";
    pub const LOGGER_LOG_FILE_NAME_KEY: &'static str = "LOGGER_LOG_FILE_NAME";
    pub const LOGGER_ENCODER_PATTERN_KEY: &'static str = "LOGGER_ENCODER_PATTERN";
    pub const SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_JRWT_ENCODING_PRIVATE_KEY";
    pub const SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY";
    pub const RESOURCE_POSTGRESQL_URL_KEY: &'static str = "RESOURCE_POSTGRESQL_URL";
    pub const RESOURCE_REDIS_URL_KEY: &'static str = "RESOURCE_REDIS_URL";
    pub const RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY: &'static str = "RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS";

    pub fn is_production(
    ) -> Result<bool, ErrorAuditor> {
        match env::var(Self::IS_PRODUCTION_KEY) {
            Ok(value) => {
                if value.as_str() == Self::IS_PRODUCTION_VALUE_TRUE {
                    return Ok(true);
                }
        
                return Ok(false);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_server_socket_address(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::SERVER_SOCKET_ADDRESS_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_logger_roller_log_file_name(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::LOGGER_ROLLER_LOG_FILE_NAME_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_logger_log_file_name(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::LOGGER_LOG_FILE_NAME_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_logger_encoder_pattern(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::LOGGER_ENCODER_PATTERN_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_security_jrwt_encoding_private_key(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_security_jawt_signature_encoding_private_key(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_resource_postgresql_url(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::RESOURCE_POSTGRESQL_URL_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_resource_redis_url(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::RESOURCE_REDIS_URL_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn get_resource_email_server_socket_address(
    ) -> Result<String, ErrorAuditor> {
        match env::var(Self::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}