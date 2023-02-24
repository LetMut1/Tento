use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::dotenv;
use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::Config as PostgresqlConfiguration;
use std::env;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::str::FromStr;

pub struct EnvironmentConfiguration_Creator;

impl EnvironmentConfiguration_Creator {
    const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "production.env";  // TODO Посмотреть, какие есть еще лучшие форматы аналоги .env (Может, Томл?)
    const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.env";  // TODO TODO TODO TODO TODOenv::remove_var can PANIC. Подумать, что делать. Использовать другой крейт (toml), или написать свой парсер. Паника - всегжа плохо
    const LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "development.local.env";

    pub fn create_from_configuration_file(configuration_file_path: &'static str) -> Result<EnvironmentConfiguration, ErrorAuditor> {
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

        let environment = if production_environment_file_path_buffer.exists() {
            if let Err(error) = dotenv::from_path(production_environment_file_path_buffer.as_path()) {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }

            Environment::Production
        } else {
            let local_development_environment_file_path_buffer = file_path.join(
                Path::new(Self::LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME)
            );

            if local_development_environment_file_path_buffer.exists() {
                if let Err(error) = dotenv::from_path(local_development_environment_file_path_buffer.as_path()) {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }

                Environment::LocalDevelopment
            } else {
                let development_environment_file_path_buffer = file_path.join(
                    Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME)
                );

                if development_environment_file_path_buffer.exists() {
                    if let Err(error) = dotenv::from_path(development_environment_file_path_buffer.as_path()) {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                } else {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError { message: "Any ....env files does not exist." },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }

                Environment::Development
            }
        };

        let application_server_socket_address = match env::var(EnvironmentConfiguration::APPLICATION_SERVER_SOCKET_ADDRESS_KEY) {
            Ok(application_server_socket_address_) => application_server_socket_address_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let mut application_server_socket_address_registry = match application_server_socket_address.to_socket_addrs() {
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
        env::remove_var(EnvironmentConfiguration::APPLICATION_SERVER_SOCKET_ADDRESS_KEY);

        let security_auart_encoding_private_key = match env::var(EnvironmentConfiguration::SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY) {
            Ok(security_auart_encoding_private_key_) => security_auart_encoding_private_key_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfiguration::SECURITY_AUART_ENCODING_PRIVATE_KEY_KEY);

        let security_auat_signature_encoding_private_key = match env::var(EnvironmentConfiguration::SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY) {
            Ok(security_auat_signature_encoding_private_key_) => security_auat_signature_encoding_private_key_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfiguration::SECURITY_AUAT_SIGNATURE_ENCODING_PRIVATE_KEY_KEY);

        let resource_database_1_postgresql_url = match env::var(EnvironmentConfiguration::RESOURCE_DATABASE_1_POSTGRESQL_URL_KEY) {
            Ok(resource_database_1_postgresql_url_) => resource_database_1_postgresql_url_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_database_1_postgresql_configuration = match PostgresqlConfiguration::from_str(resource_database_1_postgresql_url.as_str()) {
            Ok(postgresql_configuration) => postgresql_configuration,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfiguration::RESOURCE_DATABASE_1_POSTGRESQL_URL_KEY);

        let resource_database_2_postgresql_url = match env::var(EnvironmentConfiguration::RESOURCE_DATABASE_2_POSTGRESQL_URL_KEY) {
            Ok(resource_database_2_postgresql_url_) =>  resource_database_2_postgresql_url_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_database_2_postgresql_configuration = match PostgresqlConfiguration::from_str(resource_database_2_postgresql_url.as_str()) {
            Ok(postgresql_configuration) => postgresql_configuration,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfiguration::RESOURCE_DATABASE_2_POSTGRESQL_URL_KEY);

        let resource_redis_url = match env::var(EnvironmentConfiguration::RESOURCE_REDIS_URL_KEY) {
            Ok(resource_redis_url_) => resource_redis_url_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_redis_connection_info = match ConnectionInfo::from_str(resource_redis_url.as_str()) {
            Ok(connection_info) => connection_info,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfiguration::RESOURCE_REDIS_URL_KEY);

        let resource_email_server_socket_address = match env::var(EnvironmentConfiguration::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY) {
            Ok(resource_email_server_socket_address_) => resource_email_server_socket_address_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let mut resource_email_server_socket_address_registry = match resource_email_server_socket_address.to_socket_addrs() {
            Ok(resource_email_server_socket_address_registry_) => resource_email_server_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let resource_email_server_socket_address_ = match resource_email_server_socket_address_registry.next() {
            Some(resource_email_server_socket_address__) => resource_email_server_socket_address__,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Invalid socket address." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        env::remove_var(EnvironmentConfiguration::RESOURCE_EMAIL_SERVER_SOCKET_ADDRESS_KEY);

        return Ok(
            EnvironmentConfiguration::new(
                environment,
                application_server_socket_address_,
                security_auart_encoding_private_key,
                security_auat_signature_encoding_private_key,
                resource_database_1_postgresql_configuration,
                resource_database_2_postgresql_configuration,
                resource_redis_connection_info,
                resource_email_server_socket_address_
            )
        );
    }
}