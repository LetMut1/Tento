use super::loader::Loader;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentFileConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::toml::from_str as toml_from_str;
use std::fs::read_to_string;
use std::path::Path;

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
                        BaseError::LogicError {
                            message: "The directory does not exist.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let production_environment_file_path_buffer = file_path.join(Path::new(Self::PRODUCTION_ENVIRONMENT_FILE_NAME));

        let (environment, environment_file_data) = if production_environment_file_path_buffer.exists() {
            let environment_file_data_ = match read_to_string(production_environment_file_path_buffer.as_path()) {
                Ok(environment_file_data__) => environment_file_data__,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError {
                                runtime_error: RuntimeError::OtherError {
                                    other_error: OtherError::new(error),
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }
            };

            (
                Environment::Production,
                environment_file_data_,
            )
        } else {
            let local_development_environment_file_path_buffer =
                file_path.join(Path::new(Self::LOCAL_DEVELOPMENT_ENVIRONMENT_FILE_NAME));

            if local_development_environment_file_path_buffer.exists() {
                let environment_file_data_ =
                    match read_to_string(local_development_environment_file_path_buffer.as_path()) {
                        Ok(environment_file_data__) => environment_file_data__,
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RuntimeError {
                                        runtime_error: RuntimeError::OtherError {
                                            other_error: OtherError::new(error),
                                        },
                                    },
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                        None,
                                    ),
                                ),
                            );
                        }
                    };

                (
                    Environment::LocalDevelopment,
                    environment_file_data_,
                )
            } else {
                let development_environment_file_path_buffer =
                    file_path.join(Path::new(Self::DEVELOPMENT_ENVIRONMENT_FILE_NAME));

                let environment_file_data_ = if development_environment_file_path_buffer.exists() {
                    let environment_file_data__ =
                        match read_to_string(development_environment_file_path_buffer.as_path()) {
                            Ok(environment_file_data___) => environment_file_data___,
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RuntimeError {
                                            runtime_error: RuntimeError::OtherError {
                                                other_error: OtherError::new(error),
                                            },
                                        },
                                        BacktracePart::new(
                                            line!(),
                                            file!(),
                                            None,
                                        ),
                                    ),
                                );
                            }
                        };

                    environment_file_data__
                } else {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::LogicError {
                                message: "Any ....env files does not exist.",
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                };

                (
                    Environment::Development,
                    environment_file_data_,
                )
            }
        };

        let environment_file_configuration =
            match toml_from_str::<EnvironmentFileConfiguration>(environment_file_data.as_str()) {
                Ok(environment_file_configuration_) => environment_file_configuration_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError {
                                runtime_error: RuntimeError::OtherError {
                                    other_error: OtherError::new(error),
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }
            };

        let environment_configuration = EnvironmentConfiguration {
            environment,
            environment_file_configuration,
        };

        return Ok(environment_configuration);
    }
}
