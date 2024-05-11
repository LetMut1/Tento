use super::Sender;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
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
    ) -> Result<(), Auditor<Error>> {
        let email = EmailBuilder::new() //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build()
            .convert(BacktracePart::new(line!(), file!()))?;

        // TODO В static OnceLock
        let mut email_server_socket_address_registry = ENVIRONMENT_CONFIGURATION.resource.email_server.socket_address.0.to_socket_addrs().convert(BacktracePart::new(line!(), file!()))?;

        let email_server_socket_address = match email_server_socket_address_registry.next() {
            Some(email_server_socket_address_) => email_server_socket_address_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Invalid socket address.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
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
                SmtpClient::new(
                    &email_server_socket_address,
                    ClientSecurity::None,
                )
                .convert(BacktracePart::new(line!(), file!()))?
            }
        };

        smtp_client.transport().send(email.into()).convert(BacktracePart::new(line!(), file!()))?;

        return Ok(());
    }
}
