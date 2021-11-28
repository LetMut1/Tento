use crate::infrastructure_layer::error::base_error::base_error::BaseError;
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
    ) -> Result<bool, BaseError> {
        if env::var(Self::IS_PRODUCTION_KEY)? == Self::IS_PRODUCTION_VALUE_TRUE {
            return Ok(true);
        }

        return Ok(false);
    }

    pub fn get_server_socket_address(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::SERVER_SOCKET_ADDRESS_KEY)?);
    }

    pub fn get_logger_roller_log_file_name(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::LOGGER_ROLLER_LOG_FILE_NAME_KEY)?);
    }

    pub fn get_logger_log_file_name(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::LOGGER_LOG_FILE_NAME_KEY)?);
    }

    pub fn get_logger_encoder_pattern(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::LOGGER_ENCODER_PATTERN_KEY)?);
    }

    pub fn get_security_jrwt_encoding_private_key(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY)?);
    }

    pub fn get_security_jawt_signature_encoding_private_key(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY)?);
    }

    pub fn get_resource_postgresql_url(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::RESOURCE_POSTGRESQL_URL_KEY)?);
    }

    pub fn get_resource_redis_url(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::RESOURCE_REDIS_URL_KEY)?);
    }

    pub fn get_resource_email_server_socket_address(
    ) -> Result<String, BaseError> {
        return Ok(env::var(Self::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY)?);
    }
}