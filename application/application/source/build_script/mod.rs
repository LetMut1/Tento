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
        panic!(
            "{}",
            error
        );
    }

    return ();
}

struct Processor;

impl Processor {
    fn process() -> Result<(), Box<dyn Error + 'static>> {
        Self::create_rerun_instruction()?;

        Self::create_environment_configuration_constant()?;

        return Ok(());
    }

    // It is necessary that the build-script be run on each compilation command,
    // so we specify in the instructions that the Cargo watch for a non-existent
    // file with `cargo:rerun-if-changed=non_existent_file` command.
    fn create_rerun_instruction() -> Result<(), Box<dyn Error + 'static>> {
        let file_name = Uuid::new_v4().to_string();

        let file_path = format!(
            "{}/{}.txt",
            var("OUT_DIR")?.as_str(),
            file_name.as_str(),
        );

        println!(
            "cargo:rerun-if-changed={}",
            file_path.as_str(),
        );

        return Ok(());
    }

    fn create_environment_configuration_constant() -> Result<(), Box<dyn Error + 'static>> {
        let environment_configuration_file_path = format!(
            "{}/../environment_configuration",
            var("CARGO_MANIFEST_DIR")?.as_str(),
        );

        let environment_configuration = Loader::load_from_file(environment_configuration_file_path.as_str())?;

        let environment = match environment_configuration.environment {
            Environment::Production => "Environment::Production",
            Environment::Development => "Environment::Development",
            Environment::LocalDevelopment => "Environment::LocalDevelopment"
        };

        let keepalive_duration = match environment_configuration.application_server.tcp.keepalive.duration {
            Some(keepalive_duration_) => {
                format!(
                    "Some({})",
                    keepalive_duration_,
                )
            }
            None => "None".to_string()
        };

        let keepalive_interval_duration = match environment_configuration.application_server.tcp.keepalive.interval_duration {
            Some(keepalive_interval_duration_) => {
                format!(
                    "Some({})",
                    keepalive_interval_duration_,
                )
            }
            None => "None".to_string()
        };

        let keepalive_retries_quantity = match environment_configuration.application_server.tcp.keepalive.retries_quantity {
            Some(keepalive_retries_quantity_) => {
                format!(
                    "Some({})",
                    keepalive_retries_quantity_,
                )
            }
            None => "None".to_string()
        };

        let http_maximum_pending_accept_reset_streams = match environment_configuration.application_server.http.maximum_pending_accept_reset_streams {
            Some(http_maximum_pending_accept_reset_streams_) => {
                format!(
                    "Some({})",
                    http_maximum_pending_accept_reset_streams_,
                )
            }
            None => "None".to_string()
        };

        let keepalive = match environment_configuration.application_server.http.keepalive {
            Some(ref keepalive_) => {
                format!(
                    "\
                        Some( \n\t\t\t\t\
                            HttpKeepalive {{ \n\t\t\t\t\t\
                                interval_duration: {}, \n\t\t\t\t\t\
                                timeout_duration: {}, \n\t\t\t\t\
                            }} \n\t\t\t\
                        )\
                    ",
                    keepalive_.interval_duration,
                    keepalive_.timeout_duration,
                )
            }
            None => "None".to_string()
        };

        let tls = match environment_configuration.application_server.http.tls {
            Some(ref tls_) => {
                format!(
                    "\
                        Some( \n\t\t\t\t\
                            Tls {{ \n\t\t\t\t\t\
                                certificate_crt_path: StringLiteral(\"{}\"), \n\t\t\t\t\t\
                                certificate_key_path: StringLiteral(\"{}\"), \n\t\t\t\t\
                            }} \n\t\t\t\
                        )\
                    ",
                    tls_.certificate_crt_path.0.as_str(),
                    tls_.certificate_key_path.0.as_str(),
                )
            }
            None => "None".to_string()
        };

        let build_file_content = format!(
            "\
                pub use extern_crate::environment_configuration::environment_configuration::ApplicationServer; \n\
                pub use extern_crate::environment_configuration::environment_configuration::EmailServer; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Encryption; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Environment; \n\
                pub use extern_crate::environment_configuration::environment_configuration::EnvironmentConfiguration; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Http; \n\
                pub use extern_crate::environment_configuration::environment_configuration::HttpKeepalive; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Postgresql; \n\
                pub use extern_crate::environment_configuration::environment_configuration::PrivateKey; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Redis; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Resource; \n\
                pub use extern_crate::environment_configuration::environment_configuration::StringLiteral; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Tcp; \n\
                pub use extern_crate::environment_configuration::environment_configuration::TcpKeepalive; \n\
                pub use extern_crate::environment_configuration::environment_configuration::Tls; \n\
                \n\
                pub const {}: EnvironmentConfiguration<StringLiteral> = EnvironmentConfiguration {{ \n\t\
                    environment: {}, \n\t\
                    application_server: ApplicationServer {{ \n\t\t\
                        tcp: Tcp {{ \n\t\t\t\
                            socket_address: StringLiteral(\"{}\"), \n\t\t\t\
                            nodelay: {}, \n\t\t\t\
                            sleep_on_accept_errors: {}, \n\t\t\t\
                            keepalive: TcpKeepalive {{ \n\t\t\t\t\
                                duration: {}, \n\t\t\t\t\
                                interval_duration: {}, \n\t\t\t\t\
                                retries_quantity: {}, \n\t\t\t\
                            }}, \n\t\t\
                        }}, \n\t\t\
                        http: Http {{ \n\t\t\t\
                            adaptive_window: {}, \n\t\t\t\
                            connection_window_size: {}, \n\t\t\t\
                            stream_window_size: {}, \n\t\t\t\
                            maximum_frame_size: {}, \n\t\t\t\
                            maximum_sending_buffer_size: {}, \n\t\t\t\
                            enable_connect_protocol: {}, \n\t\t\t\
                            maximum_header_list_size: {}, \n\t\t\t\
                            maximum_pending_accept_reset_streams: {}, \n\t\t\t\
                            keepalive: {}, \n\t\t\t\
                            tls: {}, \n\t\t\
                        }}, \n\t\
                    }}, \n\t\
                    resource: Resource {{ \n\t\t\
                        postgresql: Postgresql {{ \n\t\t\t\
                            database_1_url: StringLiteral(\"{}\"), \n\t\t\t\
                            database_2_url: StringLiteral(\"{}\"), \n\t\t\
                        }}, \n\t\t\
                        redis: Redis {{ \n\t\t\t\
                            database_1_url: StringLiteral(\"{}\"), \n\t\t\
                        }}, \n\t\t\
                        email_server: EmailServer {{ \n\t\t\t\
                            socket_address: StringLiteral(\"{}\"), \n\t\t\
                        }}, \n\t\
                    }}, \n\t\
                    encryption: Encryption {{ \n\t\t\
                        private_key: PrivateKey {{ \n\t\t\t\
                            application_user_access_token: StringLiteral(\"{}\"), \n\t\t\t\
                            application_user_access_refresh_token: StringLiteral(\"{}\"), \n\t\t\
                        }}, \n\t\
                    }}, \n\
                }}; \
            ",
            ENVIRONMENT_CONFIGURATION_CONSTANT_NAME,
            environment,
            environment_configuration.application_server.tcp.socket_address.0.as_str(),
            environment_configuration.application_server.tcp.nodelay,
            environment_configuration.application_server.tcp.sleep_on_accept_errors,
            keepalive_duration.as_str(),
            keepalive_interval_duration.as_str(),
            keepalive_retries_quantity.as_str(),
            environment_configuration.application_server.http.adaptive_window,
            environment_configuration.application_server.http.connection_window_size,
            environment_configuration.application_server.http.stream_window_size,
            environment_configuration.application_server.http.maximum_frame_size,
            environment_configuration.application_server.http.maximum_sending_buffer_size,
            environment_configuration.application_server.http.enable_connect_protocol,
            environment_configuration.application_server.http.maximum_header_list_size,
            http_maximum_pending_accept_reset_streams.as_str(),
            keepalive.as_str(),
            tls.as_str(),
            environment_configuration.resource.postgresql.database_1_url.0.as_str(),
            environment_configuration.resource.postgresql.database_2_url.0.as_str(),
            environment_configuration.resource.redis.database_1_url.0.as_str(),
            environment_configuration.resource.email_server.socket_address.0.as_str(),
            environment_configuration.encryption.private_key.application_user_access_token.0.as_str(),
            environment_configuration.encryption.private_key.application_user_access_refresh_token.0.as_str(),
        );

        let build_file = format!(
            "{}/{}",
            var("OUT_DIR")?.as_str(),
            environment_configuration_constant_file_name!(),
        );

        let mut file = File::create(Path::new(build_file.as_str()))?;

        file.write_all(build_file_content.as_bytes())?;

        return Ok(());
    }
}
