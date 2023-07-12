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
    use serde::Deserialize;
    use self::sealed::Sealed;

    pub struct EnvironmentConfiguration<T>
    where
        T: Sealed
    {
        pub environment: Environment,
        pub environment_configuration_file: EnvironmentConfigurationFile<T>,
    }

    pub enum Environment {
        Production,
        Development,
        LocalDevelopment,
    }

    #[derive(Deserialize)]
    pub struct EnvironmentConfigurationFile<T>
    where
        T: Sealed
    {
        pub application: Application<T>,
        pub resource: Resource<T>,
        pub encryption: Encryption<T>,
    }

    #[derive(Deserialize)]
    pub struct Application<T>
    where
        T: Sealed
    {
        pub tcp: Tcp<T>,
        pub http: Http,
    }

    #[derive(Deserialize)]
    pub struct Tcp<T>
    where
        T: Sealed
    {
        pub socket_address: SimpleType<T>,
        pub nodelay: SimpleType<bool>,
        pub sleep_on_accept_errors: SimpleType<bool>,
        pub keepalive_seconds: SimpleTypeActive<u64>,
    }

    #[derive(Deserialize)]
    pub struct Http {
        pub adaptive_window: SimpleType<bool>,
        pub connection_window_size: SimpleType<u32>,
        pub stream_window_size: SimpleType<u32>,
        pub maximum_frame_size: SimpleType<u32>,
        pub maximum_sending_buffer_size: SimpleType<u32>,
        pub http2_only: SimpleType<bool>,
        pub keep_alive: KeepAlive,
    }

    #[derive(Deserialize)]
    pub struct KeepAlive {
        pub is_active: bool,
        pub interval_seconds: SimpleType<u64>,
        pub timeout_seconds: SimpleType<u64>,
    }

    #[derive(Deserialize)]
    pub struct Resource<T>
    where
        T: Sealed
    {
        pub postgresql: Postgresql<T>,
        pub redis: Redis<T>,
        pub email_server: EmailServer<T>,
    }

    #[derive(Deserialize)]
    pub struct Postgresql<T>
    where
        T: Sealed
    {
        pub database_1_url: SimpleType<T>,
        pub database_2_url: SimpleType<T>,
    }

    #[derive(Deserialize)]
    pub struct Redis<T>
    where
        T: Sealed
    {
        pub database_1_url: SimpleType<T>,
    }

    #[derive(Deserialize)]
    pub struct EmailServer<T>
    where
        T: Sealed
    {
        pub socket_address: SimpleType<T>,
    }

    #[derive(Deserialize)]
    pub struct Encryption<T>
    where
        T: Sealed
    {
        pub private_key: PrivateKey<T>,
    }

    #[derive(Deserialize)]
    pub struct PrivateKey<T>
    where
        T: Sealed
    {
        pub application_user_access_token: SimpleType<T>,
        pub application_user_access_refresh_token: SimpleType<T>,
    }

    #[derive(Deserialize)]
    pub struct SimpleTypeActive<T> {
        pub value: T,
        pub is_active: bool,
    }

    #[derive(Deserialize)]
    pub struct SimpleType<T> {
        pub value: T,
    }

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct String_(pub String);

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct Str(pub &'static str);

    mod sealed {
        use super::Str;
        use super::String_;

        pub trait Sealed {}

        impl Sealed for Str {}

        impl Sealed for String_ {}
    }
}

pub mod error {
    use std::error::Error as StdError;
    use std::fmt::Display;
    use std::fmt::Error as FormatError;
    use std::fmt::Formatter;

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
    use super::environment_configuration::Environment;
    use super::environment_configuration::EnvironmentConfiguration;
    use super::environment_configuration::EnvironmentConfigurationFile;
    use super::error::Error;
    use super::error::OtherError;
    use std::fs::read_to_string;
    use super::environment_configuration::String_;
    use std::path::Path;
    use toml::from_str;

    pub struct Loader;

    impl Loader {
        const PRODUCTION_ENVIRONMENT_FILE_NAME: &'static str = "environment.production.toml";
        const DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "environment.development.toml";
        const LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME: &'static str = "environment.development.local.toml";

        pub fn load_from_file<'a>(environment_configuration_file_path: &'a str) -> Result<EnvironmentConfiguration<String_>, Error> {
            let file_path = Path::new(environment_configuration_file_path);

            let production_environment_file_path_buffer = file_path.join(Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME));

            let (environment, environment_file_data) = if production_environment_file_path_buffer.exists() {
                let environment_file_data_ = match read_to_string(production_environment_file_path_buffer.as_path()) {
                    Ok(environment_file_data__) => environment_file_data__,
                    Err(error) => {
                        return Err(
                            Error::OtherError {
                                other_error: OtherError::new(error),
                            },
                        );
                    }
                };

                (
                    Environment::Production,
                    environment_file_data_,
                )
            } else {
                let local_development_environment_file_path_buffer = file_path.join(Path::new(Self::LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME));

                if local_development_environment_file_path_buffer.exists() {
                    let environment_file_data_ = match read_to_string(local_development_environment_file_path_buffer.as_path()) {
                        Ok(environment_file_data__) => environment_file_data__,
                        Err(error) => {
                            return Err(
                                Error::OtherError {
                                    other_error: OtherError::new(error),
                                },
                            );
                        }
                    };

                    (
                        Environment::LocalDevelopment,
                        environment_file_data_,
                    )
                } else {
                    let development_environment_file_path_buffer = file_path.join(Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME));

                    let environment_file_data_ = if development_environment_file_path_buffer.exists() {
                        let environment_file_data__ = match read_to_string(development_environment_file_path_buffer.as_path()) {
                            Ok(environment_file_data___) => environment_file_data___,
                            Err(error) => {
                                return Err(
                                    Error::OtherError {
                                        other_error: OtherError::new(error),
                                    },
                                );
                            }
                        };

                        environment_file_data__
                    } else {
                        return Err(
                            Error::LogicError {
                                message: "Environment configuration files does not exist.",
                            },
                        );
                    };

                    (
                        Environment::Development,
                        environment_file_data_,
                    )
                }
            };

            let environment_configuration_file = match from_str::<EnvironmentConfigurationFile<String_>>(environment_file_data.as_str()) {
                Ok(environment_configuration_file_) => environment_configuration_file_,
                Err(error) => {
                    return Err(
                        Error::OtherError {
                            other_error: OtherError::new(error),
                        },
                    );
                }
            };

            let environment_configuration = EnvironmentConfiguration {
                environment,
                environment_configuration_file,
            };

            return Ok(environment_configuration);
        }
    }
}
