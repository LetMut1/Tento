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

    pub fn is_production() -> bool {
        if env::var(Self::IS_PRODUCTION_KEY).unwrap() == Self::IS_PRODUCTION_VALUE_TRUE {
            return true;
        }

        return false;
    }

    pub fn get_server_socket_address() -> String {
        return env::var(Self::SERVER_SOCKET_ADDRESS_KEY).unwrap();
    }

    pub fn get_logger_roller_log_file_name() -> String {
        return env::var(Self::LOGGER_ROLLER_LOG_FILE_NAME_KEY).unwrap();
    }

    pub fn get_logger_log_file_name() -> String {
        return env::var(Self::LOGGER_LOG_FILE_NAME_KEY).unwrap();
    }

    pub fn get_logger_encoder_pattern() -> String {
        return env::var(Self::LOGGER_ENCODER_PATTERN_KEY).unwrap();
    }

    pub fn get_security_jrwt_encoding_private_key() -> String {
        return env::var(Self::SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY).unwrap();
    }

    pub fn get_security_jawt_signature_encoding_private_key() -> String {
        return env::var(Self::SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY).unwrap();
    }

    pub fn get_resource_postgresql_url() -> String {
        return env::var(Self::RESOURCE_POSTGRESQL_URL_KEY).unwrap();
    }

    pub fn get_resource_redis_url() -> String {
        return env::var(Self::RESOURCE_REDIS_URL_KEY).unwrap();
    }

    pub fn get_resource_email_server_socket_address() -> String {
        return env::var(Self::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY).unwrap();
    }
}