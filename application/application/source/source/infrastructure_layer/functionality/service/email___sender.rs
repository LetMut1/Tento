use super::sender::Sender;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::EmailServerError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::lettre::smtp::SmtpClient;
use extern_crate::lettre::ClientSecurity;
use extern_crate::lettre::Transport;
use extern_crate::lettre_email::EmailBuilder;
use std::convert::Into;
use std::net::ToSocketAddrs;

pub use crate::infrastructure_layer::data::control_type::Email;

impl Sender<Email> {
    // TODO Возможно, сразу можно положить объект в константу.  // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(
        subject: &'a str,
        body: String,
        to: &'a str,
    ) -> Result<(), ErrorAuditor> {
        let email = match EmailBuilder::new() //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build()
        {
            Ok(email_) => email_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::EmailServerError {
                                    email_server_error: EmailServerError::EmailError {
                                        email_error: error,
                                    },
                                },
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

        let mut email_server_socket_address_registry = match ENVIRONMENT_CONFIGURATION.environment_configuration_file.resource.email_server.socket_address.value.0.to_socket_addrs() {
            Ok(email_server_socket_address_registry_) => email_server_socket_address_registry_,
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

        let email_server_socket_address = match email_server_socket_address_registry.next() {
            Some(email_server_socket_address_) => email_server_socket_address_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Invalid socket address.",
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

        let smtp_client = match ENVIRONMENT_CONFIGURATION.environment {
            Environment::Production => {
                todo!();
            }
            Environment::Development | Environment::LocalDevelopment => {
                let smtp_client_ = match SmtpClient::new(
                    &email_server_socket_address,
                    ClientSecurity::None,
                ) {
                    Ok(smtp_client__) => smtp_client__,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError {
                                    runtime_error: RuntimeError::ResourceError {
                                        resource_error: ResourceError::EmailServerError {
                                            email_server_error: EmailServerError::SmtpError {
                                                smtp_error: error,
                                            },
                                        },
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

                smtp_client_
            }
        };

        if let Err(error) = smtp_client.transport().send(email.into()) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::ResourceError {
                            resource_error: ResourceError::EmailServerError {
                                email_server_error: EmailServerError::SmtpError {
                                    smtp_error: error,
                                },
                            },
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

        return Ok(());
    }
}
