use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::config::Config;
use std::clone::Clone;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct EnvironmentConfiguration {
    environment: Environment,
    application_server_socket_address: SocketAddr,
    security_auart_encoding_private_key: String,
    security_auat_signature_encoding_private_key: String,
    database_1_postgresql_configuration: Config,
    database_2_postgresql_configuration: Config,
    redis_connection_info: ConnectionInfo,
    email_server_socket_address: SocketAddr
}

impl EnvironmentConfiguration {
    pub const APPLICATION_SERVER_SOCKET_ADDRESS_KEY: &'static str = "APPLICATION_SERVER_SOCKET_ADDRESS";
    pub const SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_AUART_ENCODING_PRIVATE_KEY";
    pub const SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY";
    pub const DATABASE_1_POSTGRESQL_URL_KEY: &'static str = "DATABASE_1_POSTGRESQL_URL";
    pub const DATABASE_2_POSTGRESQL_URL_KEY: &'static str = "DATABASE_2_POSTGRESQL_URL";
    pub const REDIS_URL_KEY: &'static str = "REDIS_URL";
    pub const EMAIL_SERVER_SOCKET_ADDRESS_KEY: &'static str = "EMAIL_SERVER_SOCKET_ADDRESS";

    pub fn new(
        environment: Environment,
        application_server_socket_address: SocketAddr,
        security_auart_encoding_private_key: String,
        security_auat_signature_encoding_private_key: String,
        database_1_postgresql_configuration: Config,
        database_2_postgresql_configuration: Config,
        redis_connection_info: ConnectionInfo,
        email_server_socket_address: SocketAddr
    ) -> Self {
        return Self {
            environment,
            application_server_socket_address,
            security_auart_encoding_private_key,
            security_auat_signature_encoding_private_key,
            database_1_postgresql_configuration,
            database_2_postgresql_configuration,
            redis_connection_info,
            email_server_socket_address
        };
    }

    pub fn get_environment<'a>(&'a self) -> &'a Environment {
        return &self.environment;
    }

    pub fn get_application_server_socket_address<'a>(&'a self) -> &'a SocketAddr {
        return &self.application_server_socket_address;
    }

    pub fn get_security_auart_encoding_private_key<'a>(&'a self) -> &'a str {
        return self.security_auart_encoding_private_key.as_str();
    }

    pub fn get_security_auat_signature_encoding_private_key<'a>(&'a self) -> &'a str {
        return self.security_auat_signature_encoding_private_key.as_str();
    }

    pub fn get_database_1_postgresql_configuration<'a>(&'a self) -> &'a Config {
        return &self.database_1_postgresql_configuration;
    }

    pub fn get_database_2_postgresql_configuration<'a>(&'a self) -> &'a Config {
        return &self.database_2_postgresql_configuration;
    }

    pub fn get_redis_url<'a>(&'a self) -> &'a ConnectionInfo {
        return &self.redis_connection_info;
    }

    pub fn get_email_server_socket_address<'a>(&'a self) -> &'a SocketAddr {
        return &self.email_server_socket_address;
    }
}

#[derive(Clone)]
pub enum Environment {
    Production,
    Development,
    LocalDevelopment
}