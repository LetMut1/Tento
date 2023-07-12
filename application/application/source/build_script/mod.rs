#![allow(clippy::unused_unit)]

use extern_crate::build_script_constant::environment_configuration::ENVIRONMENT_CONFIGURATION_CONSTANT_NAME;
use extern_crate::build_script_constant::environment_configuration_constant_file_name;
use extern_crate::environment_configuration::environment_configuration::Environment;
use extern_crate::environment_configuration::loader::Loader;
use std::env::var;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use extern_crate::uuid::Uuid;

fn main() -> () {
    if let Err(error) = Processor::process() {
        println!(
            "{}",
            error
        );
    }

    return ();
}

struct Processor;

impl Processor {
    const CARGO_OUT_DIR: &'static str = "OUT_DIR";
    const ENVIRONMENT_CONFIGURATION_DIRECTORY_PATH: &'static str = "environment_configuration";

    fn process() -> Result<(), Box<dyn Error + 'static>> {
        Self::create_rerun_instruction()?;

        Self::create_environment_configuration_constant()?;

        return Ok(());
    }

    // It is necessary that the build-script be run on each compilation command,
    // so we specify in the instructions that the Cargo watch for a non-existent
    // file with `cargo:rerun-if-changed=non_existent_file` command.
    fn create_rerun_instruction() -> Result<(), Box<dyn Error + 'static>> {
        let mut file_path = var(Self::CARGO_OUT_DIR)?;

        let file_name = Uuid::new_v4().to_string();

        file_path = format!(
            "{}/{}.txt",
            file_path.as_str(),
            file_name.as_str()
        );

        println!(
            "cargo:rerun-if-changed={}",
            file_path.as_str()
        );

        return Ok(());
    }

    fn create_environment_configuration_constant() -> Result<(), Box<dyn Error + 'static>> {
        let mut environment_configuration_file_path = var("CARGO_MANIFEST_DIR")?;

        environment_configuration_file_path = format!(
            "{}/../{}",
            environment_configuration_file_path.as_str(),
            Self::ENVIRONMENT_CONFIGURATION_DIRECTORY_PATH
        );

        let environment_configuration = Loader::load_from_file(environment_configuration_file_path.as_str())?;

        let environment = match environment_configuration.environment {
            Environment::Production => "Environment::Production",
            Environment::Development => "Environment::Development",
            Environment::LocalDevelopment => "Environment::LocalDevelopment"
        };

        let build_file_content = format!(
            "\
                pub use extern_crate::environment_configuration::environment_configuration::Application; \n\
                pub use extern_crate::environment_configuration::environment_configuration::EmailServer; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Encryption; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Environment; \n\
                pub use extern_crate::environment_configuration::environment_configuration::EnvironmentConfiguration; \n\
                pub use extern_crate::environment_configuration::environment_configuration::EnvironmentConfigurationFile; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Http; \n\
                pub use extern_crate::environment_configuration::environment_configuration::KeepAlive; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Postgresql; \n\
                pub use extern_crate::environment_configuration::environment_configuration::PrivateKey; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Redis; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Resource; \n\
                pub use extern_crate::environment_configuration::environment_configuration::SimpleType; \n\
                pub use extern_crate::environment_configuration::environment_configuration::SimpleTypeActive; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Str; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Tcp; \n\
                \n\
                pub const {}: EnvironmentConfiguration<Str> = EnvironmentConfiguration {{ \n\t\
                    environment: {}, \n\t\
                    environment_configuration_file: EnvironmentConfigurationFile {{ \n\t\t\
                        application: Application {{ \n\t\t\t\
                            tcp: Tcp {{ \n\t\t\t\t\
                                socket_address: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                nodelay: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                sleep_on_accept_errors: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                keepalive_seconds: SimpleTypeActive {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\t\
                                    is_active: {} \n\t\t\t\t\
                                }}, \n\t\t\t\
                            }}, \n\t\t\t\
                            http: Http {{ \n\t\t\t\t\
                                adaptive_window: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                connection_window_size: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                stream_window_size: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                maximum_frame_size: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                maximum_sending_buffer_size: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                http2_only: SimpleType {{ \n\t\t\t\t\t\
                                    value: {}, \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                keep_alive: KeepAlive {{ \n\t\t\t\t\t\
                                    is_active: {}, \n\t\t\t\t\t\
                                    interval_seconds: SimpleType {{ \n\t\t\t\t\t\t\
                                        value: {}, \n\t\t\t\t\t\
                                    }}, \n\t\t\t\t\t\
                                    timeout_seconds: SimpleType {{ \n\t\t\t\t\t\t\
                                        value: {}, \n\t\t\t\t\t\
                                    }}, \n\t\t\t\t\
                                }}, \n\t\t\t\
                            }}, \n\t\t\
                        }}, \n\t\t\
                        resource: Resource {{ \n\t\t\t\
                            postgresql: Postgresql {{ \n\t\t\t\t\
                                database_1_url: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                database_2_url: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\
                            }}, \n\t\t\t\
                            redis: Redis {{ \n\t\t\t\t\
                                database_1_url: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\
                            }}, \n\t\t\t\
                            email_server: EmailServer {{ \n\t\t\t\t\
                                socket_address: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\
                            }}, \n\t\t\
                        }}, \n\t\t\
                        encryption: Encryption {{ \n\t\t\t\
                            private_key: PrivateKey {{ \n\t\t\t\t\
                                application_user_access_token: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\t\
                                application_user_access_refresh_token: SimpleType {{ \n\t\t\t\t\t\
                                    value: Str(\"{}\"), \n\t\t\t\t\
                                }}, \n\t\t\t\
                            }}, \n\t\t\
                        }}, \n\t\
                    }}, \n\
                }}; \n
            ",
            ENVIRONMENT_CONFIGURATION_CONSTANT_NAME,
            environment,
            environment_configuration.environment_configuration_file.application.tcp.socket_address.value.0.as_str(),
            environment_configuration.environment_configuration_file.application.tcp.nodelay.value,
            environment_configuration.environment_configuration_file.application.tcp.sleep_on_accept_errors.value,
            environment_configuration.environment_configuration_file.application.tcp.keepalive_seconds.value,
            environment_configuration.environment_configuration_file.application.tcp.keepalive_seconds.is_active,
            environment_configuration.environment_configuration_file.application.http.adaptive_window.value,
            environment_configuration.environment_configuration_file.application.http.connection_window_size.value,
            environment_configuration.environment_configuration_file.application.http.stream_window_size.value,
            environment_configuration.environment_configuration_file.application.http.maximum_frame_size.value,
            environment_configuration.environment_configuration_file.application.http.maximum_sending_buffer_size.value,
            environment_configuration.environment_configuration_file.application.http.http2_only.value,
            environment_configuration.environment_configuration_file.application.http.keep_alive.is_active,
            environment_configuration.environment_configuration_file.application.http.keep_alive.interval_seconds.value,
            environment_configuration.environment_configuration_file.application.http.keep_alive.timeout_seconds.value,
            environment_configuration.environment_configuration_file.resource.postgresql.database_1_url.value.0.as_str(),
            environment_configuration.environment_configuration_file.resource.postgresql.database_2_url.value.0.as_str(),
            environment_configuration.environment_configuration_file.resource.redis.database_1_url.value.0.as_str(),
            environment_configuration.environment_configuration_file.resource.email_server.socket_address.value.0.as_str(),
            environment_configuration.environment_configuration_file.encryption.private_key.application_user_access_token.value.0.as_str(),
            environment_configuration.environment_configuration_file.encryption.private_key.application_user_access_refresh_token.value.0.as_str()
        );

        let mut build_file = var(Self::CARGO_OUT_DIR)?;

        build_file = format!(
            "{}/{}",
            build_file.as_str(),
            environment_configuration_constant_file_name!()
        );

        let mut file = File::create(Path::new(build_file.as_str()))?;

        file.write_all(build_file_content.as_bytes())?;

        return Ok(());
    }
}
