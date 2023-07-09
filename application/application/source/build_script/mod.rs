#![allow(clippy::unused_unit)]

use extern_crate::build_const::ConstWriter;
use extern_crate::build_script_constant::environment_configuration::ENVIRONMENT_CONFIGURATION_CONSTANT_NAME;
use extern_crate::build_script_constant::ENVIRONMENT_CONFIGURATION_CONSTANT_MODULE_NAME;
use extern_crate::environment_configuration::environment_configuration::Application;
use extern_crate::environment_configuration::environment_configuration::EmailServer;
use extern_crate::environment_configuration::environment_configuration::Encryption;
use extern_crate::environment_configuration::environment_configuration::EnvironmentConfiguration;
use extern_crate::environment_configuration::environment_configuration::EnvironmentConfigurationFile;
use extern_crate::environment_configuration::environment_configuration::Postgresql;
use extern_crate::environment_configuration::environment_configuration::PrivateKey;
use extern_crate::environment_configuration::environment_configuration::Redis;
use extern_crate::environment_configuration::environment_configuration::Resource;
use extern_crate::environment_configuration::environment_configuration::SimpleType;
use extern_crate::environment_configuration::environment_configuration::Tcp;
use extern_crate::environment_configuration::environment_configuration::Str;
use extern_crate::environment_configuration::loader::Loader;
use std::env::var;
use std::error::Error;
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
    const CARGO_MANIFEST_DIR: &'static str = "CARGO_MANIFEST_DIR";
    const CARGO_OUT_DIR: &'static str = "OUT_DIR";
    const ENVIRONMENT_CONFIGURATION_DIRECTORY_NAME: &'static str = "environment_configuration";

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
        let mut environment_configuration_file_path = var(Self::CARGO_MANIFEST_DIR)?;

        environment_configuration_file_path = format!(
            "{}/../{}",
            environment_configuration_file_path.as_str(),
            Self::ENVIRONMENT_CONFIGURATION_DIRECTORY_NAME
        );

        let environment_configuration = Loader::load_from_file(environment_configuration_file_path.as_str())?;

        let environment_configuration_: EnvironmentConfiguration<Str<'_>> = EnvironmentConfiguration {
            environment: environment_configuration.environment,
            environment_configuration_file: EnvironmentConfigurationFile {
                application: Application {
                    tcp: Tcp {
                        socket_address: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.application.tcp.socket_address.value.get(),
                            ),
                        },
                        nodelay: environment_configuration.environment_configuration_file.application.tcp.nodelay,
                        sleep_on_accept_errors: environment_configuration.environment_configuration_file.application.tcp.sleep_on_accept_errors,
                        keepalive_seconds: environment_configuration.environment_configuration_file.application.tcp.keepalive_seconds,
                    },
                    http: environment_configuration.environment_configuration_file.application.http,
                },
                resource: Resource {
                    postgresql: Postgresql {
                        database_1_url: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.resource.postgresql.database_1_url.value.get(),
                            ),
                        },
                        database_2_url: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.resource.postgresql.database_2_url.value.get(),
                            ),
                        },
                    },
                    redis: Redis {
                        database_1_url: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.resource.redis.database_1_url.value.get(),
                            ),
                        },
                    },
                    email_server: EmailServer {
                        socket_address: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.resource.email_server.socket_address.value.get(),
                            ),
                        },
                    },
                },
                encryption: Encryption {
                    private_key: PrivateKey {
                        application_user_access_token: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.encryption.private_key.application_user_access_token.value.get(),
                            ),
                        },
                        application_user_access_refresh_token: SimpleType {
                            value: Str::new(
                                environment_configuration.environment_configuration_file.encryption.private_key.application_user_access_refresh_token.value.get(),
                            ),
                        },
                    },
                },
            },
        };

        let mut constant_writer = ConstWriter::for_build(ENVIRONMENT_CONFIGURATION_CONSTANT_MODULE_NAME!())?;

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Application");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::EmailServer");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Encryption");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Environment");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::EnvironmentConfiguration");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::EnvironmentConfigurationFile");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Http");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::KeepAlive");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Postgresql");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::PrivateKey");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Redis");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Resource");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::SimpleType");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::SimpleTypeActive");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Tcp");

        constant_writer.add_dependency("extern_crate::environment_configuration::environment_configuration::Str");

        let mut constant_value_writer = constant_writer.finish_dependencies();

        constant_value_writer.add_value(
            ENVIRONMENT_CONFIGURATION_CONSTANT_NAME,
            "EnvironmentConfiguration<Str<'static>>",
            environment_configuration_,
        );

        constant_value_writer.finish();

        return Ok(());
    }
}
