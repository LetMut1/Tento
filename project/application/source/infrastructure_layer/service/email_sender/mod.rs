use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use lettre_email::EmailBuilder;
use lettre::ClientSecurity;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::authentication::Mechanism;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::extension::ClientId;
use lettre::smtp::SmtpClient;
use lettre::Transport;
use std::convert::Into;

pub struct EmailSender;

impl EmailSender {   // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(
        subject: &'a str,
        body: String,
        to: &'a str
    ) -> Result<(), ErrorAuditor> {
        match EmailBuilder::new()      //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build() {
            Ok(email) => {
                let smtp_client: SmtpClient;
                if EnvironmentVariableResolver::is_production()? {
                    match SmtpClient::new_simple("TODO") {   // TODO
                        Ok(smtp_client_) => {
                            smtp_client = smtp_client_.hello_name(ClientId::Domain("TODO".to_string())) // TODO
                                .credentials(Credentials::new("usToDO".to_string(), "pasTODO".to_string())) // TODO
                                .smtp_utf8(true)
                                .authentication_mechanism(Mechanism::Plain)// TODO
                                .connection_reuse(ConnectionReuseParameters::NoReuse);// TODO 
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::RunTimeError {
                                        run_time_error: RunTimeError::ResourceError {
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
                    }
                } else {
                    match SmtpClient::new(
                        EnvironmentVariableResolver::get_resource_email_server_socket_address()?.as_str(), ClientSecurity::None
                    ) {
                        Ok(smtp_client_) => {
                            smtp_client = smtp_client_;
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::RunTimeError {
                                        run_time_error: RunTimeError::ResourceError {
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
                    }
                }
                if let Err(error) = smtp_client.transport().send(email.into()) {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::RunTimeError {
                                run_time_error: RunTimeError::ResourceError {
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
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {
                            run_time_error: RunTimeError::ResourceError {
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
        }
    }
}