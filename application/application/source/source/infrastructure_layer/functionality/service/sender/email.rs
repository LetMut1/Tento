use super::Sender;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
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
        environment_configuration: &'a EnvironmentConfiguration,
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
            .convert(Backtrace::new(line!(), file!()))?;

        // TODO В static OnceLock
        let mut email_server_socket_address_registry = environment_configuration.resource.email_server.socket_address.to_socket_addrs().convert(Backtrace::new(line!(), file!()))?;

        let email_server_socket_address = match email_server_socket_address_registry.next() {
            Some(email_server_socket_address_) => email_server_socket_address_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Invalid socket address.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let smtp_client = SmtpClient::new(
            &email_server_socket_address,
            ClientSecurity::None,
        )
        .convert(Backtrace::new(line!(), file!()))?;

        smtp_client.transport().send(email.into()).convert(Backtrace::new(line!(), file!()))?;

        return Ok(());
    }
}
