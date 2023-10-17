use super::Sender;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::EmailServer;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use lettre::smtp::SmtpClient;
use lettre::ClientSecurity;
use lettre::Transport;
use lettre_email::EmailBuilder;
use std::convert::Into;
use std::net::ToSocketAddrs;

pub use crate::infrastructure_layer::data::control_type::Email;

impl Sender<Email> {
    // TODO Возможно, сразу можно положить объект в константу.  // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(
        subject: &'a str,
        body: String,
        to: &'a str,
    ) -> Result<(), ErrorAuditor_> {
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
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::EmailServer {
                                    email_server: EmailServer::Email {
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

        let mut email_server_socket_address_registry = match ENVIRONMENT_CONFIGURATION.resource.email_server.socket_address.0.to_socket_addrs() {
            Ok(email_server_socket_address_registry_) => email_server_socket_address_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
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
                    ErrorAuditor_::new(
                        Error::Logic {
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
                            ErrorAuditor_::new(
                                Error::Runtime {
                                    runtime: Runtime::Resource {
                                        resource: ResourceError::EmailServer {
                                            email_server: EmailServer::Smtp {
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
                ErrorAuditor_::new(
                    Error::Runtime {
                        runtime: Runtime::Resource {
                            resource: ResourceError::EmailServer {
                                email_server: EmailServer::Smtp {
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
