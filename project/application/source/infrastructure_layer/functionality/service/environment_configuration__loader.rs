use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::redis::ConnectionInfo;
use extern_crate::serde::Deserialize;
use extern_crate::tokio_postgres::Config as PostgresqlConfiguration;
use extern_crate::toml::from_str as toml_from_str;
use std::fs::read_to_string;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::str::FromStr;
use super::loader::Loader;

impl Loader<EnvironmentConfiguration> {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "environment.production.toml";
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "environment.development.toml";
    const LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "environment.development.local.toml";

    pub fn load_from_file(configuration_file_path: &'static str) -> Result<EnvironmentConfiguration, ErrorAuditor> {
        let file_path = match Path::new(configuration_file_path).parent() {
            Some(file_path_) => file_path_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "The directory does not exist." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let production_environment_file_path_buffer = file_path.join(
            Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME)
        );

        let (environment, environment_file_data) = if production_environment_file_path_buffer.exists() {
            let environment_file_data_ = match read_to_string(production_environment_file_path_buffer.as_path()) {
                Ok(environment_file_data__) => environment_file_data__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            (Environment::Production, environment_file_data_)
        } else {
            let local_development_environment_file_path_buffer = file_path.join(
                Path::new(Self::LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME)
            );

            if local_development_environment_file_path_buffer.exists() {
                let environment_file_data_ = match read_to_string(local_development_environment_file_path_buffer.as_path()) {
                    Ok(environment_file_data__) => environment_file_data__,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };

                (Environment::LocalDevelopment, environment_file_data_)
            } else {
                let development_environment_file_path_buffer = file_path.join(
                    Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME)
                );

                let environment_file_data_ = if development_environment_file_path_buffer.exists() {
                    let environment_file_data__ = match read_to_string(development_environment_file_path_buffer.as_path()) {
                        Ok(environment_file_data___) => environment_file_data___,
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    };

                    environment_file_data__
                } else {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError { message: "Any ....env files does not exist." },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                };

                (Environment::Development, environment_file_data_)
            }
        };

        let environment_file_configuration = match toml_from_str::<EnvironmentFileConfiguration>(environment_file_data.as_str()) {
            Ok(environment_file_configuration_) => environment_file_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let mut application_server_socket_address_registry = match environment_file_configuration.application.socket_address.to_socket_addrs() {
            Ok(application_server_socket_address_registry_) => application_server_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_server_socket_address_ = match application_server_socket_address_registry.next() {
            Some(application_server_socket_address__) => application_server_socket_address__,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Invalid socket address." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let database_1_postgresql_configuration = match PostgresqlConfiguration::from_str(
            environment_file_configuration.resource.postgresql.database_1_url.as_str()
        ) {
            Ok(database_1_postgresql_configuration_) => database_1_postgresql_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let database_2_postgresql_configuration = match PostgresqlConfiguration::from_str(
            environment_file_configuration.resource.postgresql.database_2_url.as_str()
        ) {
            Ok(database_2_postgresql_configuration_) => database_2_postgresql_configuration_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let database_1_redis_connection_info = match ConnectionInfo::from_str(
            environment_file_configuration.resource.redis.database_1_url.as_str()
        ) {
            Ok(database_1_redis_connection_info_) => database_1_redis_connection_info_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let mut email_server_socket_address_registry = match environment_file_configuration.resource.email_server.socket_address.to_socket_addrs() {
            Ok(email_server_socket_address_registry_) => email_server_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let email_server_socket_address_ = match email_server_socket_address_registry.next() {
            Some(email_server_socket_address__) => email_server_socket_address__,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Invalid socket address." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            EnvironmentConfiguration::new(
                application_server_socket_address_,
                database_1_postgresql_configuration,
                database_2_postgresql_configuration,
                database_1_redis_connection_info,
                environment,
                environment_file_configuration.encryption.private_key.application_user_access_refresh_token,
                environment_file_configuration.encryption.private_key.application_user_access_token,
                email_server_socket_address_
            )
        );
    }
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct EnvironmentFileConfiguration {
    application: Application,
    resource: Resource,
    encryption: Encryption
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct Application {
    socket_address: String
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct Resource {
    postgresql: Postgresql,
    redis: Redis,
    email_server: EmailServer
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct Postgresql {
    database_1_url: String,
    database_2_url: String
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct Redis {
    database_1_url: String
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct EmailServer {
    socket_address: String
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct Encryption {
    private_key: PrivateKey
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct PrivateKey {
    application_user_access_token: String,
    application_user_access_refresh_token: String
}