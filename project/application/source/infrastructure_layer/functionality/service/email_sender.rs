use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::EmailServerError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::lettre_email::EmailBuilder;
use extern_crate::lettre::ClientSecurity;
// use extern_crate::lettre::smtp::authentication::Credentials;
// use extern_crate::lettre::smtp::authentication::Mechanism;
// use extern_crate::lettre::smtp::ConnectionReuseParameters;
// use extern_crate::lettre::smtp::extension::ClientId;
use extern_crate::lettre::smtp::SmtpClient;
use extern_crate::lettre::Transport;
use std::convert::Into;

pub struct EmailSender;

impl EmailSender {   // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        subject: &'a str,
        body: String,
        to: &'a str
    ) -> Result<(), ErrorAuditor> {
        let email = match EmailBuilder::new()      //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build() {
            Ok(email_) => email_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::EmailServerError {
                                    email_server_error: EmailServerError::EmailError {
                                        email_error: error
                                    }
                                }
                            }
                        },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let smtp_client = match *environment_configuration.get_environment() {
            Environment::Production => {
                todo!();
                // let smtp_client_= match SmtpClient::new_simple("TODO") {                         // TODO НАСТРОИТЬ В Препроде!!!!!!!!!!!!!!!!!!!!!
                //     Ok(smtp_client__) => smtp_client__,
                //     Err(error) => {
                //         return Err(
                //             ErrorAuditor::new(
                //                 BaseError::RuntimeError {
                //                     runtime_error: RuntimeError::ResourceError {
                //                         resource_error: ResourceError::EmailServerError {
                //                             email_server_error: EmailServerError::SmtpError {
                //                                 smtp_error: error
                //                             }
                //                         }
                //                     }
                //                 },
                //                 BacktracePart::new(line!(), file!(), None)
                //             )
                //         );
                //     }
                // };

                // smtp_client_.hello_name(ClientId::Domain("TODO".to_string())) // TODO
                //     .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO
                //     .smtp_utf8(true)
                //     .authentication_mechanism(Mechanism::Plain)// TODO
                //     .connection_reuse(ConnectionReuseParameters::NoReuse)// TODO
            }
            Environment::Development |
            Environment::LocalDevelopment => {
                let smtp_client_ = match SmtpClient::new(
                    *environment_configuration.get_email_server_socket_address(), ClientSecurity::None
                ) {
                    Ok(smtp_client__) => smtp_client__,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError {
                                    runtime_error: RuntimeError::ResourceError {
                                        resource_error: ResourceError::EmailServerError {
                                            email_server_error: EmailServerError::SmtpError {
                                                smtp_error: error
                                            }
                                        }
                                    }
                                },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };

                smtp_client_
            }
        };

        if let Err(error) = smtp_client
            .transport()
            .send(email.into()) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError {
                        runtime_error: RuntimeError::ResourceError {
                            resource_error: ResourceError::EmailServerError {
                                email_server_error: EmailServerError::SmtpError {
                                    smtp_error: error
                                }
                            }
                        }
                    },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }
}