#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_match,
    clippy::explicit_into_iter_loop,
    clippy::module_inception,
    clippy::needless_continue,
    clippy::needless_lifetimes,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::redundant_pattern_matching,
    clippy::single_match_else,
    clippy::string_add,
    clippy::too_many_arguments,
    clippy::trait_duplication_in_bounds,
    clippy::unused_unit,
    clippy::empty_enum,
    clippy::let_unit_value,
    clippy::let_and_return,
    clippy::manual_range_contains,
    clippy::enum_variant_names,
    clippy::type_complexity,
    clippy::explicit_auto_deref,
    clippy::redundant_static_lifetimes,
    clippy::manual_map
)]
#![deny(
    clippy::unnecessary_cast,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::fallible_impl_from,
    clippy::float_cmp_const,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::mem_forget,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values
)]

pub mod environment_configuration {
    use self::sealed::Sealed;

    pub enum Environment {
        Production,
        Development,
        LocalDevelopment,
    }

    pub struct EnvironmentConfiguration<T>
    where
        T: Sealed
    {
        pub environment: Environment,
        pub application_server: ApplicationServer<T>,
        pub resource: Resource<T>,
        pub encryption: Encryption<T>,
    }

    pub struct ApplicationServer<T>
    where
        T: Sealed
    {
        pub tcp: Tcp<T>,
        pub http: Http<T>,
    }

    pub struct Tcp<T>
    where
        T: Sealed
    {
        pub socket_address: T,
        pub nodelay: bool,
        pub sleep_on_accept_errors: bool,
        pub keepalive: TcpKeepalive,
    }

    pub struct TcpKeepalive {
        pub duration: Option<u64>,
        pub interval_duration: Option<u64>,
        pub retries_quantity: Option<u32>,
    }

    pub struct Http<T>
    where
        T: Sealed
    {
        pub adaptive_window: bool,
        pub connection_window_size: u32,
        pub stream_window_size: u32,
        pub maximum_frame_size: u32,
        pub maximum_sending_buffer_size: u32,
        pub enable_connect_protocol: bool,
        pub maximum_header_list_size: u32,
        pub maximum_pending_accept_reset_streams: Option<usize>,
        pub keepalive: Option<HttpKeepalive>,
        pub tls: Option<Tls<T>>
    }

    pub struct HttpKeepalive {
        pub interval_duration: u64,
        pub timeout_duration: u64,
    }

    pub struct Tls<T>
    where
        T: Sealed
    {
        pub certificate_crt_path: T,
        pub certificate_key_path: T,
    }

    pub struct Resource<T>
    where
        T: Sealed
    {
        pub postgresql: Postgresql<T>,
        pub redis: Redis<T>,
        pub email_server: EmailServer<T>,
    }

    pub struct Postgresql<T>
    where
        T: Sealed
    {
        pub database_1_url: T,
        pub database_2_url: T,
    }

    pub struct Redis<T>
    where
        T: Sealed
    {
        pub database_1_url: T,
    }

    pub struct EmailServer<T>
    where
        T: Sealed
    {
        pub socket_address: T,
    }

    pub struct Encryption<T>
    where
        T: Sealed
    {
        pub private_key: PrivateKey<T>,
    }

    pub struct PrivateKey<T>
    where
        T: Sealed
    {
        pub application_user_access_token: T,
        pub application_user_access_refresh_token: T,
    }

    pub struct String_(pub String);

