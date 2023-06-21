use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::config::Config;
use std::clone::Clone;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct EnvironmentConfiguration {
    application_server_socket_address: SocketAddr,
    database_1_postgresql_configuration: Config,
    database_2_postgresql_configuration: Config,
    database_1_redis_connection_info: ConnectionInfo,
    pushable_environment_configuration: PushableEnvironmentConfiguration
}

impl EnvironmentConfiguration {
    pub fn new(
        application_server_socket_address: SocketAddr,
        database_1_postgresql_configuration: Config,
        database_2_postgresql_configuration: Config,
        database_1_redis_connection_info: ConnectionInfo,
        environment: Environment,
        application_user_access_refresh_token_private_key: String,
        application_user_access_token_private_key: String,
        email_server_socket_address: SocketAddr,
    ) -> Self {
        return Self {
            application_server_socket_address,
            database_1_postgresql_configuration,
            database_2_postgresql_configuration,
            database_1_redis_connection_info,
            pushable_environment_configuration: PushableEnvironmentConfiguration {
                environment,
                application_user_access_refresh_token_private_key,
                application_user_access_token_private_key,
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

    pub fn get_database_1_redis_connection_info<'a>(&'a self) -> &'a ConnectionInfo {
        return &self.database_1_redis_connection_info;
    }

    pub fn get_pushable_environment_configuration<'a>(&'a self) -> &'a PushableEnvironmentConfiguration {
        return &self.pushable_environment_configuration;
    }
}

#[derive(Clone)]
pub struct PushableEnvironmentConfiguration {
    environment: Environment,
    application_user_access_refresh_token_private_key: String,
    application_user_access_token_private_key: String,
    email_server_socket_address: SocketAddr
}

impl PushableEnvironmentConfiguration {
    pub fn get_environment<'a>(&'a self) -> &'a Environment {
        return &self.environment;
    }

    pub fn get_application_user_access_refresh_token_private_key<'a>(&'a self) -> &'a str {
        return self.application_user_access_refresh_token_private_key.as_str();
    }

    pub fn get_application_user_access_token_private_key<'a>(&'a self) -> &'a str {
        return self.application_user_access_token_private_key.as_str();
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