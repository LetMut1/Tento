use super::environment_configuration::environment_configuration_file::EnvironmentConfigurationFile;
use super::environment_configuration::ApplicationServer;
use super::environment_configuration::EmailServer;
use super::environment_configuration::Encryption;
use super::environment_configuration::Environment;
use super::environment_configuration::EnvironmentConfiguration;
use super::environment_configuration::Http;
use super::environment_configuration::HttpKeepalive;
use super::environment_configuration::Postgresql;
use super::environment_configuration::PrivateKey;
use super::environment_configuration::Redis;
use super::environment_configuration::Resource;
use super::environment_configuration::String_;
use super::environment_configuration::Tcp;
use super::environment_configuration::TcpKeepalive;
use super::environment_configuration::Tls;
use super::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use toml::from_str;

pub struct Loader;

impl Loader {
    const PRODUCTION_ENVIRONMENT_DIRECTORY_NAME: &'static str = "production";
    const DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "development";
    const LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "local_development";
    const ENVIRONMENT_FILE_NAME: &'static str = "environment.toml";

    pub fn load_from_file<'a>(environment_configuration_directory_path: &'a str) -> Result<EnvironmentConfiguration<String_>, Error> {
        let production_environment_file_path = format!(
            "{}/{}/{}",
            environment_configuration_directory_path,
            Self::PRODUCTION_ENVIRONMENT_DIRECTORY_NAME,
            Self::ENVIRONMENT_FILE_NAME,
        );

        let production_environment_file_path_ = Path::new(production_environment_file_path.as_str());

        let (environment, environment_file_data) = if production_environment_file_path_.try_exists()? {
            (
                Environment::Production,
                read_to_string(production_environment_file_path_)?,
            )
        } else {
            let local_development_environment_file_path = format!(
                "{}/{}/{}",
                environment_configuration_directory_path,
                Self::LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME,
                Self::ENVIRONMENT_FILE_NAME,
            );

            let local_development_environment_file_path_ = Path::new(local_development_environment_file_path.as_str());

            if local_development_environment_file_path_.try_exists()? {
                (
                    Environment::LocalDevelopment,
                    read_to_string(local_development_environment_file_path_)?,
                )
            } else {
                let development_environment_file_path = format!(
                    "{}/{}/{}",
                    environment_configuration_directory_path,
                    Self::DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME,
                    Self::ENVIRONMENT_FILE_NAME,
                );

                let development_environment_file_path_ = Path::new(development_environment_file_path.as_str());

                if development_environment_file_path_.try_exists()? {
                    (
                        Environment::Development,
                        read_to_string(development_environment_file_path_)?,
                    )
                } else {
                    return Err(
                        Error::LogicError {
                            message: "The environment.toml file does not exist.",
                        },
                    );
                }
            }
        };

        let environment_configuration_file = from_str::<EnvironmentConfigurationFile>(environment_file_data.as_str())?;

        let environment_configuration = {
            let application_server = {
                let tcp = {
                    let keepalive = {
                        let duration = if environment_configuration_file.application_server.tcp.keepalive.duration.is_exist {
                            Some(environment_configuration_file.application_server.tcp.keepalive.duration.value)
                        } else {
                            None
                        };

                        let interval_duration = if environment_configuration_file.application_server.tcp.keepalive.interval_duration.is_exist {
                            Some(environment_configuration_file.application_server.tcp.keepalive.interval_duration.value)
                        } else {
                            None
                        };

                        let retries_quantity = if environment_configuration_file.application_server.tcp.keepalive.retries_quantity.is_exist {
                            Some(environment_configuration_file.application_server.tcp.keepalive.retries_quantity.value)
                        } else {
                            None
                        };

                        TcpKeepalive {
                            duration,
                            interval_duration,
                            retries_quantity,
                        }
                    };

                    Tcp {
                        socket_address: String_(environment_configuration_file.application_server.tcp.socket_address.value),
                        nodelay: environment_configuration_file.application_server.tcp.nodelay.value,
                        sleep_on_accept_errors: environment_configuration_file.application_server.tcp.sleep_on_accept_errors.value,
                        keepalive,
                    }
                };

                let http = {
                    let keepalive = if environment_configuration_file.application_server.http.keepalive.is_exist {
                        Some(
                            HttpKeepalive {
                                interval_duration: environment_configuration_file.application_server.http.keepalive.interval_duration.value,
                                timeout_duration: environment_configuration_file.application_server.http.keepalive.timeout_duration.value,
                            },
                        )
                    } else {
                        None
                    };

                    let tls = if environment_configuration_file.application_server.http.tls.is_exist {
                        Some(
                            Tls {
                                certificate_crt_path: String_(environment_configuration_file.application_server.http.tls.certificate_crt_path.value),
                                certificate_key_path: String_(environment_configuration_file.application_server.http.tls.certificate_key_path.value),
                            },
                        )
                    } else {
                        None
                    };

                    let maximum_pending_accept_reset_streams = if environment_configuration_file.application_server.http.maximum_pending_accept_reset_streams.is_exist {
                        Some(environment_configuration_file.application_server.http.maximum_pending_accept_reset_streams.value)
                    } else {
                        None
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

            let resource = Resource {
                postgresql: Postgresql {
                    database_1_url: String_(environment_configuration_file.resource.postgresql.database_1_url.value),
                    database_2_url: String_(environment_configuration_file.resource.postgresql.database_2_url.value),
                },
                redis: Redis {
                    database_1_url: String_(environment_configuration_file.resource.redis.database_1_url.value),
                },
                email_server: EmailServer {
                    socket_address: String_(environment_configuration_file.resource.email_server.socket_address.value),
                },
            };

            let encryption = Encryption {
                private_key: PrivateKey {
                    application_user_access_token: String_(environment_configuration_file.encryption.private_key.application_user_access_token.value),
                    application_user_access_refresh_token: String_(environment_configuration_file.encryption.private_key.application_user_access_refresh_token.value),
                },
            };

            EnvironmentConfiguration {
                environment,
                application_server,
                resource,
                encryption,
            }
        };

        return Ok(environment_configuration);
    }
}