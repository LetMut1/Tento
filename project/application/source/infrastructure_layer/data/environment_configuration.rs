use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::config::Config;
use std::clone::Clone;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct EnvironmentConfiguration {
    application_server_socket_address: SocketAddr,
    database_1_postgresql_configuration: Config,
    database_2_postgresql_configuration: Config,
    redis_connection_info: ConnectionInfo,
    pushable_environment_configuration: PushableEnvironmentConfiguration
}

impl EnvironmentConfiguration {
    pub const APPLICATION_SERVER_SOCKET_ADDRESS_KEY: &'static str = "APPLICATION_SERVER_SOCKET_ADDRESS";
    pub const DATABASE_1_POSTGRESQL_URL_KEY: &'static str = "DATABASE_1_POSTGRESQL_URL";
    pub const DATABASE_2_POSTGRESQL_URL_KEY: &'static str = "DATABASE_2_POSTGRESQL_URL";
    pub const REDIS_URL_KEY: &'static str = "REDIS_URL";
    pub const SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY: &'static str = PushableEnvironmentConfiguration::SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY;
    pub const SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY: &'static str = PushableEnvironmentConfiguration::SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY;
    pub const EMAIL_SERVER_SOCKET_ADDRESS_KEY: &'static str = PushableEnvironmentConfiguration::EMAIL_SERVER_SOCKET_ADDRESS_KEY;

    pub fn new(
        application_server_socket_address: SocketAddr,
        database_1_postgresql_configuration: Config,
        database_2_postgresql_configuration: Config,
        redis_connection_info: ConnectionInfo,
        environment: Environment,
        security_auart_encoding_private_key: String,
        security_auat_signature_encoding_private_key: String,
        email_server_socket_address: SocketAddr,
    ) -> Self {
        return Self {
            application_server_socket_address,
            database_1_postgresql_configuration,
            database_2_postgresql_configuration,
            redis_connection_info,
            pushable_environment_configuration: PushableEnvironmentConfiguration {
                environment,
                security_auart_encoding_private_key,
                security_auat_signature_encoding_private_key,
                email_server_socket_address
            }
        };
    }

    pub fn get_application_server_socket_address<'a>(&'a self) -> &'a SocketAddr {
        return &self.application_server_socket_address;
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

    pub fn get_pushable_environment_configuration<'a>(&'a self) -> &'a PushableEnvironmentConfiguration {
        return &self.pushable_environment_configuration;
    }
}

#[derive(Clone)]
pub struct PushableEnvironmentConfiguration {
    environment: Environment,
    security_auart_encoding_private_key: String,
    security_auat_signature_encoding_private_key: String,
    email_server_socket_address: SocketAddr
}

impl PushableEnvironmentConfiguration {
    const SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_AUART_ENCODING_PRIVATE_KEY";
    const SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY: &'static str = "SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY";
    const EMAIL_SERVER_SOCKET_ADDRESS_KEY: &'static str = "EMAIL_SERVER_SOCKET_ADDRESS";

    pub fn get_environment<'a>(&'a self) -> &'a Environment {
        return &self.environment;
    }

    pub fn get_security_auart_encoding_private_key<'a>(&'a self) -> &'a str {
        return self.security_auart_encoding_private_key.as_str();
    }

    pub fn get_security_auat_signature_encoding_private_key<'a>(&'a self) -> &'a str {
        return self.security_auat_signature_encoding_private_key.as_str();
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