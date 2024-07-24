use super::Sender;
use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    },
    control_type::Email,
    environment_configuration::EnvironmentConfiguration,
};
use lettre::{
    smtp::SmtpClient,
    ClientSecurity,
    Transport,
};
use lettre_email::EmailBuilder;
use std::{
    convert::Into,
    net::ToSocketAddrs,
};
impl Sender<Email> {
    //TODO  ASYNC client // TODO Возможно, сразу можно положить объект в константу.  // TODO В предпродакшене, когда будет smtp-ссервер, настройить все через константы и енв
    pub fn send<'a>(environment_configuration: &'a EnvironmentConfiguration, subject: &'a str, body: String, to: &'a str) -> Result<(), AggregateError> {
        let email = EmailBuilder::new() //TODO
            .subject(subject)
            .text(body)
            .from("from_changethis@yandex.ru".to_string())
            .to(to)
            .build()
            .into_invalid_argument_from_client_code(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        // TODO В static OnceLock
        let mut email_server_socket_address_registry = environment_configuration.resource.email_server.socket_address.to_socket_addrs().into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let email_server_socket_address = match email_server_socket_address_registry.next() {
            Some(email_server_socket_address_) => email_server_socket_address_,
            None => {
                return Err(
                    AggregateError::new_logic(
                        "Invalid socket address.",
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
        .into_invalid_argument_from_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        smtp_client.transport().send(email.into()).into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(());
    }
}
