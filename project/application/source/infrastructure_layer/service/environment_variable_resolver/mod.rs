use std::net::SocketAddr;

pub struct EnvironmentVariableResolver {
    is_production_environment: bool,
    server_socket_address: SocketAddr,
    logger_roller_log_file_name: String,
    logger_log_file_name: String,
    logger_encoder_pattern: String,
    security_jrwt_encoding_private_key: String,
    security_jawt_signature_encoding_private_key: String,
    resource_postgresql_url: String,
    resource_redis_url: String,
    resource_email_server_socket_address: SocketAddr
}

impl EnvironmentVariableResolver {
    pub const SERVER_SOCKET_ADDRESS_KEY: &'static str = "SERVER_SOCKET_ADDRESS";
    pub const LOGGER_ROLLER_LOG_FILE_NAME_KEY: &'static str = "LOGGER_ROLLER_LOG_FILE_NAME";
    pub const LOGGER_LOG_FILE_NAME_KEY: &'static str = "LOGGER_LOG_FILE_NAME";
    pub const LOGGER_ENCODER_PATTERN_KEY: &'static str = "LOGGER_ENCODER_PATTERN";
    pub const SECURITY_JRWT_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_JRWT_ENCODING_PRIVATE_KEY";
    pub const SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY";
    pub const RESOURCE_POSTGRESQL_URL_KEY: &'static str = "RESOURCE_POSTGRESQL_URL";
    pub const RESOURCE_REDIS_URL_KEY: &'static str = "RESOURCE_REDIS_URL";
    pub const RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY: &'static str = "RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS";

    pub fn new(
        is_production_environment: bool,
        server_socket_address: SocketAddr,
        logger_roller_log_file_name: String,
        logger_log_file_name: String,
        logger_encoder_pattern: String,
        security_jrwt_encoding_private_key: String,
        security_jawt_signature_encoding_private_key: String,
        resource_postgresql_url: String,
        resource_redis_url: String,
        resource_email_server_socket_address: SocketAddr
    ) -> Self {
        return Self {
            is_production_environment,
            server_socket_address,
            logger_roller_log_file_name,
            logger_log_file_name,
            logger_encoder_pattern,
            security_jrwt_encoding_private_key,
            security_jawt_signature_encoding_private_key,
            resource_postgresql_url,
            resource_redis_url,
            resource_email_server_socket_address
        };
    }

    pub fn get_is_production_environment<'a>(&'a self) -> bool {
        return self.is_production_environment;
    }

    pub fn get_server_socket_address<'a>(&'a self) -> &'a SocketAddr {
        return &self.server_socket_address;
    }

    pub fn get_logger_roller_log_file_name<'a>(&'a self) -> &'a str {
        return self.logger_roller_log_file_name.as_str();
    }

    pub fn get_logger_log_file_name<'a>(&'a self) -> &'a str {
        return self.logger_log_file_name.as_str();
    }

    pub fn get_logger_encoder_pattern<'a>(&'a self) -> &'a str {
        return self.logger_encoder_pattern.as_str();
    }

    pub fn get_security_jrwt_encoding_private_key<'a>(&'a self) -> &'a str {
        return self.security_jrwt_encoding_private_key.as_str();
    }

    pub fn get_security_jawt_signature_encoding_private_key<'a>(&'a self) -> &'a str {
        return self.security_jawt_signature_encoding_private_key.as_str();
    }

    pub fn get_resource_postgresql_url<'a>(&'a self) -> &'a str {
        return self.resource_postgresql_url.as_str();
    }

    pub fn get_resource_redis_url<'a>(&'a self) -> &'a str {
        return self.resource_redis_url.as_str();
    }

    pub fn get_resource_email_server_socket_address<'a>(&'a self) -> &'a SocketAddr {
        return &self.resource_email_server_socket_address;
    }
}