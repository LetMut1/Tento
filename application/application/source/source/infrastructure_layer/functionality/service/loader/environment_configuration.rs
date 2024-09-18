use super::Loader;
use crate::infrastructure_layer::data::environment_configuration::{
    environment_configuration::{
        ApplicationServer,
        EmailServer,
        Encryption,
        EnvironmentConfiguration,
        Http,
        HttpKeepalive,
        Logging,
        Postgresql,
        PrivateKey,
        Resource,
        Tcp,
        TcpKeepalive,
        Tls,
        TokioRuntime,
    },
    environment_configuration_file::EnvironmentConfigurationFile,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
    ResultConverter,
};
use std::{
    net::ToSocketAddrs,
    path::Path,
};
impl Loader<EnvironmentConfiguration> {
    const ENVIRONMENT_FILE_NAME: &'static str = "environment.toml";
    pub fn load_from_file<'a>(environment_configuration_directory_path: &'a str) -> Result<EnvironmentConfiguration, AggregateError> {
        let environment_file_path = format!(
            "{}/{}",
            environment_configuration_directory_path,
            Self::ENVIRONMENT_FILE_NAME,
        );
        let environment_file_path_ = Path::new(environment_file_path.as_str());
        let environment_file_data = if environment_file_path_.try_exists().into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )? {
            std::fs::read_to_string(environment_file_path_).into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?
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
        let environment_configuration_file = toml::from_str::<EnvironmentConfigurationFile>(environment_file_data.as_str()).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let mut application_server_tcp_socket_address_registry = environment_configuration_file.application_server.tcp.socket_address.value.to_socket_addrs().into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let application_server_tcp_socket_address = application_server_tcp_socket_address_registry.next().ok_or_else(
            || -> _ {
                return AggregateError::new_logic_(
                    Common::InvalidSocketAddress,
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                );
            },
        )?;
        let mut email_server_tcp_socket_address_registry = environment_configuration_file.resource.email_server.socket_address.value.to_socket_addrs().into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let email_server_tcp_socket_address = email_server_tcp_socket_address_registry.next().ok_or_else(
            || -> _ {
                return AggregateError::new_logic_(
                    Common::InvalidSocketAddress,
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                );
            },
        )?;
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
                resource: Resource {
                    postgresql: Postgresql {
                        database_1_url: environment_configuration_file.resource.postgresql.database_1_url.value,
                        database_2_url: environment_configuration_file.resource.postgresql.database_2_url.value,
                    },
                    email_server: EmailServer {
                        socket_address: email_server_tcp_socket_address,
                    },
                },
                encryption: Encryption {
                    private_key: PrivateKey {
                        application_user_access_token: environment_configuration_file.encryption.private_key.application_user_access_token.value,
                        application_user_access_refresh_token: environment_configuration_file.encryption.private_key.application_user_access_refresh_token.value,
                    },
                },
            },
        );
    }
}
