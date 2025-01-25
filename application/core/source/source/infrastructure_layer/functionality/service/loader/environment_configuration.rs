use super::Loader;
use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        Backtrace,
    },
    environment_configuration::{
        EnvironmentConfiguration,
        Postgresql,
        PostgresqlInner,
    },
    environment_configuration::run_server::{
        ApplicationServer,
        EmailServer,
        Encryption,
        RunServer,
        EnvironmentConfigurationFile as RunServerEnvironmentConfigurationFile,
        Http,
        HttpKeepalive,
        Logging,
        PrivateKey,
        Resource as RunServerResource,
        Tcp,
        TcpKeepalive,
        Tls,
        TokioRuntime,
    },
    environment_configuration::create_fixtures::{
        CreateFixtures,
        EnvironmentConfigurationFile as CreateFixturesEnvironmentConfigurationFile,
        Resource as CreateFixturesResource,
    },
};
use std::{
    net::ToSocketAddrs,
    path::Path,
    str::FromStr,
};
use serde::Deserialize;
use tokio_postgres::config::Config;
impl Loader<EnvironmentConfiguration<RunServer>> {
    pub fn load_from_file<'a>(environment_configuration_file_path: &'a str) -> Result<EnvironmentConfiguration<RunServer>, AggregateError> {
        let environment_configuration_file = load_from_file::<RunServerEnvironmentConfigurationFile>(environment_configuration_file_path)?;
        let mut application_server_tcp_socket_address_registry = crate::result_return_runtime!(
            environment_configuration_file.application_server.tcp.socket_address.value.to_socket_addrs()
        );
        let application_server_tcp_socket_address = crate::option_return_logic_invalid_socket_address!(application_server_tcp_socket_address_registry.next());
        let mut email_server_tcp_socket_address_registry = crate::result_return_runtime!(
            environment_configuration_file.resource.email_server.socket_address.value.to_socket_addrs()
        );
        let email_server_tcp_socket_address = crate::option_return_logic_invalid_socket_address!(email_server_tcp_socket_address_registry.next());
        let postgreql_database_1_configuration = crate::result_return_logic!(
            Config::from_str(environment_configuration_file.resource.postgresql.database_1.url.value.as_str())
        );
        let postgreql_database_2_configuration = crate::result_return_logic!(
            Config::from_str(environment_configuration_file.resource.postgresql.database_2.url.value.as_str())
        );
        let application_server = {
            let tcp = {
                let keepalive = {
                    let duration = if environment_configuration_file.application_server.tcp.keepalive.duration.is_exist {
                        Option::Some(environment_configuration_file.application_server.tcp.keepalive.duration.value)
                    } else {
                        Option::None
                    };
                    let interval_duration = if environment_configuration_file.application_server.tcp.keepalive.interval_duration.is_exist {
                        Option::Some(environment_configuration_file.application_server.tcp.keepalive.interval_duration.value)
                    } else {
                        Option::None
                    };
                    let retries_quantity = if environment_configuration_file.application_server.tcp.keepalive.retries_quantity.is_exist {
                        Option::Some(environment_configuration_file.application_server.tcp.keepalive.retries_quantity.value)
                    } else {
                        Option::None
                    };
                    TcpKeepalive {
                        duration,
                        interval_duration,
                        retries_quantity,
                    }
                };
                Tcp {
                    socket_address: application_server_tcp_socket_address,
                    nodelay: environment_configuration_file.application_server.tcp.nodelay.value,
                    sleep_on_accept_errors: environment_configuration_file.application_server.tcp.sleep_on_accept_errors.value,
                    keepalive,
                }
            };
            let http = {
                let keepalive = if environment_configuration_file.application_server.http.keepalive.is_exist {
                    Option::Some(
                        HttpKeepalive {
                            interval_duration: environment_configuration_file.application_server.http.keepalive.interval_duration.value,
                            timeout_duration: environment_configuration_file.application_server.http.keepalive.timeout_duration.value,
                        },
                    )
                } else {
                    Option::None
                };
                let tls = if environment_configuration_file.application_server.http.tls.is_exist {
                    Option::Some(
                        Tls {
                            certificate_crt_file_path: environment_configuration_file.application_server.http.tls.certificate_crt_file_path.value,
                            certificate_key_file_path: environment_configuration_file.application_server.http.tls.certificate_key_file_path.value,
                        },
                    )
                } else {
                    Option::None
                };
                let maximum_pending_accept_reset_streams = if environment_configuration_file.application_server.http.maximum_pending_accept_reset_streams.is_exist {
                    Option::Some(environment_configuration_file.application_server.http.maximum_pending_accept_reset_streams.value)
                } else {
                    Option::None
                };
                Http {
                    adaptive_window: environment_configuration_file.application_server.http.adaptive_window.value,
                    connection_window_size: environment_configuration_file.application_server.http.connection_window_size.value,
                    stream_window_size: environment_configuration_file.application_server.http.stream_window_size.value,
                    maximum_frame_size: environment_configuration_file.application_server.http.maximum_frame_size.value,
                    maximum_sending_buffer_size: environment_configuration_file.application_server.http.maximum_sending_buffer_size.value,
                    enable_connect_protocol: environment_configuration_file.application_server.http.enable_connect_protocol.value,
                    maximum_header_list_size: environment_configuration_file.application_server.http.maximum_header_list_size.value,
                    maximum_pending_accept_reset_streams,
                    keepalive,
                    tls,
                }
            };
            ApplicationServer {
                tcp,
                http,
            }
        };
        return Result::Ok(
            EnvironmentConfiguration {
                subject: RunServer {
                    tokio_runtime: TokioRuntime {
                        maximum_blocking_threads_quantity: environment_configuration_file.tokio_runtime.maximum_blocking_threads_quantity.value,
                        worker_threads_quantity: environment_configuration_file.tokio_runtime.worker_threads_quantity.value,
                        worker_thread_stack_size: environment_configuration_file.tokio_runtime.worker_thread_stack_size.value,
                    },
                    application_server,
                    logging: Logging {
                        directory_path: environment_configuration_file.logging.directory_path.value,
                        file_name_prefix: environment_configuration_file.logging.file_name_prefix.value,
                    },
                    resource: RunServerResource {
                        postgresql: Postgresql {
                            database_1: PostgresqlInner {
                                configuration: postgreql_database_1_configuration,
                                maximum_connection_pool_size: environment_configuration_file.resource.postgresql.database_1.maximum_connection_pool_size.value,
                                connection_pool_waiting_timeout_duration: environment_configuration_file.resource.postgresql.database_1.connection_pool_waiting_timeout_duration.value,
                            },
                            database_2: PostgresqlInner {
                                configuration: postgreql_database_2_configuration,
                                maximum_connection_pool_size: environment_configuration_file.resource.postgresql.database_2.maximum_connection_pool_size.value,
                                connection_pool_waiting_timeout_duration: environment_configuration_file.resource.postgresql.database_2.connection_pool_waiting_timeout_duration.value,
                            },
                        },
                        email_server: EmailServer {
                            socket_address: email_server_tcp_socket_address,
                        },
                    },
                    encryption: Encryption {
                        private_key: PrivateKey {
                            user_access_token: environment_configuration_file.encryption.private_key.user_access_token.value,
                            user_access_refresh_token: environment_configuration_file.encryption.private_key.user_access_refresh_token.value,
                        },
                    },
                },
            },
        );
    }
}
impl Loader<EnvironmentConfiguration<CreateFixtures>> {
    pub fn load_from_file<'a>(environment_configuration_file_path: &'a str) -> Result<EnvironmentConfiguration<CreateFixtures>, AggregateError> {
        let environment_configuration_file = load_from_file::<CreateFixturesEnvironmentConfigurationFile>(environment_configuration_file_path)?;
        let postgreql_database_1_configuration = crate::result_return_logic!(
            Config::from_str(environment_configuration_file.resource.postgresql.database_1.url.value.as_str())
        );
        let postgreql_database_2_configuration = crate::result_return_logic!(
            Config::from_str(environment_configuration_file.resource.postgresql.database_2.url.value.as_str())
        );
        return Result::Ok(
            EnvironmentConfiguration {
                subject: CreateFixtures {
                    resource: CreateFixturesResource {
                        postgresql: Postgresql {
                            database_1: PostgresqlInner {
                                configuration: postgreql_database_1_configuration,
                                maximum_connection_pool_size: environment_configuration_file.resource.postgresql.database_1.maximum_connection_pool_size.value,
                                connection_pool_waiting_timeout_duration: environment_configuration_file.resource.postgresql.database_1.connection_pool_waiting_timeout_duration.value,
                            },
                            database_2: PostgresqlInner {
                                configuration: postgreql_database_2_configuration,
                                maximum_connection_pool_size: environment_configuration_file.resource.postgresql.database_2.maximum_connection_pool_size.value,
                                connection_pool_waiting_timeout_duration: environment_configuration_file.resource.postgresql.database_2.connection_pool_waiting_timeout_duration.value,
                            },
                        },
                    },
                },
            },
        );
    }
}
pub fn load_from_file<'a, T>(environment_configuration_file_path: &'a str) -> Result<T, AggregateError>
where
    T: for<'de> Deserialize<'de>,
{
    let environment_configuration_file_path_ = Path::new(environment_configuration_file_path);
    let environment_file_data = if crate::result_return_runtime!(environment_configuration_file_path_.try_exists()) {
        crate::result_return_logic!(std::fs::read_to_string(environment_configuration_file_path_))
    } else {
        return Result::Err(
            AggregateError::new_logic(
                "The environment.toml file does not exist.".into(),
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            ),
        );
    };
    return crate::result_into_logic!(toml::from_str::<T>(environment_file_data.as_str()));
}