    pub struct StringLiteral(pub &'static str);

    mod sealed {
        use super::StringLiteral;
        use super::String_;

        pub trait Sealed {}

        impl Sealed for StringLiteral {}

        impl Sealed for String_ {}
    }

    pub(crate) mod environment_configuration_file {
        use serde::Deserialize;

        #[derive(Deserialize)]
        pub struct EnvironmentConfigurationFile {
            pub application_server: ApplicationServer,
            pub resource: Resource,
            pub encryption: Encryption,
        }

        #[derive(Deserialize)]
        pub struct ApplicationServer {
            pub tcp: Tcp,
            pub http: Http,
        }

        #[derive(Deserialize)]
        pub struct Tcp {
            pub socket_address: Value<String>,
            pub nodelay: Value<bool>,
            pub sleep_on_accept_errors: Value<bool>,
            pub keepalive: TcpKeepalive,
        }

        #[derive(Deserialize)]
        pub struct TcpKeepalive {
            pub duration: ValueExist<u64>,
            pub interval_duration: ValueExist<u64>,
            pub retries_quantity: ValueExist<u32>,
        }

        #[derive(Deserialize)]
        pub struct Http {
            pub adaptive_window: Value<bool>,
            pub connection_window_size: Value<u32>,
            pub stream_window_size: Value<u32>,
            pub maximum_frame_size: Value<u32>,
            pub maximum_sending_buffer_size: Value<u32>,
            pub enable_connect_protocol: Value<bool>,
            pub maximum_header_list_size: Value<u32>,
            pub maximum_pending_accept_reset_streams: ValueExist<usize>,
            pub keepalive: HttpKeepalive,
            pub tls: Tls
        }

        #[derive(Deserialize)]
        pub struct HttpKeepalive {
            pub is_exist: bool,
            pub interval_duration: Value<u64>,
            pub timeout_duration: Value<u64>,
        }

        #[derive(Deserialize)]
        pub struct Tls {
            pub is_exist: bool,
            pub certificate_crt_path: Value<String>,
            pub certificate_key_path: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Resource {
            pub postgresql: Postgresql,
            pub redis: Redis,
            pub email_server: EmailServer,
        }

        #[derive(Deserialize)]
        pub struct Postgresql {
            pub database_1_url: Value<String>,
            pub database_2_url: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Redis {
            pub database_1_url: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct EmailServer {
            pub socket_address: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Encryption {
            pub private_key: PrivateKey,
        }

        #[derive(Deserialize)]
        pub struct PrivateKey {
            pub application_user_access_token: Value<String>,
            pub application_user_access_refresh_token: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Value<T> {
            pub value: T,
        }

        #[derive(Deserialize)]
        pub struct ValueExist<T> {
            pub value: T,
            pub is_exist: bool,
        }
    }
}

pub mod error {
    use std::convert::From;
    use std::error::Error as StdError;
    use std::fmt::Display;
    use std::fmt::Error as FormatError;
    use std::fmt::Formatter;
    use std::io::Error as IOError;
    use toml::de::Error as TomlError;

    #[derive(Debug)]
    pub enum Error {
        LogicError {
            message: &'static str,
        },
        OtherError {
            other_error: OtherError,
        },
    }

    impl Display for Error {
        fn fmt<'a, 'b>(
            &'a self,
            formatter: &'b mut Formatter<'_>,
        ) -> Result<(), FormatError> {
            match *self {
                Self::LogicError {
                    ref message,
                } => {
                    return write!(
                        formatter,
                        "Error, logic: {}.",
                        message
                    );
                }
                Self::OtherError {
                    ref other_error,
                } => {
                    return write!(
                        formatter,
                        "Error, other: {}.",
                        other_error.get_message()
                    );
                }
            }
        }
    }

    impl StdError for Error {}

    impl From<IOError> for Error {
        fn from(value: IOError) -> Self {
            return Self::OtherError {
                other_error: OtherError::new(&value),
            };
        }
    }

    impl From<TomlError> for Error {
        fn from(value: TomlError) -> Self {
            return Self::OtherError {
                other_error: OtherError::new(&value),
            };
        }
    }

    #[derive(Debug)]
    pub struct OtherError {
        message: String,
    }

    impl OtherError {
        pub fn new<E>(error: E) -> Self
        where
            E: StdError,
        {
            return Self {
                message: format!(
                    "{}",
                    error
                ),
            };
        }

        pub fn get_message<'a>(&'a self) -> &'a str {
            return self.message.as_str();
        }
    }

    impl Display for OtherError {
        fn fmt<'a, 'b>(
            &'a self,
            _: &'b mut Formatter<'_>,
        ) -> Result<(), FormatError> {
            return Ok(());
        }
    }
}

pub mod loader {
    use super::environment_configuration::HttpKeepalive;
    use super::environment_configuration::String_;
    use super::environment_configuration::Tcp;
    use super::environment_configuration::Http;
    use super::environment_configuration::Environment;
    use super::environment_configuration::EnvironmentConfiguration;
    use super::environment_configuration::Tls;
    use super::environment_configuration::EmailServer;
    use super::environment_configuration::Encryption;
    use super::environment_configuration::Postgresql;
    use super::environment_configuration::PrivateKey;
    use super::environment_configuration::TcpKeepalive;
    use super::environment_configuration::Redis;
    use super::environment_configuration::Resource;
    use super::environment_configuration::ApplicationServer;
    use super::environment_configuration::environment_configuration_file::EnvironmentConfigurationFile;
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
                            read_to_string(development_environment_file_path_)?
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
                            keepalive
                        }
                    };

                    let http = {
                        let keepalive = if environment_configuration_file.application_server.http.keepalive.is_exist {
                            Some(
                                HttpKeepalive {
                                    interval_duration: environment_configuration_file.application_server.http.keepalive.interval_duration.value,
                                    timeout_duration: environment_configuration_file.application_server.http.keepalive.timeout_duration.value,
                                }
                            )
                        } else {
                            None
                        };

                        let tls = if environment_configuration_file.application_server.http.tls.is_exist {
                            Some(
                                Tls {
                                    certificate_crt_path: String_(environment_configuration_file.application_server.http.tls.certificate_crt_path.value),
                                    certificate_key_path: String_(environment_configuration_file.application_server.http.tls.certificate_key_path.value),
                                }
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
                            enable_connect_protocol:environment_configuration_file.application_server.http.enable_connect_protocol.value,
                            maximum_header_list_size:environment_configuration_file.application_server.http.maximum_header_list_size.value,
                            maximum_pending_accept_reset_streams,
                            keepalive,
                            tls
                        }
                    };

                    ApplicationServer {
                        tcp,
                        http
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
                    encryption
                }
            };

            return Ok(environment_configuration);
        }
    }
}
